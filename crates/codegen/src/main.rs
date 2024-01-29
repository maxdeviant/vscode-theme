use anyhow::Result;
use indexmap::IndexMap;
use oxc::allocator::Allocator;
use oxc::ast::ast;
use oxc::parser::Parser;
use oxc::span::SourceType;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
struct ThemeColor {
    pub name: String,
    pub description: String,
}

fn main() -> Result<()> {
    let color_registry_src = include_str!("../../../colorRegistry.ts");
    let colors = parse_registry_colors(&color_registry_src)?;

    dbg!(colors);

    Ok(())
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
