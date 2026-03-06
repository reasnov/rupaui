use crate::style::utilities::style::Style;
use std::cell::RefMut;
use crate::core::component::Component;

pub trait StyleModifier {
    fn apply(&self, style: &mut Style);
}

impl<F> StyleModifier for F where F: Fn(&mut Style) {
    fn apply(&self, style: &mut Style) { self(style); }
}

// Tuple support for composition (DRY implementation via macro would be better, but we keep it explicit for now)
impl<A: StyleModifier, B: StyleModifier> StyleModifier for (A, B) {
    fn apply(&self, style: &mut Style) { self.0.apply(style); self.1.apply(style); }
}
impl<A: StyleModifier, B: StyleModifier, C: StyleModifier> StyleModifier for (A, B, C) {
    fn apply(&self, style: &mut Style) { self.0.apply(style); self.1.apply(style); self.2.apply(style); }
}
impl<A: StyleModifier, B: StyleModifier, C: StyleModifier, D: StyleModifier> StyleModifier for (A, B, C, D) {
    fn apply(&self, style: &mut Style) { self.0.apply(style); self.1.apply(style); self.2.apply(style); self.3.apply(style); }
}

/// The core trait that enables utility-first chaining for any component.
pub trait Stylable: Component + Sized {
    fn style(self, modifier: impl StyleModifier) -> Self {
        {
            let mut style = self.get_style_mut();
            modifier.apply(&mut *style);
        }
        self.mark_dirty();
        self
    }

    /// internal helper to get mutable access to style.
    fn get_style_mut(&self) -> RefMut<'_, Style>;
}
