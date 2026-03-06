pub mod id;
pub mod vector;
pub mod ui_helpers;
pub mod state;

pub use id::*;
pub use vector::*;
pub use ui_helpers::*;
pub use state::*;

// Note: Visual types and modifiers are now sourced from crate::style::utilities and crate::style::modifiers
// following the 9-layer architecture. 
// Standard re-exports for internal framework use:

pub use crate::style::utilities::style::Style;
pub use crate::style::modifiers::base::{StyleModifier, Stylable};
pub use crate::style::modifiers::theme::{Theme, Variant, ColorMode};
pub use crate::style::utilities::color::Color;
pub use crate::style::utilities::attributes::Attributes;
pub use crate::style::utilities::accessibility::{Accessibility, Role};
pub use crate::style::utilities::layout::{Display, Position, Overflow};
pub use crate::style::utilities::flex::{FlexDirection, AlignItems, JustifyContent};
pub use crate::style::utilities::spacing::Spacing;
pub use crate::style::utilities::border::Rounding;
pub use crate::style::utilities::typography::TextAlign;
pub use crate::style::utilities::scale::Scale;
pub use crate::platform::events::EventListeners;
