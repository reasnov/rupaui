use std::collections::HashMap;
use super::spacing::{Spacing, IntoSpacing};
use super::border::{Border, Rounding, Outline, BorderStyle};
use super::background::{Background, BackgroundAttachment, BackgroundClip, BackgroundOrigin, BackgroundRepeat, BackgroundSize};
use super::color::Color;
pub use super::layout::*;
pub use super::flex::*;
pub use super::grid::*;
pub use super::sizing::*;
pub use super::typography::*;
pub use super::effects::*;
pub use super::filters::*;
pub use super::table::*;
pub use super::animation::*;
pub use super::transform::*;
pub use super::interactivity::*;
pub use super::svg::*;
pub use super::accessibility::*;
use super::modifiers::StyleModifier;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Style {
    pub layout: Layout,
    pub flex: Flex,
    pub grid: Grid,
    pub sizing: Sizing,
    pub typography: Typography,
    pub background: Background,
    pub border: Border,
    pub outline: Outline,
    pub rounding: Rounding,
    pub margin: Spacing,
    pub padding: Spacing,
    pub effects: Effects,
    pub filters: FilterStack,
    pub table: Table,
    pub motion: Motion,
    pub transform: Transform,
    pub interactivity: Interactivity,
    pub svg: Svg,
    pub accessibility: Accessibility,

    // --- Extensibility: User-defined properties ---
    pub custom: HashMap<String, String>,

    pub hover: Option<Box<Style>>,
    pub focus: Option<Box<Style>>,
    pub active: Option<Box<Style>>,
    pub disabled: Option<Box<Style>>,
}

impl Style {
    pub fn new() -> Self { Self::default() }

    /// Set a custom style property (Extensibility).
    pub fn set_custom(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.custom.insert(key.into(), value.into());
        self
    }

    // --- Shorthands ---
    pub fn bg(mut self, color: impl Into<Color>) -> Self { self.background.color = Some(color.into()); self }
    pub fn color(mut self, color: impl Into<Color>) -> Self { self.typography.color = Some(color.into()); self }
    pub fn p(mut self, val: impl IntoSpacing) -> Self { self.padding = val.into_spacing(); self }
    pub fn m(mut self, val: impl IntoSpacing) -> Self { self.margin = val.into_spacing(); self }
    pub fn w(mut self, val: f32) -> Self { self.sizing.width = Some(val); self }
    pub fn h(mut self, val: f32) -> Self { self.sizing.height = Some(val); self }
    pub fn rounded(mut self, val: f32) -> Self { self.rounding = Rounding::all(val); self }
    
    // Interaction Variants
    pub fn hover(mut self, modifier: impl StyleModifier) -> Self {
        let mut s = self.hover.take().map(|b| *b).unwrap_or_default();
        modifier.apply(&mut s);
        self.hover = Some(Box::new(s));
        self
    }
}
