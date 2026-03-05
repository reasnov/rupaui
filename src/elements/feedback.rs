use crate::utils::{Style, StyleModifier, generate_id, Accessibility, Role, Signal, Variant, Theme};
use crate::Component;
use crate::container::Children;

pub struct Alert { pub id: String, pub variant: Variant, pub is_visible: Signal<bool>, pub style: Style, pub children: Children }
impl Alert {
    pub fn new() -> Self {
        let mut style = Style::default();
        Theme::current().apply_defaults(&mut style);
        style.background.color = Some(Theme::variant(Variant::Info));
        Self { id: generate_id(), variant: Variant::Info, is_visible: Signal::new(true), style, children: Children::new() }
    }
    pub fn variant(mut self, v: Variant) -> Self { self.variant = v; self.style.background.color = Some(Theme::variant(v)); self }
    pub fn child(mut self, child: Box<dyn Component>) -> Self { self.children.add(child); self }
}
impl Component for Alert { fn id(&self) -> &str { &self.id } fn render(&self) { if self.is_visible.get() { self.children.render_all(); } } }

pub struct Badge { pub id: String, pub label: String, pub variant: Variant, pub style: Style }
impl Badge {
    pub fn new(label: impl Into<String>) -> Self {
        let mut style = Style::default();
        Theme::current().apply_defaults(&mut style);
        style.background.color = Some(Theme::variant(Variant::Secondary));
        Self { id: generate_id(), label: label.into(), variant: Variant::Secondary, style }
    }
}
impl Component for Badge { fn id(&self) -> &str { &self.id } fn render(&self) { log::debug!("Badge: {}", self.label); } }

pub struct Progress { pub id: String, pub value: Signal<f32>, pub style: Style }
impl Progress {
    pub fn new(val: Signal<f32>) -> Self {
        let mut style = Style::default();
        Theme::current().apply_defaults(&mut style);
        Self { id: generate_id(), value: val, style }
    }
}
impl Component for Progress { fn id(&self) -> &str { &self.id } fn render(&self) { log::debug!("Progress: {}%", self.value.get()); } }

pub struct Spinner { pub id: String, pub style: Style }
impl Spinner { pub fn new() -> Self { let mut style = Style::default(); Theme::current().apply_defaults(&mut style); Self { id: generate_id(), style } } }
impl Component for Spinner { fn id(&self) -> &str { &self.id } fn render(&self) { log::debug!("Spinner spinning..."); } }

pub struct Placeholder { pub id: String, pub style: Style }
impl Placeholder { pub fn new() -> Self { let mut style = Style::default(); Theme::current().apply_defaults(&mut style); Self { id: generate_id(), style } } }
impl Component for Placeholder { fn id(&self) -> &str { &self.id } fn render(&self) { log::debug!("Placeholder loading..."); } }
