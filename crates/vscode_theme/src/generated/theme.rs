// This is a generated file.
// Do not modify by hand!

use crate::serde::empty_string_as_none;
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Colors {
    /// Overall foreground color. This color is only used if not overridden by a component.
    #[serde(
        default,
        rename = "foreground",
        deserialize_with = "empty_string_as_none"
    )]
    pub foreground: Option<String>,
    /// Overall foreground for disabled elements. This color is only used if not overridden by a component.
    #[serde(
        default,
        rename = "disabledForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub disabled_foreground: Option<String>,
    /// Overall foreground color for error messages. This color is only used if not overridden by a component.
    #[serde(
        default,
        rename = "errorForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub error_foreground: Option<String>,
    /// Foreground color for description text providing additional information, for example for a label.
    #[serde(
        default,
        rename = "descriptionForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub description_foreground: Option<String>,
    /// Overall border color for focused elements. This color is only used if not overridden by a component.
    #[serde(
        default,
        rename = "focusBorder",
        deserialize_with = "empty_string_as_none"
    )]
    pub focus_border: Option<String>,
    /// An extra border around elements to separate them from others for greater contrast.
    #[serde(
        default,
        rename = "contrastBorder",
        deserialize_with = "empty_string_as_none"
    )]
    pub contrast_border: Option<String>,
    /// An extra border around active elements to separate them from others for greater contrast.
    #[serde(
        default,
        rename = "contrastActiveBorder",
        deserialize_with = "empty_string_as_none"
    )]
    pub contrast_active_border: Option<String>,
    #[serde(flatten)]
    pub icon: IconColors,
    #[serde(flatten)]
    pub selection: SelectionColors,
    #[serde(flatten)]
    pub text_separator: TextSeparatorColors,
    #[serde(flatten)]
    pub text_link: TextLinkColors,
    #[serde(flatten)]
    pub text_preformat: TextPreformatColors,
    #[serde(flatten)]
    pub text_block_quote: TextBlockQuoteColors,
    #[serde(flatten)]
    pub text_code_block: TextCodeBlockColors,
    #[serde(flatten)]
    pub widget: WidgetColors,
    #[serde(flatten)]
    pub input: InputColors,
    #[serde(flatten)]
    pub input_option: InputOptionColors,
    #[serde(flatten)]
    pub input_validation: InputValidationColors,
    #[serde(flatten)]
    pub dropdown: DropdownColors,
    #[serde(flatten)]
    pub button: ButtonColors,
    #[serde(flatten)]
    pub badge: BadgeColors,
    #[serde(flatten)]
    pub scrollbar: ScrollbarColors,
    #[serde(flatten)]
    pub scrollbar_slider: ScrollbarSliderColors,
    #[serde(flatten)]
    pub progress_bar: ProgressBarColors,
    #[serde(flatten)]
    pub editor_error: EditorErrorColors,
    #[serde(flatten)]
    pub editor_warning: EditorWarningColors,
    #[serde(flatten)]
    pub editor_info: EditorInfoColors,
    #[serde(flatten)]
    pub editor_hint: EditorHintColors,
    #[serde(flatten)]
    pub sash: SashColors,
    #[serde(flatten)]
    pub editor: EditorColors,
    #[serde(flatten)]
    pub editor_sticky_scroll: EditorStickyScrollColors,
    #[serde(flatten)]
    pub editor_sticky_scroll_hover: EditorStickyScrollHoverColors,
    #[serde(flatten)]
    pub editor_widget: EditorWidgetColors,
    #[serde(flatten)]
    pub quick_input: QuickInputColors,
    #[serde(flatten)]
    pub quick_input_title: QuickInputTitleColors,
    #[serde(flatten)]
    pub picker_group: PickerGroupColors,
    #[serde(flatten)]
    pub keybinding_label: KeybindingLabelColors,
    #[serde(flatten)]
    pub search_editor: SearchEditorColors,
    #[serde(flatten)]
    pub search: SearchColors,
    #[serde(flatten)]
    pub editor_hover_widget: EditorHoverWidgetColors,
    #[serde(flatten)]
    pub editor_link: EditorLinkColors,
    #[serde(flatten)]
    pub editor_inlay_hint: EditorInlayHintColors,
    #[serde(flatten)]
    pub editor_light_bulb: EditorLightBulbColors,
    #[serde(flatten)]
    pub editor_light_bulb_auto_fix: EditorLightBulbAutoFixColors,
    #[serde(flatten)]
    pub editor_light_bulb_ai: EditorLightBulbAiColors,
    #[serde(flatten)]
    pub diff_editor: DiffEditorColors,
    #[serde(flatten)]
    pub diff_editor_gutter: DiffEditorGutterColors,
    #[serde(flatten)]
    pub diff_editor_overview: DiffEditorOverviewColors,
    #[serde(flatten)]
    pub list: ListColors,
    #[serde(flatten)]
    pub list_filter_widget: ListFilterWidgetColors,
    #[serde(flatten)]
    pub tree: TreeColors,
    #[serde(flatten)]
    pub checkbox: CheckboxColors,
    #[serde(flatten)]
    pub quick_input_list: QuickInputListColors,
    #[serde(flatten)]
    pub menu: MenuColors,
    #[serde(flatten)]
    pub toolbar: ToolbarColors,
    #[serde(flatten)]
    pub breadcrumb: BreadcrumbColors,
    #[serde(flatten)]
    pub breadcrumb_picker: BreadcrumbPickerColors,
    #[serde(flatten)]
    pub merge: MergeColors,
    #[serde(flatten)]
    pub editor_overview_ruler: EditorOverviewRulerColors,
    #[serde(flatten)]
    pub minimap: MinimapColors,
    #[serde(flatten)]
    pub minimap_slider: MinimapSliderColors,
    #[serde(flatten)]
    pub problems_error_icon: ProblemsErrorIconColors,
    #[serde(flatten)]
    pub problems_warning_icon: ProblemsWarningIconColors,
    #[serde(flatten)]
    pub problems_info_icon: ProblemsInfoIconColors,
    #[serde(flatten)]
    pub charts: ChartsColors,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IconColors {
    /// The default color for icons in the workbench.
    #[serde(
        default,
        rename = "icon.foreground",
        deserialize_with = "empty_string_as_none"
    )]
    pub foreground: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelectionColors {
    /// The background color of text selections in the workbench (e.g. for input fields or text areas). Note that this does not apply to selections within the editor.
    #[serde(
        default,
        rename = "selection.background",
        deserialize_with = "empty_string_as_none"
    )]
    pub background: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextSeparatorColors {
    /// Color for text separators.
    #[serde(
        default,
        rename = "textSeparator.foreground",
        deserialize_with = "empty_string_as_none"
    )]
    pub foreground: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextLinkColors {
    /// Foreground color for links in text.
    #[serde(
        default,
        rename = "textLink.foreground",
        deserialize_with = "empty_string_as_none"
    )]
    pub foreground: Option<String>,
    /// Foreground color for links in text when clicked on and on mouse hover.
    #[serde(
        default,
        rename = "textLink.activeForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub active_foreground: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextPreformatColors {
    /// Foreground color for preformatted text segments.
    #[serde(
        default,
        rename = "textPreformat.foreground",
        deserialize_with = "empty_string_as_none"
    )]
    pub foreground: Option<String>,
    /// Background color for preformatted text segments.
    #[serde(
        default,
        rename = "textPreformat.background",
        deserialize_with = "empty_string_as_none"
    )]
    pub background: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextBlockQuoteColors {
    /// Background color for block quotes in text.
    #[serde(
        default,
        rename = "textBlockQuote.background",
        deserialize_with = "empty_string_as_none"
    )]
    pub background: Option<String>,
    /// Border color for block quotes in text.
    #[serde(
        default,
        rename = "textBlockQuote.border",
        deserialize_with = "empty_string_as_none"
    )]
    pub border: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextCodeBlockColors {
    /// Background color for code blocks in text.
    #[serde(
        default,
        rename = "textCodeBlock.background",
        deserialize_with = "empty_string_as_none"
    )]
    pub background: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WidgetColors {
    /// Shadow color of widgets such as find/replace inside the editor.
    #[serde(
        default,
        rename = "widget.shadow",
        deserialize_with = "empty_string_as_none"
    )]
    pub shadow: Option<String>,
    /// Border color of widgets such as find/replace inside the editor.
    #[serde(
        default,
        rename = "widget.border",
        deserialize_with = "empty_string_as_none"
    )]
    pub border: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputColors {
    /// Input box background.
    #[serde(
        default,
        rename = "input.background",
        deserialize_with = "empty_string_as_none"
    )]
    pub background: Option<String>,
    /// Input box foreground.
    #[serde(
        default,
        rename = "input.foreground",
        deserialize_with = "empty_string_as_none"
    )]
    pub foreground: Option<String>,
    /// Input box border.
    #[serde(
        default,
        rename = "input.border",
        deserialize_with = "empty_string_as_none"
    )]
    pub border: Option<String>,
    /// Input box foreground color for placeholder text.
    #[serde(
        default,
        rename = "input.placeholderForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub placeholder_foreground: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputOptionColors {
    /// Border color of activated options in input fields.
    #[serde(
        default,
        rename = "inputOption.activeBorder",
        deserialize_with = "empty_string_as_none"
    )]
    pub active_border: Option<String>,
    /// Background color of activated options in input fields.
    #[serde(
        default,
        rename = "inputOption.hoverBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub hover_background: Option<String>,
    /// Background hover color of options in input fields.
    #[serde(
        default,
        rename = "inputOption.activeBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub active_background: Option<String>,
    /// Foreground color of activated options in input fields.
    #[serde(
        default,
        rename = "inputOption.activeForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub active_foreground: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputValidationColors {
    /// Input validation background color for information severity.
    #[serde(
        default,
        rename = "inputValidation.infoBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub info_background: Option<String>,
    /// Input validation foreground color for information severity.
    #[serde(
        default,
        rename = "inputValidation.infoForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub info_foreground: Option<String>,
    /// Input validation border color for information severity.
    #[serde(
        default,
        rename = "inputValidation.infoBorder",
        deserialize_with = "empty_string_as_none"
    )]
    pub info_border: Option<String>,
    /// Input validation background color for warning severity.
    #[serde(
        default,
        rename = "inputValidation.warningBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub warning_background: Option<String>,
    /// Input validation foreground color for warning severity.
    #[serde(
        default,
        rename = "inputValidation.warningForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub warning_foreground: Option<String>,
    /// Input validation border color for warning severity.
    #[serde(
        default,
        rename = "inputValidation.warningBorder",
        deserialize_with = "empty_string_as_none"
    )]
    pub warning_border: Option<String>,
    /// Input validation background color for error severity.
    #[serde(
        default,
        rename = "inputValidation.errorBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub error_background: Option<String>,
    /// Input validation foreground color for error severity.
    #[serde(
        default,
        rename = "inputValidation.errorForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub error_foreground: Option<String>,
    /// Input validation border color for error severity.
    #[serde(
        default,
        rename = "inputValidation.errorBorder",
        deserialize_with = "empty_string_as_none"
    )]
    pub error_border: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DropdownColors {
    /// Dropdown background.
    #[serde(
        default,
        rename = "dropdown.background",
        deserialize_with = "empty_string_as_none"
    )]
    pub background: Option<String>,
    /// Dropdown list background.
    #[serde(
        default,
        rename = "dropdown.listBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub list_background: Option<String>,
    /// Dropdown foreground.
    #[serde(
        default,
        rename = "dropdown.foreground",
        deserialize_with = "empty_string_as_none"
    )]
    pub foreground: Option<String>,
    /// Dropdown border.
    #[serde(
        default,
        rename = "dropdown.border",
        deserialize_with = "empty_string_as_none"
    )]
    pub border: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ButtonColors {
    /// Button foreground color.
    #[serde(
        default,
        rename = "button.foreground",
        deserialize_with = "empty_string_as_none"
    )]
    pub foreground: Option<String>,
    /// Button separator color.
    #[serde(
        default,
        rename = "button.separator",
        deserialize_with = "empty_string_as_none"
    )]
    pub separator: Option<String>,
    /// Button background color.
    #[serde(
        default,
        rename = "button.background",
        deserialize_with = "empty_string_as_none"
    )]
    pub background: Option<String>,
    /// Button background color when hovering.
    #[serde(
        default,
        rename = "button.hoverBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub hover_background: Option<String>,
    /// Button border color.
    #[serde(
        default,
        rename = "button.border",
        deserialize_with = "empty_string_as_none"
    )]
    pub border: Option<String>,
    /// Secondary button foreground color.
    #[serde(
        default,
        rename = "button.secondaryForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub secondary_foreground: Option<String>,
    /// Secondary button background color.
    #[serde(
        default,
        rename = "button.secondaryBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub secondary_background: Option<String>,
    /// Secondary button background color when hovering.
    #[serde(
        default,
        rename = "button.secondaryHoverBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub secondary_hover_background: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BadgeColors {
    /// Badge background color. Badges are small information labels, e.g. for search results count.
    #[serde(
        default,
        rename = "badge.background",
        deserialize_with = "empty_string_as_none"
    )]
    pub background: Option<String>,
    /// Badge foreground color. Badges are small information labels, e.g. for search results count.
    #[serde(
        default,
        rename = "badge.foreground",
        deserialize_with = "empty_string_as_none"
    )]
    pub foreground: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScrollbarColors {
    /// Scrollbar shadow to indicate that the view is scrolled.
    #[serde(
        default,
        rename = "scrollbar.shadow",
        deserialize_with = "empty_string_as_none"
    )]
    pub shadow: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScrollbarSliderColors {
    /// Scrollbar slider background color.
    #[serde(
        default,
        rename = "scrollbarSlider.background",
        deserialize_with = "empty_string_as_none"
    )]
    pub background: Option<String>,
    /// Scrollbar slider background color when hovering.
    #[serde(
        default,
        rename = "scrollbarSlider.hoverBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub hover_background: Option<String>,
    /// Scrollbar slider background color when clicked on.
    #[serde(
        default,
        rename = "scrollbarSlider.activeBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub active_background: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProgressBarColors {
    /// Background color of the progress bar that can show for long running operations.
    #[serde(
        default,
        rename = "progressBar.background",
        deserialize_with = "empty_string_as_none"
    )]
    pub background: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EditorErrorColors {
    /// Background color of error text in the editor. The color must not be opaque so as not to hide underlying decorations.
    #[serde(
        default,
        rename = "editorError.background",
        deserialize_with = "empty_string_as_none"
    )]
    pub background: Option<String>,
    /// Foreground color of error squigglies in the editor.
    #[serde(
        default,
        rename = "editorError.foreground",
        deserialize_with = "empty_string_as_none"
    )]
    pub foreground: Option<String>,
    /// If set, color of double underlines for errors in the editor.
    #[serde(
        default,
        rename = "editorError.border",
        deserialize_with = "empty_string_as_none"
    )]
    pub border: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EditorWarningColors {
    /// Background color of warning text in the editor. The color must not be opaque so as not to hide underlying decorations.
    #[serde(
        default,
        rename = "editorWarning.background",
        deserialize_with = "empty_string_as_none"
    )]
    pub background: Option<String>,
    /// Foreground color of warning squigglies in the editor.
    #[serde(
        default,
        rename = "editorWarning.foreground",
        deserialize_with = "empty_string_as_none"
    )]
    pub foreground: Option<String>,
    /// If set, color of double underlines for warnings in the editor.
    #[serde(
        default,
        rename = "editorWarning.border",
        deserialize_with = "empty_string_as_none"
    )]
    pub border: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EditorInfoColors {
    /// Background color of info text in the editor. The color must not be opaque so as not to hide underlying decorations.
    #[serde(
        default,
        rename = "editorInfo.background",
        deserialize_with = "empty_string_as_none"
    )]
    pub background: Option<String>,
    /// Foreground color of info squigglies in the editor.
    #[serde(
        default,
        rename = "editorInfo.foreground",
        deserialize_with = "empty_string_as_none"
    )]
    pub foreground: Option<String>,
    /// If set, color of double underlines for infos in the editor.
    #[serde(
        default,
        rename = "editorInfo.border",
        deserialize_with = "empty_string_as_none"
    )]
    pub border: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EditorHintColors {
    /// Foreground color of hint squigglies in the editor.
    #[serde(
        default,
        rename = "editorHint.foreground",
        deserialize_with = "empty_string_as_none"
    )]
    pub foreground: Option<String>,
    /// If set, color of double underlines for hints in the editor.
    #[serde(
        default,
        rename = "editorHint.border",
        deserialize_with = "empty_string_as_none"
    )]
    pub border: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SashColors {
    /// Border color of active sashes.
    #[serde(
        default,
        rename = "sash.hoverBorder",
        deserialize_with = "empty_string_as_none"
    )]
    pub hover_border: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EditorColors {
    /// Editor background color.
    #[serde(
        default,
        rename = "editor.background",
        deserialize_with = "empty_string_as_none"
    )]
    pub background: Option<String>,
    /// Editor default foreground color.
    #[serde(
        default,
        rename = "editor.foreground",
        deserialize_with = "empty_string_as_none"
    )]
    pub foreground: Option<String>,
    /// Color of the editor selection.
    #[serde(
        default,
        rename = "editor.selectionBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub selection_background: Option<String>,
    /// Color of the selected text for high contrast.
    #[serde(
        default,
        rename = "editor.selectionForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub selection_foreground: Option<String>,
    /// Color of the selection in an inactive editor. The color must not be opaque so as not to hide underlying decorations.
    #[serde(
        default,
        rename = "editor.inactiveSelectionBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub inactive_selection_background: Option<String>,
    /// Color for regions with the same content as the selection. The color must not be opaque so as not to hide underlying decorations.
    #[serde(
        default,
        rename = "editor.selectionHighlightBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub selection_highlight_background: Option<String>,
    /// Border color for regions with the same content as the selection.
    #[serde(
        default,
        rename = "editor.selectionHighlightBorder",
        deserialize_with = "empty_string_as_none"
    )]
    pub selection_highlight_border: Option<String>,
    /// Color of the current search match.
    #[serde(
        default,
        rename = "editor.findMatchBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub find_match_background: Option<String>,
    /// Color of the other search matches. The color must not be opaque so as not to hide underlying decorations.
    #[serde(
        default,
        rename = "editor.findMatchHighlightBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub find_match_highlight_background: Option<String>,
    /// Color of the range limiting the search. The color must not be opaque so as not to hide underlying decorations.
    #[serde(
        default,
        rename = "editor.findRangeHighlightBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub find_range_highlight_background: Option<String>,
    /// Border color of the current search match.
    #[serde(
        default,
        rename = "editor.findMatchBorder",
        deserialize_with = "empty_string_as_none"
    )]
    pub find_match_border: Option<String>,
    /// Border color of the other search matches.
    #[serde(
        default,
        rename = "editor.findMatchHighlightBorder",
        deserialize_with = "empty_string_as_none"
    )]
    pub find_match_highlight_border: Option<String>,
    /// Border color of the range limiting the search. The color must not be opaque so as not to hide underlying decorations.
    #[serde(
        default,
        rename = "editor.findRangeHighlightBorder",
        deserialize_with = "empty_string_as_none"
    )]
    pub find_range_highlight_border: Option<String>,
    /// Highlight below the word for which a hover is shown. The color must not be opaque so as not to hide underlying decorations.
    #[serde(
        default,
        rename = "editor.hoverHighlightBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub hover_highlight_background: Option<String>,
    /// Highlight background color of a snippet tabstop.
    #[serde(
        default,
        rename = "editor.snippetTabstopHighlightBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub snippet_tabstop_highlight_background: Option<String>,
    /// Highlight border color of a snippet tabstop.
    #[serde(
        default,
        rename = "editor.snippetTabstopHighlightBorder",
        deserialize_with = "empty_string_as_none"
    )]
    pub snippet_tabstop_highlight_border: Option<String>,
    /// Highlight background color of the final tabstop of a snippet.
    #[serde(
        default,
        rename = "editor.snippetFinalTabstopHighlightBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub snippet_final_tabstop_highlight_background: Option<String>,
    /// Highlight border color of the final tabstop of a snippet.
    #[serde(
        default,
        rename = "editor.snippetFinalTabstopHighlightBorder",
        deserialize_with = "empty_string_as_none"
    )]
    pub snippet_final_tabstop_highlight_border: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EditorStickyScrollColors {
    /// Background color of sticky scroll in the editor
    #[serde(
        default,
        rename = "editorStickyScroll.background",
        deserialize_with = "empty_string_as_none"
    )]
    pub background: Option<String>,
    /// Border color of sticky scroll in the editor
    #[serde(
        default,
        rename = "editorStickyScroll.border",
        deserialize_with = "empty_string_as_none"
    )]
    pub border: Option<String>,
    ///  Shadow color of sticky scroll in the editor
    #[serde(
        default,
        rename = "editorStickyScroll.shadow",
        deserialize_with = "empty_string_as_none"
    )]
    pub shadow: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EditorStickyScrollHoverColors {
    /// Background color of sticky scroll on hover in the editor
    #[serde(
        default,
        rename = "editorStickyScrollHover.background",
        deserialize_with = "empty_string_as_none"
    )]
    pub background: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EditorWidgetColors {
    /// Background color of editor widgets, such as find/replace.
    #[serde(
        default,
        rename = "editorWidget.background",
        deserialize_with = "empty_string_as_none"
    )]
    pub background: Option<String>,
    /// Foreground color of editor widgets, such as find/replace.
    #[serde(
        default,
        rename = "editorWidget.foreground",
        deserialize_with = "empty_string_as_none"
    )]
    pub foreground: Option<String>,
    /// Border color of editor widgets. The color is only used if the widget chooses to have a border and if the color is not overridden by a widget.
    #[serde(
        default,
        rename = "editorWidget.border",
        deserialize_with = "empty_string_as_none"
    )]
    pub border: Option<String>,
    /// Border color of the resize bar of editor widgets. The color is only used if the widget chooses to have a resize border and if the color is not overridden by a widget.
    #[serde(
        default,
        rename = "editorWidget.resizeBorder",
        deserialize_with = "empty_string_as_none"
    )]
    pub resize_border: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuickInputColors {
    /// Quick picker background color. The quick picker widget is the container for pickers like the command palette.
    #[serde(
        default,
        rename = "quickInput.background",
        deserialize_with = "empty_string_as_none"
    )]
    pub background: Option<String>,
    /// Quick picker foreground color. The quick picker widget is the container for pickers like the command palette.
    #[serde(
        default,
        rename = "quickInput.foreground",
        deserialize_with = "empty_string_as_none"
    )]
    pub foreground: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuickInputTitleColors {
    /// Quick picker title background color. The quick picker widget is the container for pickers like the command palette.
    #[serde(
        default,
        rename = "quickInputTitle.background",
        deserialize_with = "empty_string_as_none"
    )]
    pub background: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PickerGroupColors {
    /// Quick picker color for grouping labels.
    #[serde(
        default,
        rename = "pickerGroup.foreground",
        deserialize_with = "empty_string_as_none"
    )]
    pub foreground: Option<String>,
    /// Quick picker color for grouping borders.
    #[serde(
        default,
        rename = "pickerGroup.border",
        deserialize_with = "empty_string_as_none"
    )]
    pub border: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeybindingLabelColors {
    /// Keybinding label background color. The keybinding label is used to represent a keyboard shortcut.
    #[serde(
        default,
        rename = "keybindingLabel.background",
        deserialize_with = "empty_string_as_none"
    )]
    pub background: Option<String>,
    /// Keybinding label foreground color. The keybinding label is used to represent a keyboard shortcut.
    #[serde(
        default,
        rename = "keybindingLabel.foreground",
        deserialize_with = "empty_string_as_none"
    )]
    pub foreground: Option<String>,
    /// Keybinding label border color. The keybinding label is used to represent a keyboard shortcut.
    #[serde(
        default,
        rename = "keybindingLabel.border",
        deserialize_with = "empty_string_as_none"
    )]
    pub border: Option<String>,
    /// Keybinding label border bottom color. The keybinding label is used to represent a keyboard shortcut.
    #[serde(
        default,
        rename = "keybindingLabel.bottomBorder",
        deserialize_with = "empty_string_as_none"
    )]
    pub bottom_border: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchEditorColors {
    /// Color of the Search Editor query matches.
    #[serde(
        default,
        rename = "searchEditor.findMatchBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub find_match_background: Option<String>,
    /// Border color of the Search Editor query matches.
    #[serde(
        default,
        rename = "searchEditor.findMatchBorder",
        deserialize_with = "empty_string_as_none"
    )]
    pub find_match_border: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchColors {
    /// Color of the text in the search viewlet's completion message.
    #[serde(
        default,
        rename = "search.resultsInfoForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub results_info_foreground: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EditorHoverWidgetColors {
    /// Background color of the editor hover.
    #[serde(
        default,
        rename = "editorHoverWidget.background",
        deserialize_with = "empty_string_as_none"
    )]
    pub background: Option<String>,
    /// Foreground color of the editor hover.
    #[serde(
        default,
        rename = "editorHoverWidget.foreground",
        deserialize_with = "empty_string_as_none"
    )]
    pub foreground: Option<String>,
    /// Border color of the editor hover.
    #[serde(
        default,
        rename = "editorHoverWidget.border",
        deserialize_with = "empty_string_as_none"
    )]
    pub border: Option<String>,
    /// Background color of the editor hover status bar.
    #[serde(
        default,
        rename = "editorHoverWidget.statusBarBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub status_bar_background: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EditorLinkColors {
    /// Color of active links.
    #[serde(
        default,
        rename = "editorLink.activeForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub active_foreground: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EditorInlayHintColors {
    /// Foreground color of inline hints
    #[serde(
        default,
        rename = "editorInlayHint.foreground",
        deserialize_with = "empty_string_as_none"
    )]
    pub foreground: Option<String>,
    /// Background color of inline hints
    #[serde(
        default,
        rename = "editorInlayHint.background",
        deserialize_with = "empty_string_as_none"
    )]
    pub background: Option<String>,
    /// Foreground color of inline hints for types
    #[serde(
        default,
        rename = "editorInlayHint.typeForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub type_foreground: Option<String>,
    /// Background color of inline hints for types
    #[serde(
        default,
        rename = "editorInlayHint.typeBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub type_background: Option<String>,
    /// Foreground color of inline hints for parameters
    #[serde(
        default,
        rename = "editorInlayHint.parameterForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub parameter_foreground: Option<String>,
    /// Background color of inline hints for parameters
    #[serde(
        default,
        rename = "editorInlayHint.parameterBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub parameter_background: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EditorLightBulbColors {
    /// The color used for the lightbulb actions icon.
    #[serde(
        default,
        rename = "editorLightBulb.foreground",
        deserialize_with = "empty_string_as_none"
    )]
    pub foreground: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EditorLightBulbAutoFixColors {
    /// The color used for the lightbulb auto fix actions icon.
    #[serde(
        default,
        rename = "editorLightBulbAutoFix.foreground",
        deserialize_with = "empty_string_as_none"
    )]
    pub foreground: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EditorLightBulbAiColors {
    /// The color used for the lightbulb AI icon.
    #[serde(
        default,
        rename = "editorLightBulbAi.foreground",
        deserialize_with = "empty_string_as_none"
    )]
    pub foreground: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiffEditorColors {
    /// Background color for text that got inserted. The color must not be opaque so as not to hide underlying decorations.
    #[serde(
        default,
        rename = "diffEditor.insertedTextBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub inserted_text_background: Option<String>,
    /// Background color for text that got removed. The color must not be opaque so as not to hide underlying decorations.
    #[serde(
        default,
        rename = "diffEditor.removedTextBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub removed_text_background: Option<String>,
    /// Background color for lines that got inserted. The color must not be opaque so as not to hide underlying decorations.
    #[serde(
        default,
        rename = "diffEditor.insertedLineBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub inserted_line_background: Option<String>,
    /// Background color for lines that got removed. The color must not be opaque so as not to hide underlying decorations.
    #[serde(
        default,
        rename = "diffEditor.removedLineBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub removed_line_background: Option<String>,
    /// Outline color for the text that got inserted.
    #[serde(
        default,
        rename = "diffEditor.insertedTextBorder",
        deserialize_with = "empty_string_as_none"
    )]
    pub inserted_text_border: Option<String>,
    /// Outline color for text that got removed.
    #[serde(
        default,
        rename = "diffEditor.removedTextBorder",
        deserialize_with = "empty_string_as_none"
    )]
    pub removed_text_border: Option<String>,
    /// Border color between the two text editors.
    #[serde(
        default,
        rename = "diffEditor.border",
        deserialize_with = "empty_string_as_none"
    )]
    pub border: Option<String>,
    /// Color of the diff editor's diagonal fill. The diagonal fill is used in side-by-side diff views.
    #[serde(
        default,
        rename = "diffEditor.diagonalFill",
        deserialize_with = "empty_string_as_none"
    )]
    pub diagonal_fill: Option<String>,
    /// The background color of unchanged blocks in the diff editor.
    #[serde(
        default,
        rename = "diffEditor.unchangedRegionBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub unchanged_region_background: Option<String>,
    /// The foreground color of unchanged blocks in the diff editor.
    #[serde(
        default,
        rename = "diffEditor.unchangedRegionForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub unchanged_region_foreground: Option<String>,
    /// The background color of unchanged code in the diff editor.
    #[serde(
        default,
        rename = "diffEditor.unchangedCodeBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub unchanged_code_background: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiffEditorGutterColors {
    /// Background color for the margin where lines got inserted.
    #[serde(
        default,
        rename = "diffEditorGutter.insertedLineBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub inserted_line_background: Option<String>,
    /// Background color for the margin where lines got removed.
    #[serde(
        default,
        rename = "diffEditorGutter.removedLineBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub removed_line_background: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiffEditorOverviewColors {
    /// Diff overview ruler foreground for inserted content.
    #[serde(
        default,
        rename = "diffEditorOverview.insertedForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub inserted_foreground: Option<String>,
    /// Diff overview ruler foreground for removed content.
    #[serde(
        default,
        rename = "diffEditorOverview.removedForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub removed_foreground: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListColors {
    /// List/Tree background color for the focused item when the list/tree is active. An active list/tree has keyboard focus, an inactive does not.
    #[serde(
        default,
        rename = "list.focusBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub focus_background: Option<String>,
    /// List/Tree foreground color for the focused item when the list/tree is active. An active list/tree has keyboard focus, an inactive does not.
    #[serde(
        default,
        rename = "list.focusForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub focus_foreground: Option<String>,
    /// List/Tree outline color for the focused item when the list/tree is active. An active list/tree has keyboard focus, an inactive does not.
    #[serde(
        default,
        rename = "list.focusOutline",
        deserialize_with = "empty_string_as_none"
    )]
    pub focus_outline: Option<String>,
    /// List/Tree outline color for the focused item when the list/tree is active and selected. An active list/tree has keyboard focus, an inactive does not.
    #[serde(
        default,
        rename = "list.focusAndSelectionOutline",
        deserialize_with = "empty_string_as_none"
    )]
    pub focus_and_selection_outline: Option<String>,
    /// List/Tree background color for the selected item when the list/tree is active. An active list/tree has keyboard focus, an inactive does not.
    #[serde(
        default,
        rename = "list.activeSelectionBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub active_selection_background: Option<String>,
    /// List/Tree foreground color for the selected item when the list/tree is active. An active list/tree has keyboard focus, an inactive does not.
    #[serde(
        default,
        rename = "list.activeSelectionForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub active_selection_foreground: Option<String>,
    /// List/Tree icon foreground color for the selected item when the list/tree is active. An active list/tree has keyboard focus, an inactive does not.
    #[serde(
        default,
        rename = "list.activeSelectionIconForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub active_selection_icon_foreground: Option<String>,
    /// List/Tree background color for the selected item when the list/tree is inactive. An active list/tree has keyboard focus, an inactive does not.
    #[serde(
        default,
        rename = "list.inactiveSelectionBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub inactive_selection_background: Option<String>,
    /// List/Tree foreground color for the selected item when the list/tree is inactive. An active list/tree has keyboard focus, an inactive does not.
    #[serde(
        default,
        rename = "list.inactiveSelectionForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub inactive_selection_foreground: Option<String>,
    /// List/Tree icon foreground color for the selected item when the list/tree is inactive. An active list/tree has keyboard focus, an inactive does not.
    #[serde(
        default,
        rename = "list.inactiveSelectionIconForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub inactive_selection_icon_foreground: Option<String>,
    /// List/Tree background color for the focused item when the list/tree is inactive. An active list/tree has keyboard focus, an inactive does not.
    #[serde(
        default,
        rename = "list.inactiveFocusBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub inactive_focus_background: Option<String>,
    /// List/Tree outline color for the focused item when the list/tree is inactive. An active list/tree has keyboard focus, an inactive does not.
    #[serde(
        default,
        rename = "list.inactiveFocusOutline",
        deserialize_with = "empty_string_as_none"
    )]
    pub inactive_focus_outline: Option<String>,
    /// List/Tree background when hovering over items using the mouse.
    #[serde(
        default,
        rename = "list.hoverBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub hover_background: Option<String>,
    /// List/Tree foreground when hovering over items using the mouse.
    #[serde(
        default,
        rename = "list.hoverForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub hover_foreground: Option<String>,
    /// List/Tree drag and drop background when moving items over other items when using the mouse.
    #[serde(
        default,
        rename = "list.dropBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub drop_background: Option<String>,
    /// List/Tree drag and drop border color when moving items between items when using the mouse.
    #[serde(
        default,
        rename = "list.dropBetweenBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub drop_between_background: Option<String>,
    /// List/Tree foreground color of the match highlights when searching inside the list/tree.
    #[serde(
        default,
        rename = "list.highlightForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub highlight_foreground: Option<String>,
    /// List/Tree foreground color of the match highlights on actively focused items when searching inside the list/tree.
    #[serde(
        default,
        rename = "list.focusHighlightForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub focus_highlight_foreground: Option<String>,
    /// List/Tree foreground color for invalid items, for example an unresolved root in explorer.
    #[serde(
        default,
        rename = "list.invalidItemForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub invalid_item_foreground: Option<String>,
    /// Foreground color of list items containing errors.
    #[serde(
        default,
        rename = "list.errorForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub error_foreground: Option<String>,
    /// Foreground color of list items containing warnings.
    #[serde(
        default,
        rename = "list.warningForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub warning_foreground: Option<String>,
    /// Background color of the filtered match.
    #[serde(
        default,
        rename = "list.filterMatchBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub filter_match_background: Option<String>,
    /// Border color of the filtered match.
    #[serde(
        default,
        rename = "list.filterMatchBorder",
        deserialize_with = "empty_string_as_none"
    )]
    pub filter_match_border: Option<String>,
    /// List/Tree foreground color for items that are deemphasized.
    #[serde(
        default,
        rename = "list.deemphasizedForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub deemphasized_foreground: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListFilterWidgetColors {
    /// Background color of the type filter widget in lists and trees.
    #[serde(
        default,
        rename = "listFilterWidget.background",
        deserialize_with = "empty_string_as_none"
    )]
    pub background: Option<String>,
    /// Outline color of the type filter widget in lists and trees.
    #[serde(
        default,
        rename = "listFilterWidget.outline",
        deserialize_with = "empty_string_as_none"
    )]
    pub outline: Option<String>,
    /// Outline color of the type filter widget in lists and trees, when there are no matches.
    #[serde(
        default,
        rename = "listFilterWidget.noMatchesOutline",
        deserialize_with = "empty_string_as_none"
    )]
    pub no_matches_outline: Option<String>,
    /// Shadow color of the type filter widget in lists and trees.
    #[serde(
        default,
        rename = "listFilterWidget.shadow",
        deserialize_with = "empty_string_as_none"
    )]
    pub shadow: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TreeColors {
    /// Tree stroke color for the indentation guides.
    #[serde(
        default,
        rename = "tree.indentGuidesStroke",
        deserialize_with = "empty_string_as_none"
    )]
    pub indent_guides_stroke: Option<String>,
    /// Tree stroke color for the indentation guides that are not active.
    #[serde(
        default,
        rename = "tree.inactiveIndentGuidesStroke",
        deserialize_with = "empty_string_as_none"
    )]
    pub inactive_indent_guides_stroke: Option<String>,
    /// Table border color between columns.
    #[serde(
        default,
        rename = "tree.tableColumnsBorder",
        deserialize_with = "empty_string_as_none"
    )]
    pub table_columns_border: Option<String>,
    /// Background color for odd table rows.
    #[serde(
        default,
        rename = "tree.tableOddRowsBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub table_odd_rows_background: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckboxColors {
    /// Background color of checkbox widget.
    #[serde(
        default,
        rename = "checkbox.background",
        deserialize_with = "empty_string_as_none"
    )]
    pub background: Option<String>,
    /// Background color of checkbox widget when the element it's in is selected.
    #[serde(
        default,
        rename = "checkbox.selectBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub select_background: Option<String>,
    /// Foreground color of checkbox widget.
    #[serde(
        default,
        rename = "checkbox.foreground",
        deserialize_with = "empty_string_as_none"
    )]
    pub foreground: Option<String>,
    /// Border color of checkbox widget.
    #[serde(
        default,
        rename = "checkbox.border",
        deserialize_with = "empty_string_as_none"
    )]
    pub border: Option<String>,
    /// Border color of checkbox widget when the element it's in is selected.
    #[serde(
        default,
        rename = "checkbox.selectBorder",
        deserialize_with = "empty_string_as_none"
    )]
    pub select_border: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuickInputListColors {
    /// Quick picker foreground color for the focused item.
    #[serde(
        default,
        rename = "quickInputList.focusForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub focus_foreground: Option<String>,
    /// Quick picker icon foreground color for the focused item.
    #[serde(
        default,
        rename = "quickInputList.focusIconForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub focus_icon_foreground: Option<String>,
    /// Quick picker background color for the focused item.
    #[serde(
        default,
        rename = "quickInputList.focusBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub focus_background: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MenuColors {
    /// Border color of menus.
    #[serde(
        default,
        rename = "menu.border",
        deserialize_with = "empty_string_as_none"
    )]
    pub border: Option<String>,
    /// Foreground color of menu items.
    #[serde(
        default,
        rename = "menu.foreground",
        deserialize_with = "empty_string_as_none"
    )]
    pub foreground: Option<String>,
    /// Background color of menu items.
    #[serde(
        default,
        rename = "menu.background",
        deserialize_with = "empty_string_as_none"
    )]
    pub background: Option<String>,
    /// Foreground color of the selected menu item in menus.
    #[serde(
        default,
        rename = "menu.selectionForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub selection_foreground: Option<String>,
    /// Background color of the selected menu item in menus.
    #[serde(
        default,
        rename = "menu.selectionBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub selection_background: Option<String>,
    /// Border color of the selected menu item in menus.
    #[serde(
        default,
        rename = "menu.selectionBorder",
        deserialize_with = "empty_string_as_none"
    )]
    pub selection_border: Option<String>,
    /// Color of a separator menu item in menus.
    #[serde(
        default,
        rename = "menu.separatorBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub separator_background: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolbarColors {
    /// Toolbar background when hovering over actions using the mouse
    #[serde(
        default,
        rename = "toolbar.hoverBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub hover_background: Option<String>,
    /// Toolbar outline when hovering over actions using the mouse
    #[serde(
        default,
        rename = "toolbar.hoverOutline",
        deserialize_with = "empty_string_as_none"
    )]
    pub hover_outline: Option<String>,
    /// Toolbar background when holding the mouse over actions
    #[serde(
        default,
        rename = "toolbar.activeBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub active_background: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BreadcrumbColors {
    /// Color of focused breadcrumb items.
    #[serde(
        default,
        rename = "breadcrumb.foreground",
        deserialize_with = "empty_string_as_none"
    )]
    pub foreground: Option<String>,
    /// Background color of breadcrumb items.
    #[serde(
        default,
        rename = "breadcrumb.background",
        deserialize_with = "empty_string_as_none"
    )]
    pub background: Option<String>,
    /// Color of focused breadcrumb items.
    #[serde(
        default,
        rename = "breadcrumb.focusForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub focus_foreground: Option<String>,
    /// Color of selected breadcrumb items.
    #[serde(
        default,
        rename = "breadcrumb.activeSelectionForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub active_selection_foreground: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BreadcrumbPickerColors {
    /// Background color of breadcrumb item picker.
    #[serde(
        default,
        rename = "breadcrumbPicker.background",
        deserialize_with = "empty_string_as_none"
    )]
    pub background: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MergeColors {
    /// Current header background in inline merge-conflicts. The color must not be opaque so as not to hide underlying decorations.
    #[serde(
        default,
        rename = "merge.currentHeaderBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub current_header_background: Option<String>,
    /// Current content background in inline merge-conflicts. The color must not be opaque so as not to hide underlying decorations.
    #[serde(
        default,
        rename = "merge.currentContentBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub current_content_background: Option<String>,
    /// Incoming header background in inline merge-conflicts. The color must not be opaque so as not to hide underlying decorations.
    #[serde(
        default,
        rename = "merge.incomingHeaderBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub incoming_header_background: Option<String>,
    /// Incoming content background in inline merge-conflicts. The color must not be opaque so as not to hide underlying decorations.
    #[serde(
        default,
        rename = "merge.incomingContentBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub incoming_content_background: Option<String>,
    /// Common ancestor header background in inline merge-conflicts. The color must not be opaque so as not to hide underlying decorations.
    #[serde(
        default,
        rename = "merge.commonHeaderBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub common_header_background: Option<String>,
    /// Common ancestor content background in inline merge-conflicts. The color must not be opaque so as not to hide underlying decorations.
    #[serde(
        default,
        rename = "merge.commonContentBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub common_content_background: Option<String>,
    /// Border color on headers and the splitter in inline merge-conflicts.
    #[serde(
        default,
        rename = "merge.border",
        deserialize_with = "empty_string_as_none"
    )]
    pub border: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EditorOverviewRulerColors {
    /// Current overview ruler foreground for inline merge-conflicts.
    #[serde(
        default,
        rename = "editorOverviewRuler.currentContentForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub current_content_foreground: Option<String>,
    /// Incoming overview ruler foreground for inline merge-conflicts.
    #[serde(
        default,
        rename = "editorOverviewRuler.incomingContentForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub incoming_content_foreground: Option<String>,
    /// Common ancestor overview ruler foreground for inline merge-conflicts.
    #[serde(
        default,
        rename = "editorOverviewRuler.commonContentForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub common_content_foreground: Option<String>,
    /// Overview ruler marker color for find matches. The color must not be opaque so as not to hide underlying decorations.
    #[serde(
        default,
        rename = "editorOverviewRuler.findMatchForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub find_match_foreground: Option<String>,
    /// Overview ruler marker color for selection highlights. The color must not be opaque so as not to hide underlying decorations.
    #[serde(
        default,
        rename = "editorOverviewRuler.selectionHighlightForeground",
        deserialize_with = "empty_string_as_none"
    )]
    pub selection_highlight_foreground: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MinimapColors {
    /// Minimap marker color for find matches.
    #[serde(
        default,
        rename = "minimap.findMatchHighlight",
        deserialize_with = "empty_string_as_none"
    )]
    pub find_match_highlight: Option<String>,
    /// Minimap marker color for repeating editor selections.
    #[serde(
        default,
        rename = "minimap.selectionOccurrenceHighlight",
        deserialize_with = "empty_string_as_none"
    )]
    pub selection_occurrence_highlight: Option<String>,
    /// Minimap marker color for the editor selection.
    #[serde(
        default,
        rename = "minimap.selectionHighlight",
        deserialize_with = "empty_string_as_none"
    )]
    pub selection_highlight: Option<String>,
    /// Minimap marker color for infos.
    #[serde(
        default,
        rename = "minimap.infoHighlight",
        deserialize_with = "empty_string_as_none"
    )]
    pub info_highlight: Option<String>,
    /// Minimap marker color for warnings.
    #[serde(
        default,
        rename = "minimap.warningHighlight",
        deserialize_with = "empty_string_as_none"
    )]
    pub warning_highlight: Option<String>,
    /// Minimap marker color for errors.
    #[serde(
        default,
        rename = "minimap.errorHighlight",
        deserialize_with = "empty_string_as_none"
    )]
    pub error_highlight: Option<String>,
    /// Minimap background color.
    #[serde(
        default,
        rename = "minimap.background",
        deserialize_with = "empty_string_as_none"
    )]
    pub background: Option<String>,
    /// Opacity of foreground elements rendered in the minimap. For example, "#000000c0" will render the elements with 75% opacity.
    #[serde(
        default,
        rename = "minimap.foregroundOpacity",
        deserialize_with = "empty_string_as_none"
    )]
    pub foreground_opacity: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MinimapSliderColors {
    /// Minimap slider background color.
    #[serde(
        default,
        rename = "minimapSlider.background",
        deserialize_with = "empty_string_as_none"
    )]
    pub background: Option<String>,
    /// Minimap slider background color when hovering.
    #[serde(
        default,
        rename = "minimapSlider.hoverBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub hover_background: Option<String>,
    /// Minimap slider background color when clicked on.
    #[serde(
        default,
        rename = "minimapSlider.activeBackground",
        deserialize_with = "empty_string_as_none"
    )]
    pub active_background: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProblemsErrorIconColors {
    /// The color used for the problems error icon.
    #[serde(
        default,
        rename = "problemsErrorIcon.foreground",
        deserialize_with = "empty_string_as_none"
    )]
    pub foreground: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProblemsWarningIconColors {
    /// The color used for the problems warning icon.
    #[serde(
        default,
        rename = "problemsWarningIcon.foreground",
        deserialize_with = "empty_string_as_none"
    )]
    pub foreground: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProblemsInfoIconColors {
    /// The color used for the problems info icon.
    #[serde(
        default,
        rename = "problemsInfoIcon.foreground",
        deserialize_with = "empty_string_as_none"
    )]
    pub foreground: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChartsColors {
    /// The foreground color used in charts.
    #[serde(
        default,
        rename = "charts.foreground",
        deserialize_with = "empty_string_as_none"
    )]
    pub foreground: Option<String>,
    /// The color used for horizontal lines in charts.
    #[serde(
        default,
        rename = "charts.lines",
        deserialize_with = "empty_string_as_none"
    )]
    pub lines: Option<String>,
    /// The red color used in chart visualizations.
    #[serde(
        default,
        rename = "charts.red",
        deserialize_with = "empty_string_as_none"
    )]
    pub red: Option<String>,
    /// The blue color used in chart visualizations.
    #[serde(
        default,
        rename = "charts.blue",
        deserialize_with = "empty_string_as_none"
    )]
    pub blue: Option<String>,
    /// The yellow color used in chart visualizations.
    #[serde(
        default,
        rename = "charts.yellow",
        deserialize_with = "empty_string_as_none"
    )]
    pub yellow: Option<String>,
    /// The orange color used in chart visualizations.
    #[serde(
        default,
        rename = "charts.orange",
        deserialize_with = "empty_string_as_none"
    )]
    pub orange: Option<String>,
    /// The green color used in chart visualizations.
    #[serde(
        default,
        rename = "charts.green",
        deserialize_with = "empty_string_as_none"
    )]
    pub green: Option<String>,
    /// The purple color used in chart visualizations.
    #[serde(
        default,
        rename = "charts.purple",
        deserialize_with = "empty_string_as_none"
    )]
    pub purple: Option<String>,
}
