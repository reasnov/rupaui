use crate::style::utilities::style::Style;
use crate::style::utilities::spacing::Spacing;
use crate::style::utilities::scale::Scale;
use super::base::{StyleModifier, Stylable};

// --- Functional API ---

pub fn p(val: f32) -> impl StyleModifier { move |s: &mut Style| s.padding = Spacing::all(val) }
pub fn px(val: f32) -> impl StyleModifier { move |s: &mut Style| { s.padding.left = val; s.padding.right = val; } }
pub fn py(val: f32) -> impl StyleModifier { move |s: &mut Style| { s.padding.top = val; s.padding.bottom = val; } }

pub fn m(val: f32) -> impl StyleModifier { move |s: &mut Style| s.margin = Spacing::all(val) }
pub fn mx(val: f32) -> impl StyleModifier { move |s: &mut Style| { s.margin.left = val; s.margin.right = val; } }
pub fn my(val: f32) -> impl StyleModifier { move |s: &mut Style| { s.margin.top = val; s.margin.bottom = val; } }

pub fn p_scale(sc: Scale) -> impl StyleModifier { move |s: &mut Style| s.padding = Spacing::all(sc.value(16.0)) }

// --- Chaining API ---

pub trait ChainedSpacing: Stylable {
    fn p(self, val: f32) -> Self { self.style(p(val)) }
    fn px(self, val: f32) -> Self { self.style(px(val)) }
    fn py(self, val: f32) -> Self { self.style(py(val)) }
    fn m(self, val: f32) -> Self { self.style(m(val)) }
}

impl<T: Stylable> ChainedSpacing for T {}
