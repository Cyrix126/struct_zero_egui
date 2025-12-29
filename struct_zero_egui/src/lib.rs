#![doc = include_str!("../README.md")]

use egui::{Label, RichText, TextStyle, Ui};

#[cfg(feature = "derive")]
pub extern crate struct_zero_egui_derive;

/// This trait can only be implemented for structs with String fields only
///# Example
///```rust,no_run
///
///use struct_zero_egui::EguiDisplay;
///
///```
pub trait EguiDisplay {
    /// Display the struct
    fn show(&self, ui: &mut Ui) {
        if let Some(title) = self.title() {
            let mut widget_title =
                ui.label(RichText::new(title.value).text_style(TextStyle::Heading));
            if let Some(hover_enabled) = title.hover_enabled {
                widget_title = widget_title.on_hover_text(hover_enabled);
            }
            if let Some(hover_disabled) = title.hover_disabled {
                widget_title = widget_title.on_disabled_hover_text(hover_disabled);
            }
            let _ = widget_title;
        }

        for FieldParams {
            name,
            value,
            hover,
            spacing,
        } in self.fields()
        {
            let mut widget_name = ui.label(RichText::new(name).underline());
            if let Some(hover_text) = hover {
                widget_name = widget_name.on_hover_text(hover_text);
            }
            let _ = widget_name;
            ui.label(value);
            if let Some(space) = spacing {
                ui.add_sized(
                    [
                        0.0,
                        (ui.text_style_height(&TextStyle::Body) + ui.spacing().item_spacing.y)
                            * space,
                    ],
                    Label::new(value),
                );
            } else {
                ui.label(value);
            }
        }
    }
    fn title(&'_ self) -> Option<TitleParams<'_>>;
    fn fields(&self) -> impl Iterator<Item = FieldParams<'_>>;
}

pub struct TitleParams<'a> {
    pub value: &'a str,
    pub hover_enabled: Option<&'a str>,
    pub hover_disabled: Option<&'a str>,
}
pub struct FieldParams<'a> {
    pub name: &'a str,
    pub value: &'a str,
    pub hover: Option<&'a str>,
    pub spacing: Option<f32>,
}
