use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Colors {
    /// Overall foreground color. This color is only used if not overridden by a component.
    #[serde(rename = "foreground")]
    pub foreground: Option<String>,
    /// Overall foreground for disabled elements. This color is only used if not overridden by a component.
    #[serde(rename = "disabledForeground")]
    pub disabled_foreground: Option<String>,
    /// Overall foreground color for error messages. This color is only used if not overridden by a component.
    #[serde(rename = "errorForeground")]
    pub error_foreground: Option<String>,
    /// Foreground color for description text providing additional information, for example for a label.
    #[serde(rename = "descriptionForeground")]
    pub description_foreground: Option<String>,
    /// Overall border color for focused elements. This color is only used if not overridden by a component.
    #[serde(rename = "focusBorder")]
    pub focus_border: Option<String>,
    /// An extra border around elements to separate them from others for greater contrast.
    #[serde(rename = "contrastBorder")]
    pub contrast_border: Option<String>,
    /// An extra border around active elements to separate them from others for greater contrast.
    #[serde(rename = "contrastActiveBorder")]
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
    #[serde(rename = "icon.foreground")]
    pub foreground: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelectionColors {
    /// The background color of text selections in the workbench (e.g. for input fields or text areas). Note that this does not apply to selections within the editor.
    #[serde(rename = "selection.background")]
    pub background: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextSeparatorColors {
    /// Color for text separators.
    #[serde(rename = "textSeparator.foreground")]
    pub foreground: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextLinkColors {
    /// Foreground color for links in text.
    #[serde(rename = "textLink.foreground")]
    pub foreground: Option<String>,
    /// Foreground color for links in text when clicked on and on mouse hover.
    #[serde(rename = "textLink.activeForeground")]
    pub active_foreground: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextPreformatColors {
    /// Foreground color for preformatted text segments.
    #[serde(rename = "textPreformat.foreground")]
    pub foreground: Option<String>,
    /// Background color for preformatted text segments.
    #[serde(rename = "textPreformat.background")]
    pub background: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextBlockQuoteColors {
    /// Background color for block quotes in text.
    #[serde(rename = "textBlockQuote.background")]
    pub background: Option<String>,
    /// Border color for block quotes in text.
    #[serde(rename = "textBlockQuote.border")]
    pub border: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextCodeBlockColors {
    /// Background color for code blocks in text.
    #[serde(rename = "textCodeBlock.background")]
    pub background: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WidgetColors {
    /// Shadow color of widgets such as find/replace inside the editor.
    #[serde(rename = "widget.shadow")]
    pub shadow: Option<String>,
    /// Border color of widgets such as find/replace inside the editor.
    #[serde(rename = "widget.border")]
    pub border: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputColors {
    /// Input box background.
    #[serde(rename = "input.background")]
    pub background: Option<String>,
    /// Input box foreground.
    #[serde(rename = "input.foreground")]
    pub foreground: Option<String>,
    /// Input box border.
    #[serde(rename = "input.border")]
    pub border: Option<String>,
    /// Input box foreground color for placeholder text.
    #[serde(rename = "input.placeholderForeground")]
    pub placeholder_foreground: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputOptionColors {
    /// Border color of activated options in input fields.
    #[serde(rename = "inputOption.activeBorder")]
    pub active_border: Option<String>,
    /// Background color of activated options in input fields.
    #[serde(rename = "inputOption.hoverBackground")]
    pub hover_background: Option<String>,
    /// Background hover color of options in input fields.
    #[serde(rename = "inputOption.activeBackground")]
    pub active_background: Option<String>,
    /// Foreground color of activated options in input fields.
    #[serde(rename = "inputOption.activeForeground")]
    pub active_foreground: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputValidationColors {
    /// Input validation background color for information severity.
    #[serde(rename = "inputValidation.infoBackground")]
    pub info_background: Option<String>,
    /// Input validation foreground color for information severity.
    #[serde(rename = "inputValidation.infoForeground")]
    pub info_foreground: Option<String>,
    /// Input validation border color for information severity.
    #[serde(rename = "inputValidation.infoBorder")]
    pub info_border: Option<String>,
    /// Input validation background color for warning severity.
    #[serde(rename = "inputValidation.warningBackground")]
    pub warning_background: Option<String>,
    /// Input validation foreground color for warning severity.
    #[serde(rename = "inputValidation.warningForeground")]
    pub warning_foreground: Option<String>,
    /// Input validation border color for warning severity.
    #[serde(rename = "inputValidation.warningBorder")]
    pub warning_border: Option<String>,
    /// Input validation background color for error severity.
    #[serde(rename = "inputValidation.errorBackground")]
    pub error_background: Option<String>,
    /// Input validation foreground color for error severity.
    #[serde(rename = "inputValidation.errorForeground")]
    pub error_foreground: Option<String>,
    /// Input validation border color for error severity.
    #[serde(rename = "inputValidation.errorBorder")]
    pub error_border: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DropdownColors {
    /// Dropdown background.
    #[serde(rename = "dropdown.background")]
    pub background: Option<String>,
    /// Dropdown list background.
    #[serde(rename = "dropdown.listBackground")]
    pub list_background: Option<String>,
    /// Dropdown foreground.
    #[serde(rename = "dropdown.foreground")]
    pub foreground: Option<String>,
    /// Dropdown border.
    #[serde(rename = "dropdown.border")]
    pub border: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ButtonColors {
    /// Button foreground color.
    #[serde(rename = "button.foreground")]
    pub foreground: Option<String>,
    /// Button separator color.
    #[serde(rename = "button.separator")]
    pub separator: Option<String>,
    /// Button background color.
    #[serde(rename = "button.background")]
    pub background: Option<String>,
    /// Button background color when hovering.
    #[serde(rename = "button.hoverBackground")]
    pub hover_background: Option<String>,
    /// Button border color.
    #[serde(rename = "button.border")]
    pub border: Option<String>,
    /// Secondary button foreground color.
    #[serde(rename = "button.secondaryForeground")]
    pub secondary_foreground: Option<String>,
    /// Secondary button background color.
    #[serde(rename = "button.secondaryBackground")]
    pub secondary_background: Option<String>,
    /// Secondary button background color when hovering.
    #[serde(rename = "button.secondaryHoverBackground")]
    pub secondary_hover_background: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BadgeColors {
    /// Badge background color. Badges are small information labels, e.g. for search results count.
    #[serde(rename = "badge.background")]
    pub background: Option<String>,
    /// Badge foreground color. Badges are small information labels, e.g. for search results count.
    #[serde(rename = "badge.foreground")]
    pub foreground: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScrollbarColors {
    /// Scrollbar shadow to indicate that the view is scrolled.
    #[serde(rename = "scrollbar.shadow")]
    pub shadow: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScrollbarSliderColors {
    /// Scrollbar slider background color.
    #[serde(rename = "scrollbarSlider.background")]
    pub background: Option<String>,
    /// Scrollbar slider background color when hovering.
    #[serde(rename = "scrollbarSlider.hoverBackground")]
    pub hover_background: Option<String>,
    /// Scrollbar slider background color when clicked on.
    #[serde(rename = "scrollbarSlider.activeBackground")]
    pub active_background: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProgressBarColors {
    /// Background color of the progress bar that can show for long running operations.
    #[serde(rename = "progressBar.background")]
    pub background: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EditorErrorColors {
    /// Background color of error text in the editor. The color must not be opaque so as not to hide underlying decorations.
    #[serde(rename = "editorError.background")]
    pub background: Option<String>,
    /// Foreground color of error squigglies in the editor.
    #[serde(rename = "editorError.foreground")]
    pub foreground: Option<String>,
    /// If set, color of double underlines for errors in the editor.
    #[serde(rename = "editorError.border")]
    pub border: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EditorWarningColors {
    /// Background color of warning text in the editor. The color must not be opaque so as not to hide underlying decorations.
    #[serde(rename = "editorWarning.background")]
    pub background: Option<String>,
    /// Foreground color of warning squigglies in the editor.
    #[serde(rename = "editorWarning.foreground")]
    pub foreground: Option<String>,
    /// If set, color of double underlines for warnings in the editor.
    #[serde(rename = "editorWarning.border")]
    pub border: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EditorInfoColors {
    /// Background color of info text in the editor. The color must not be opaque so as not to hide underlying decorations.
    #[serde(rename = "editorInfo.background")]
    pub background: Option<String>,
    /// Foreground color of info squigglies in the editor.
    #[serde(rename = "editorInfo.foreground")]
    pub foreground: Option<String>,
    /// If set, color of double underlines for infos in the editor.
    #[serde(rename = "editorInfo.border")]
    pub border: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EditorHintColors {
    /// Foreground color of hint squigglies in the editor.
    #[serde(rename = "editorHint.foreground")]
    pub foreground: Option<String>,
    /// If set, color of double underlines for hints in the editor.
    #[serde(rename = "editorHint.border")]
    pub border: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SashColors {
    /// Border color of active sashes.
    #[serde(rename = "sash.hoverBorder")]
    pub hover_border: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EditorColors {
    /// Editor background color.
    #[serde(rename = "editor.background")]
    pub background: Option<String>,
    /// Editor default foreground color.
    #[serde(rename = "editor.foreground")]
    pub foreground: Option<String>,
    /// Color of the editor selection.
    #[serde(rename = "editor.selectionBackground")]
    pub selection_background: Option<String>,
    /// Color of the selected text for high contrast.
    #[serde(rename = "editor.selectionForeground")]
    pub selection_foreground: Option<String>,
    /// Color of the selection in an inactive editor. The color must not be opaque so as not to hide underlying decorations.
    #[serde(rename = "editor.inactiveSelectionBackground")]
    pub inactive_selection_background: Option<String>,
    /// Color for regions with the same content as the selection. The color must not be opaque so as not to hide underlying decorations.
    #[serde(rename = "editor.selectionHighlightBackground")]
    pub selection_highlight_background: Option<String>,
    /// Border color for regions with the same content as the selection.
    #[serde(rename = "editor.selectionHighlightBorder")]
    pub selection_highlight_border: Option<String>,
    /// Color of the current search match.
    #[serde(rename = "editor.findMatchBackground")]
    pub find_match_background: Option<String>,
    /// Color of the other search matches. The color must not be opaque so as not to hide underlying decorations.
    #[serde(rename = "editor.findMatchHighlightBackground")]
    pub find_match_highlight_background: Option<String>,
    /// Color of the range limiting the search. The color must not be opaque so as not to hide underlying decorations.
    #[serde(rename = "editor.findRangeHighlightBackground")]
    pub find_range_highlight_background: Option<String>,
    /// Border color of the current search match.
    #[serde(rename = "editor.findMatchBorder")]
    pub find_match_border: Option<String>,
    /// Border color of the other search matches.
    #[serde(rename = "editor.findMatchHighlightBorder")]
    pub find_match_highlight_border: Option<String>,
    /// Border color of the range limiting the search. The color must not be opaque so as not to hide underlying decorations.
    #[serde(rename = "editor.findRangeHighlightBorder")]
    pub find_range_highlight_border: Option<String>,
    /// Highlight below the word for which a hover is shown. The color must not be opaque so as not to hide underlying decorations.
    #[serde(rename = "editor.hoverHighlightBackground")]
    pub hover_highlight_background: Option<String>,
    /// Highlight background color of a snippet tabstop.
    #[serde(rename = "editor.snippetTabstopHighlightBackground")]
    pub snippet_tabstop_highlight_background: Option<String>,
    /// Highlight border color of a snippet tabstop.
    #[serde(rename = "editor.snippetTabstopHighlightBorder")]
    pub snippet_tabstop_highlight_border: Option<String>,
    /// Highlight background color of the final tabstop of a snippet.
    #[serde(rename = "editor.snippetFinalTabstopHighlightBackground")]
    pub snippet_final_tabstop_highlight_background: Option<String>,
    /// Highlight border color of the final tabstop of a snippet.
    #[serde(rename = "editor.snippetFinalTabstopHighlightBorder")]
    pub snippet_final_tabstop_highlight_border: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EditorStickyScrollColors {
    /// Background color of sticky scroll in the editor
    #[serde(rename = "editorStickyScroll.background")]
    pub background: Option<String>,
    /// Border color of sticky scroll in the editor
    #[serde(rename = "editorStickyScroll.border")]
    pub border: Option<String>,
    ///  Shadow color of sticky scroll in the editor
    #[serde(rename = "editorStickyScroll.shadow")]
    pub shadow: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EditorStickyScrollHoverColors {
    /// Background color of sticky scroll on hover in the editor
    #[serde(rename = "editorStickyScrollHover.background")]
    pub background: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EditorWidgetColors {
    /// Background color of editor widgets, such as find/replace.
    #[serde(rename = "editorWidget.background")]
    pub background: Option<String>,
    /// Foreground color of editor widgets, such as find/replace.
    #[serde(rename = "editorWidget.foreground")]
    pub foreground: Option<String>,
    /// Border color of editor widgets. The color is only used if the widget chooses to have a border and if the color is not overridden by a widget.
    #[serde(rename = "editorWidget.border")]
    pub border: Option<String>,
    /// Border color of the resize bar of editor widgets. The color is only used if the widget chooses to have a resize border and if the color is not overridden by a widget.
    #[serde(rename = "editorWidget.resizeBorder")]
    pub resize_border: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuickInputColors {
    /// Quick picker background color. The quick picker widget is the container for pickers like the command palette.
    #[serde(rename = "quickInput.background")]
    pub background: Option<String>,
    /// Quick picker foreground color. The quick picker widget is the container for pickers like the command palette.
    #[serde(rename = "quickInput.foreground")]
    pub foreground: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuickInputTitleColors {
    /// Quick picker title background color. The quick picker widget is the container for pickers like the command palette.
    #[serde(rename = "quickInputTitle.background")]
    pub background: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PickerGroupColors {
    /// Quick picker color for grouping labels.
    #[serde(rename = "pickerGroup.foreground")]
    pub foreground: Option<String>,
    /// Quick picker color for grouping borders.
    #[serde(rename = "pickerGroup.border")]
    pub border: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeybindingLabelColors {
    /// Keybinding label background color. The keybinding label is used to represent a keyboard shortcut.
    #[serde(rename = "keybindingLabel.background")]
    pub background: Option<String>,
    /// Keybinding label foreground color. The keybinding label is used to represent a keyboard shortcut.
    #[serde(rename = "keybindingLabel.foreground")]
    pub foreground: Option<String>,
    /// Keybinding label border color. The keybinding label is used to represent a keyboard shortcut.
    #[serde(rename = "keybindingLabel.border")]
    pub border: Option<String>,
    /// Keybinding label border bottom color. The keybinding label is used to represent a keyboard shortcut.
    #[serde(rename = "keybindingLabel.bottomBorder")]
    pub bottom_border: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchEditorColors {
    /// Color of the Search Editor query matches.
    #[serde(rename = "searchEditor.findMatchBackground")]
    pub find_match_background: Option<String>,
    /// Border color of the Search Editor query matches.
    #[serde(rename = "searchEditor.findMatchBorder")]
    pub find_match_border: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchColors {
    /// Color of the text in the search viewlet's completion message.
    #[serde(rename = "search.resultsInfoForeground")]
    pub results_info_foreground: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EditorHoverWidgetColors {
    /// Background color of the editor hover.
    #[serde(rename = "editorHoverWidget.background")]
    pub background: Option<String>,
    /// Foreground color of the editor hover.
    #[serde(rename = "editorHoverWidget.foreground")]
    pub foreground: Option<String>,
    /// Border color of the editor hover.
    #[serde(rename = "editorHoverWidget.border")]
    pub border: Option<String>,
    /// Background color of the editor hover status bar.
    #[serde(rename = "editorHoverWidget.statusBarBackground")]
    pub status_bar_background: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EditorLinkColors {
    /// Color of active links.
    #[serde(rename = "editorLink.activeForeground")]
    pub active_foreground: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EditorInlayHintColors {
    /// Foreground color of inline hints
    #[serde(rename = "editorInlayHint.foreground")]
    pub foreground: Option<String>,
    /// Background color of inline hints
    #[serde(rename = "editorInlayHint.background")]
    pub background: Option<String>,
    /// Foreground color of inline hints for types
    #[serde(rename = "editorInlayHint.typeForeground")]
    pub type_foreground: Option<String>,
    /// Background color of inline hints for types
    #[serde(rename = "editorInlayHint.typeBackground")]
    pub type_background: Option<String>,
    /// Foreground color of inline hints for parameters
    #[serde(rename = "editorInlayHint.parameterForeground")]
    pub parameter_foreground: Option<String>,
    /// Background color of inline hints for parameters
    #[serde(rename = "editorInlayHint.parameterBackground")]
    pub parameter_background: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EditorLightBulbColors {
    /// The color used for the lightbulb actions icon.
    #[serde(rename = "editorLightBulb.foreground")]
    pub foreground: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EditorLightBulbAutoFixColors {
    /// The color used for the lightbulb auto fix actions icon.
    #[serde(rename = "editorLightBulbAutoFix.foreground")]
    pub foreground: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EditorLightBulbAiColors {
    /// The color used for the lightbulb AI icon.
    #[serde(rename = "editorLightBulbAi.foreground")]
    pub foreground: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiffEditorColors {
    /// Background color for text that got inserted. The color must not be opaque so as not to hide underlying decorations.
    #[serde(rename = "diffEditor.insertedTextBackground")]
    pub inserted_text_background: Option<String>,
    /// Background color for text that got removed. The color must not be opaque so as not to hide underlying decorations.
    #[serde(rename = "diffEditor.removedTextBackground")]
    pub removed_text_background: Option<String>,
    /// Background color for lines that got inserted. The color must not be opaque so as not to hide underlying decorations.
    #[serde(rename = "diffEditor.insertedLineBackground")]
    pub inserted_line_background: Option<String>,
    /// Background color for lines that got removed. The color must not be opaque so as not to hide underlying decorations.
    #[serde(rename = "diffEditor.removedLineBackground")]
    pub removed_line_background: Option<String>,
    /// Outline color for the text that got inserted.
    #[serde(rename = "diffEditor.insertedTextBorder")]
    pub inserted_text_border: Option<String>,
    /// Outline color for text that got removed.
    #[serde(rename = "diffEditor.removedTextBorder")]
    pub removed_text_border: Option<String>,
    /// Border color between the two text editors.
    #[serde(rename = "diffEditor.border")]
    pub border: Option<String>,
    /// Color of the diff editor's diagonal fill. The diagonal fill is used in side-by-side diff views.
    #[serde(rename = "diffEditor.diagonalFill")]
    pub diagonal_fill: Option<String>,
    /// The background color of unchanged blocks in the diff editor.
    #[serde(rename = "diffEditor.unchangedRegionBackground")]
    pub unchanged_region_background: Option<String>,
    /// The foreground color of unchanged blocks in the diff editor.
    #[serde(rename = "diffEditor.unchangedRegionForeground")]
    pub unchanged_region_foreground: Option<String>,
    /// The background color of unchanged code in the diff editor.
    #[serde(rename = "diffEditor.unchangedCodeBackground")]
    pub unchanged_code_background: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiffEditorGutterColors {
    /// Background color for the margin where lines got inserted.
    #[serde(rename = "diffEditorGutter.insertedLineBackground")]
    pub inserted_line_background: Option<String>,
    /// Background color for the margin where lines got removed.
    #[serde(rename = "diffEditorGutter.removedLineBackground")]
    pub removed_line_background: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiffEditorOverviewColors {
    /// Diff overview ruler foreground for inserted content.
    #[serde(rename = "diffEditorOverview.insertedForeground")]
    pub inserted_foreground: Option<String>,
    /// Diff overview ruler foreground for removed content.
    #[serde(rename = "diffEditorOverview.removedForeground")]
    pub removed_foreground: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListColors {
    /// List/Tree background color for the focused item when the list/tree is active. An active list/tree has keyboard focus, an inactive does not.
    #[serde(rename = "list.focusBackground")]
    pub focus_background: Option<String>,
    /// List/Tree foreground color for the focused item when the list/tree is active. An active list/tree has keyboard focus, an inactive does not.
    #[serde(rename = "list.focusForeground")]
    pub focus_foreground: Option<String>,
    /// List/Tree outline color for the focused item when the list/tree is active. An active list/tree has keyboard focus, an inactive does not.
    #[serde(rename = "list.focusOutline")]
    pub focus_outline: Option<String>,
    /// List/Tree outline color for the focused item when the list/tree is active and selected. An active list/tree has keyboard focus, an inactive does not.
    #[serde(rename = "list.focusAndSelectionOutline")]
    pub focus_and_selection_outline: Option<String>,
    /// List/Tree background color for the selected item when the list/tree is active. An active list/tree has keyboard focus, an inactive does not.
    #[serde(rename = "list.activeSelectionBackground")]
    pub active_selection_background: Option<String>,
    /// List/Tree foreground color for the selected item when the list/tree is active. An active list/tree has keyboard focus, an inactive does not.
    #[serde(rename = "list.activeSelectionForeground")]
    pub active_selection_foreground: Option<String>,
    /// List/Tree icon foreground color for the selected item when the list/tree is active. An active list/tree has keyboard focus, an inactive does not.
    #[serde(rename = "list.activeSelectionIconForeground")]
    pub active_selection_icon_foreground: Option<String>,
    /// List/Tree background color for the selected item when the list/tree is inactive. An active list/tree has keyboard focus, an inactive does not.
    #[serde(rename = "list.inactiveSelectionBackground")]
    pub inactive_selection_background: Option<String>,
    /// List/Tree foreground color for the selected item when the list/tree is inactive. An active list/tree has keyboard focus, an inactive does not.
    #[serde(rename = "list.inactiveSelectionForeground")]
    pub inactive_selection_foreground: Option<String>,
    /// List/Tree icon foreground color for the selected item when the list/tree is inactive. An active list/tree has keyboard focus, an inactive does not.
    #[serde(rename = "list.inactiveSelectionIconForeground")]
    pub inactive_selection_icon_foreground: Option<String>,
    /// List/Tree background color for the focused item when the list/tree is inactive. An active list/tree has keyboard focus, an inactive does not.
    #[serde(rename = "list.inactiveFocusBackground")]
    pub inactive_focus_background: Option<String>,
    /// List/Tree outline color for the focused item when the list/tree is inactive. An active list/tree has keyboard focus, an inactive does not.
    #[serde(rename = "list.inactiveFocusOutline")]
    pub inactive_focus_outline: Option<String>,
    /// List/Tree background when hovering over items using the mouse.
    #[serde(rename = "list.hoverBackground")]
    pub hover_background: Option<String>,
    /// List/Tree foreground when hovering over items using the mouse.
    #[serde(rename = "list.hoverForeground")]
    pub hover_foreground: Option<String>,
    /// List/Tree drag and drop background when moving items over other items when using the mouse.
    #[serde(rename = "list.dropBackground")]
    pub drop_background: Option<String>,
    /// List/Tree drag and drop border color when moving items between items when using the mouse.
    #[serde(rename = "list.dropBetweenBackground")]
    pub drop_between_background: Option<String>,
    /// List/Tree foreground color of the match highlights when searching inside the list/tree.
    #[serde(rename = "list.highlightForeground")]
    pub highlight_foreground: Option<String>,
    /// List/Tree foreground color of the match highlights on actively focused items when searching inside the list/tree.
    #[serde(rename = "list.focusHighlightForeground")]
    pub focus_highlight_foreground: Option<String>,
    /// List/Tree foreground color for invalid items, for example an unresolved root in explorer.
    #[serde(rename = "list.invalidItemForeground")]
    pub invalid_item_foreground: Option<String>,
    /// Foreground color of list items containing errors.
    #[serde(rename = "list.errorForeground")]
    pub error_foreground: Option<String>,
    /// Foreground color of list items containing warnings.
    #[serde(rename = "list.warningForeground")]
    pub warning_foreground: Option<String>,
    /// Background color of the filtered match.
    #[serde(rename = "list.filterMatchBackground")]
    pub filter_match_background: Option<String>,
    /// Border color of the filtered match.
    #[serde(rename = "list.filterMatchBorder")]
    pub filter_match_border: Option<String>,
    /// List/Tree foreground color for items that are deemphasized.
    #[serde(rename = "list.deemphasizedForeground")]
    pub deemphasized_foreground: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListFilterWidgetColors {
    /// Background color of the type filter widget in lists and trees.
    #[serde(rename = "listFilterWidget.background")]
    pub background: Option<String>,
    /// Outline color of the type filter widget in lists and trees.
    #[serde(rename = "listFilterWidget.outline")]
    pub outline: Option<String>,
    /// Outline color of the type filter widget in lists and trees, when there are no matches.
    #[serde(rename = "listFilterWidget.noMatchesOutline")]
    pub no_matches_outline: Option<String>,
    /// Shadow color of the type filter widget in lists and trees.
    #[serde(rename = "listFilterWidget.shadow")]
    pub shadow: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TreeColors {
    /// Tree stroke color for the indentation guides.
    #[serde(rename = "tree.indentGuidesStroke")]
    pub indent_guides_stroke: Option<String>,
    /// Tree stroke color for the indentation guides that are not active.
    #[serde(rename = "tree.inactiveIndentGuidesStroke")]
    pub inactive_indent_guides_stroke: Option<String>,
    /// Table border color between columns.
    #[serde(rename = "tree.tableColumnsBorder")]
    pub table_columns_border: Option<String>,
    /// Background color for odd table rows.
    #[serde(rename = "tree.tableOddRowsBackground")]
    pub table_odd_rows_background: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckboxColors {
    /// Background color of checkbox widget.
    #[serde(rename = "checkbox.background")]
    pub background: Option<String>,
    /// Background color of checkbox widget when the element it's in is selected.
    #[serde(rename = "checkbox.selectBackground")]
    pub select_background: Option<String>,
    /// Foreground color of checkbox widget.
    #[serde(rename = "checkbox.foreground")]
    pub foreground: Option<String>,
    /// Border color of checkbox widget.
    #[serde(rename = "checkbox.border")]
    pub border: Option<String>,
    /// Border color of checkbox widget when the element it's in is selected.
    #[serde(rename = "checkbox.selectBorder")]
    pub select_border: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuickInputListColors {
    /// Quick picker foreground color for the focused item.
    #[serde(rename = "quickInputList.focusForeground")]
    pub focus_foreground: Option<String>,
    /// Quick picker icon foreground color for the focused item.
    #[serde(rename = "quickInputList.focusIconForeground")]
    pub focus_icon_foreground: Option<String>,
    /// Quick picker background color for the focused item.
    #[serde(rename = "quickInputList.focusBackground")]
    pub focus_background: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MenuColors {
    /// Border color of menus.
    #[serde(rename = "menu.border")]
    pub border: Option<String>,
    /// Foreground color of menu items.
    #[serde(rename = "menu.foreground")]
    pub foreground: Option<String>,
    /// Background color of menu items.
    #[serde(rename = "menu.background")]
    pub background: Option<String>,
    /// Foreground color of the selected menu item in menus.
    #[serde(rename = "menu.selectionForeground")]
    pub selection_foreground: Option<String>,
    /// Background color of the selected menu item in menus.
    #[serde(rename = "menu.selectionBackground")]
    pub selection_background: Option<String>,
    /// Border color of the selected menu item in menus.
    #[serde(rename = "menu.selectionBorder")]
    pub selection_border: Option<String>,
    /// Color of a separator menu item in menus.
    #[serde(rename = "menu.separatorBackground")]
    pub separator_background: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolbarColors {
    /// Toolbar background when hovering over actions using the mouse
    #[serde(rename = "toolbar.hoverBackground")]
    pub hover_background: Option<String>,
    /// Toolbar outline when hovering over actions using the mouse
    #[serde(rename = "toolbar.hoverOutline")]
    pub hover_outline: Option<String>,
    /// Toolbar background when holding the mouse over actions
    #[serde(rename = "toolbar.activeBackground")]
    pub active_background: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BreadcrumbColors {
    /// Color of focused breadcrumb items.
    #[serde(rename = "breadcrumb.foreground")]
    pub foreground: Option<String>,
    /// Background color of breadcrumb items.
    #[serde(rename = "breadcrumb.background")]
    pub background: Option<String>,
    /// Color of focused breadcrumb items.
    #[serde(rename = "breadcrumb.focusForeground")]
    pub focus_foreground: Option<String>,
    /// Color of selected breadcrumb items.
    #[serde(rename = "breadcrumb.activeSelectionForeground")]
    pub active_selection_foreground: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BreadcrumbPickerColors {
    /// Background color of breadcrumb item picker.
    #[serde(rename = "breadcrumbPicker.background")]
    pub background: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MergeColors {
    /// Current header background in inline merge-conflicts. The color must not be opaque so as not to hide underlying decorations.
    #[serde(rename = "merge.currentHeaderBackground")]
    pub current_header_background: Option<String>,
    /// Current content background in inline merge-conflicts. The color must not be opaque so as not to hide underlying decorations.
    #[serde(rename = "merge.currentContentBackground")]
    pub current_content_background: Option<String>,
    /// Incoming header background in inline merge-conflicts. The color must not be opaque so as not to hide underlying decorations.
    #[serde(rename = "merge.incomingHeaderBackground")]
    pub incoming_header_background: Option<String>,
    /// Incoming content background in inline merge-conflicts. The color must not be opaque so as not to hide underlying decorations.
    #[serde(rename = "merge.incomingContentBackground")]
    pub incoming_content_background: Option<String>,
    /// Common ancestor header background in inline merge-conflicts. The color must not be opaque so as not to hide underlying decorations.
    #[serde(rename = "merge.commonHeaderBackground")]
    pub common_header_background: Option<String>,
    /// Common ancestor content background in inline merge-conflicts. The color must not be opaque so as not to hide underlying decorations.
    #[serde(rename = "merge.commonContentBackground")]
    pub common_content_background: Option<String>,
    /// Border color on headers and the splitter in inline merge-conflicts.
    #[serde(rename = "merge.border")]
    pub border: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EditorOverviewRulerColors {
    /// Current overview ruler foreground for inline merge-conflicts.
    #[serde(rename = "editorOverviewRuler.currentContentForeground")]
    pub current_content_foreground: Option<String>,
    /// Incoming overview ruler foreground for inline merge-conflicts.
    #[serde(rename = "editorOverviewRuler.incomingContentForeground")]
    pub incoming_content_foreground: Option<String>,
    /// Common ancestor overview ruler foreground for inline merge-conflicts.
    #[serde(rename = "editorOverviewRuler.commonContentForeground")]
    pub common_content_foreground: Option<String>,
    /// Overview ruler marker color for find matches. The color must not be opaque so as not to hide underlying decorations.
    #[serde(rename = "editorOverviewRuler.findMatchForeground")]
    pub find_match_foreground: Option<String>,
    /// Overview ruler marker color for selection highlights. The color must not be opaque so as not to hide underlying decorations.
    #[serde(rename = "editorOverviewRuler.selectionHighlightForeground")]
    pub selection_highlight_foreground: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MinimapColors {
    /// Minimap marker color for find matches.
    #[serde(rename = "minimap.findMatchHighlight")]
    pub find_match_highlight: Option<String>,
    /// Minimap marker color for repeating editor selections.
    #[serde(rename = "minimap.selectionOccurrenceHighlight")]
    pub selection_occurrence_highlight: Option<String>,
    /// Minimap marker color for the editor selection.
    #[serde(rename = "minimap.selectionHighlight")]
    pub selection_highlight: Option<String>,
    /// Minimap marker color for infos.
    #[serde(rename = "minimap.infoHighlight")]
    pub info_highlight: Option<String>,
    /// Minimap marker color for warnings.
    #[serde(rename = "minimap.warningHighlight")]
    pub warning_highlight: Option<String>,
    /// Minimap marker color for errors.
    #[serde(rename = "minimap.errorHighlight")]
    pub error_highlight: Option<String>,
    /// Minimap background color.
    #[serde(rename = "minimap.background")]
    pub background: Option<String>,
    /// Opacity of foreground elements rendered in the minimap. For example, "#000000c0" will render the elements with 75% opacity.
    #[serde(rename = "minimap.foregroundOpacity")]
    pub foreground_opacity: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MinimapSliderColors {
    /// Minimap slider background color.
    #[serde(rename = "minimapSlider.background")]
    pub background: Option<String>,
    /// Minimap slider background color when hovering.
    #[serde(rename = "minimapSlider.hoverBackground")]
    pub hover_background: Option<String>,
    /// Minimap slider background color when clicked on.
    #[serde(rename = "minimapSlider.activeBackground")]
    pub active_background: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProblemsErrorIconColors {
    /// The color used for the problems error icon.
    #[serde(rename = "problemsErrorIcon.foreground")]
    pub foreground: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProblemsWarningIconColors {
    /// The color used for the problems warning icon.
    #[serde(rename = "problemsWarningIcon.foreground")]
    pub foreground: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProblemsInfoIconColors {
    /// The color used for the problems info icon.
    #[serde(rename = "problemsInfoIcon.foreground")]
    pub foreground: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChartsColors {
    /// The foreground color used in charts.
    #[serde(rename = "charts.foreground")]
    pub foreground: Option<String>,
    /// The color used for horizontal lines in charts.
    #[serde(rename = "charts.lines")]
    pub lines: Option<String>,
    /// The red color used in chart visualizations.
    #[serde(rename = "charts.red")]
    pub red: Option<String>,
    /// The blue color used in chart visualizations.
    #[serde(rename = "charts.blue")]
    pub blue: Option<String>,
    /// The yellow color used in chart visualizations.
    #[serde(rename = "charts.yellow")]
    pub yellow: Option<String>,
    /// The orange color used in chart visualizations.
    #[serde(rename = "charts.orange")]
    pub orange: Option<String>,
    /// The green color used in chart visualizations.
    #[serde(rename = "charts.green")]
    pub green: Option<String>,
    /// The purple color used in chart visualizations.
    #[serde(rename = "charts.purple")]
    pub purple: Option<String>,
}
