use std::collections::HashMap;
use taffy::prelude::*;
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
    pub custom: HashMap<String, String>,
    pub hover: Option<Box<Style>>,
    pub focus: Option<Box<Style>>,
    pub active: Option<Box<Style>>,
    pub disabled: Option<Box<Style>>,
    pub group_hover: Option<Box<Style>>,
    pub is_group: bool,
}

impl Style {
    pub fn new() -> Self { Self::default() }

    pub fn to_taffy(&self) -> taffy::style::Style {
        let mut t = taffy::style::Style::default();
        
        // 1. Display & Position
        t.display = match self.layout.display {
            Display::Flex => taffy::style::Display::Flex,
            Display::Grid => taffy::style::Display::Grid,
            Display::None => taffy::style::Display::None,
            _ => taffy::style::Display::Block,
        };
        t.position = match self.layout.position {
            Position::Relative => taffy::style::Position::Relative,
            Position::Absolute => taffy::style::Position::Absolute,
            _ => taffy::style::Position::Relative,
        };

        // 2. Flexbox
        t.flex_direction = match self.flex.flex_direction {
            FlexDirection::Row => taffy::style::FlexDirection::Row,
            FlexDirection::Col => taffy::style::FlexDirection::Column,
            FlexDirection::RowReverse => taffy::style::FlexDirection::RowReverse,
            FlexDirection::ColReverse => taffy::style::FlexDirection::ColumnReverse,
        };
        t.flex_wrap = match self.flex.flex_wrap {
            FlexWrap::Wrap => taffy::style::FlexWrap::Wrap,
            FlexWrap::WrapReverse => taffy::style::FlexWrap::WrapReverse,
            _ => taffy::style::FlexWrap::NoWrap,
        };
        t.flex_grow = self.flex.flex_grow;
        t.flex_shrink = self.flex.flex_shrink;
        if let Some(basis) = self.flex.flex_basis { t.flex_basis = length(basis); }
        
        if let Some(ref align) = self.flex.align_items {
            t.align_items = Some(match align {
                AlignItems::Center => taffy::style::AlignItems::Center,
                AlignItems::FlexStart => taffy::style::AlignItems::FlexStart,
                AlignItems::FlexEnd => taffy::style::AlignItems::FlexEnd,
                AlignItems::Stretch => taffy::style::AlignItems::Stretch,
                AlignItems::Baseline => taffy::style::AlignItems::Baseline,
            });
        }

        if let Some(ref justify) = self.flex.justify_content {
            t.justify_content = Some(match justify {
                JustifyContent::Center => taffy::style::JustifyContent::Center,
                JustifyContent::FlexStart => taffy::style::JustifyContent::FlexStart,
                JustifyContent::FlexEnd => taffy::style::JustifyContent::FlexEnd,
                JustifyContent::SpaceBetween => taffy::style::JustifyContent::SpaceBetween,
                JustifyContent::SpaceAround => taffy::style::JustifyContent::SpaceAround,
                JustifyContent::SpaceEvenly => taffy::style::JustifyContent::SpaceEvenly,
            });
        }

        // 3. Spacing (Padding, Margin, Gap)
        if let Some(gap) = self.flex.gap {
            t.gap = taffy::geometry::Size { width: length(gap), height: length(gap) };
        }
        t.padding = taffy::geometry::Rect {
            left: length(self.padding.left), right: length(self.padding.right),
            top: length(self.padding.top), bottom: length(self.padding.bottom),
        };
        t.margin = taffy::geometry::Rect {
            left: length(self.margin.left), right: length(self.margin.right),
            top: length(self.margin.top), bottom: length(self.margin.bottom),
        };

        // 4. Sizing
        if let Some(w) = self.sizing.width { t.size.width = length(w); }
        if let Some(h) = self.sizing.height { t.size.height = length(h); }
        if let Some(min_w) = self.sizing.min_width { t.min_size.width = length(min_w); }
        if let Some(min_h) = self.sizing.min_height { t.min_size.height = length(min_h); }
        if let Some(max_w) = self.sizing.max_width { t.max_size.width = length(max_w); }
        if let Some(max_h) = self.sizing.max_height { t.max_size.height = length(max_h); }
        t.aspect_ratio = self.sizing.aspect_ratio;

        // 5. Overflow
        t.overflow.x = match self.layout.overflow_x {
            Overflow::Hidden => taffy::style::Overflow::Hidden,
            Overflow::Scroll => taffy::style::Overflow::Scroll,
            _ => taffy::style::Overflow::Visible,
        };
        t.overflow.y = match self.layout.overflow_y {
            Overflow::Hidden => taffy::style::Overflow::Hidden,
            Overflow::Scroll => taffy::style::Overflow::Scroll,
            _ => taffy::style::Overflow::Visible,
        };

        t
    }

    pub fn bg(mut self, color: impl Into<Color>) -> Self { self.background.color = Some(color.into()); self }
}
