use std::fs::File;
use std::io::Write;

use anyhow::Result;
use heck::{ToSnakeCase, ToUpperCamelCase};
use indexmap::IndexMap;
use oxc::allocator::Allocator;
use oxc::ast::ast;
use oxc::parser::Parser;
use oxc::span::SourceType;
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
struct ThemeColor {
    pub name: String,
    pub description: String,
}

fn main() -> Result<()> {
    // let color_registry_src = include_str!("../../../colorRegistry.ts");
    // let colors = parse_registry_colors(&color_registry_src)?;

    let theme_color_src = include_str!("../theme-color.md");
    let colors = parse_theme_colors(&theme_color_src)?;
    let colors = group_colors(colors);

    let mut color_fields = Vec::new();
    let mut structs = Vec::new();

    for (group, colors) in colors {
        if let Some(group) = group {
            let field_name = format_ident!("{}", group.to_snake_case());

            let (struct_name, struct_decl) = color_group_struct(group, colors);

            structs.push(struct_decl);

            color_fields.push(quote! {
                #[serde(flatten)]
                pub #field_name: #struct_name
            });
        } else {
            for color in colors {
                color_fields.push(color_field(None, color));
            }
        }
    }

    let module = quote! {
        use serde::{Deserialize, Serialize};

        use crate::serde::empty_string_as_none;

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct Colors {
            #(#color_fields),*
        }

        #(#structs)*
    };

    let file = syn::parse_str(&module.to_string())?;

    let mut output_file = File::create("crates/vscode_theme/src/generated/theme.rs")?;

    let contents = format!(
        "// This is a generated file.\n// Do not modify by hand!\n\n{}",
        prettyplease::unparse(&file)
    );

    output_file.write_all(contents.as_bytes())?;

    Ok(())
}

fn group_colors(colors: IndexMap<String, ThemeColor>) -> IndexMap<Option<String>, Vec<ThemeColor>> {
    let mut grouped_colors = IndexMap::new();

    for (color_name, color) in colors {
        let group = color_name
            .split_once('.')
            .map(|(group, _)| group.to_owned());

        let entry: &mut Vec<ThemeColor> = grouped_colors.entry(group).or_default();

        entry.push(color);
    }

    grouped_colors
}

fn parse_theme_colors(src: &str) -> Result<IndexMap<String, ThemeColor>> {
    let mut colors = IndexMap::new();

    let lines = src.lines();

    for line in lines {
        if line.starts_with("- `") && line.contains("`:") {
            if let Some((name, description)) = line.split_once("`:") {
                let name = name.trim_start_matches("- `").to_string();
                let description = description.trim().to_string();

                let color = ThemeColor { name, description };

                colors.insert(color.name.clone(), color);
            }
        }
    }

    Ok(colors)
}

fn parse_registry_colors(ts_src: &str) -> Result<IndexMap<String, ThemeColor>> {
    let mut colors = IndexMap::new();

    let allocator = Allocator::default();

    let parser = Parser::new(
        &allocator,
        ts_src,
        SourceType::default().with_typescript(true),
    )
    .parse();

    if !parser.errors.is_empty() {
        dbg!(&parser.errors);
    }

    for color in parser.program.body.into_iter().filter_map(parse_color_decl) {
        colors.insert(color.name.clone(), color);
    }

    Ok(colors)
}

fn parse_color_decl(stmt: ast::Statement) -> Option<ThemeColor> {
    let ast::Statement::ModuleDeclaration(module_decl) = stmt else {
        return None;
    };

    let ast::ModuleDeclaration::ExportNamedDeclaration(named_decl) = module_decl.unbox() else {
        return None;
    };

    let ast::Declaration::VariableDeclaration(variable_decl) = named_decl.unbox().declaration?
    else {
        return None;
    };

    if !variable_decl.kind.is_const() || variable_decl.declarations.is_empty() {
        return None;
    }

    let variable_decl = &variable_decl.declarations[0];

    let ast::Expression::CallExpression(call_expr) = variable_decl.init.as_ref()? else {
        return None;
    };

    let ast::Expression::Identifier(callee) = &call_expr.callee else {
        return None;
    };

    if callee.name != "registerColor" {
        return None;
    };

    let ast::Argument::Expression(ast::Expression::StringLiteral(color_name_lit)) =
        call_expr.arguments.get(0)?
    else {
        return None;
    };

    let ast::Argument::Expression(ast::Expression::CallExpression(call_expr)) =
        call_expr.arguments.get(2)?
    else {
        return None;
    };

    let ast::Expression::MemberExpression(member_expr) = &call_expr.callee else {
        return None;
    };

    let ast::MemberExpression::StaticMemberExpression(ref static_member_expr) = member_expr.0
    else {
        return None;
    };

    let ast::Expression::Identifier(object_ident) = &static_member_expr.object else {
        return None;
    };

    let is_nls_localize_call =
        object_ident.name == "nls" && static_member_expr.property.name == "localize";
    if !is_nls_localize_call {
        return None;
    }

    let ast::Argument::Expression(ast::Expression::StringLiteral(description_lit)) =
        call_expr.arguments.get(1)?
    else {
        return None;
    };

    let color_name = color_name_lit.value.to_string();
    let description = description_lit.value.to_string();

    Some(ThemeColor {
        name: color_name,
        description,
    })
}

fn color_field(group: Option<String>, color: ThemeColor) -> TokenStream {
    let json_name = color.name.clone();
    let name = match group {
        Some(group) => color.name.replace(&format!("{group}."), ""),
        None => color.name,
    };
    let name = format_ident!("{}", name.to_snake_case());
    let description = format!(" {}", color.description);

    quote! {
        #[doc = #description]
        #[serde(default, rename = #json_name, deserialize_with = "empty_string_as_none")]
        pub #name: Option<String>
    }
}

fn color_group_struct(group: String, colors: Vec<ThemeColor>) -> (Ident, TokenStream) {
    let name = format_ident!("{}Colors", group.to_upper_camel_case());

    let fields = colors
        .into_iter()
        .map(|color| color_field(Some(group.clone()), color))
        .collect::<Vec<_>>();

    let struct_decl = quote! {
        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct #name {
            #(#fields),*
        }
    };

    (name, struct_decl)
}
