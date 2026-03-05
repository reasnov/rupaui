use crate::utils::{Style, StyleModifier, generate_id, Accessibility, Role, Signal, Variant, Attributes};
use crate::Component;
use crate::container::Children;

#[derive(Clone, Debug, PartialEq, Default)]
pub enum ButtonSize { Sm, #[default] Md, Lg }

/// A semantic Button component with reactive states and dynamic attributes.
pub struct Button {
    pub id: String,
    pub label: String,
    pub variant: Variant,
    pub size: ButtonSize,
    pub disabled: Signal<bool>,
    pub is_loading: Signal<bool>,
    pub style: Style,
    pub attributes: Attributes,
    pub accessibility: Accessibility,
}

impl Button {
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            id: generate_id(),
            label: label.into(),
            variant: Variant::Primary,
            size: ButtonSize::Md,
            disabled: Signal::new(false),
            is_loading: Signal::new(false),
            style: Style::default(),
            attributes: Attributes::new(),
            accessibility: Accessibility { role: Role::Button, ..Default::default() },
        }
    }

    /// Set a custom attribute on this component.
    pub fn attr(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.attributes.set(key, value);
        self
    }

    pub fn variant(mut self, v: Variant) -> Self { self.variant = v; self }
    pub fn disabled(self, signal: Signal<bool>) -> Self { let mut s = self; s.disabled = signal; s }
    pub fn style(mut self, modifier: impl StyleModifier) -> Self { modifier.apply(&mut self.style); self }
}

impl Component for Button {
    fn id(&self) -> &str { &self.id }
    fn render(&self) {
        log::debug!("Rendering Button [{}] '{}' attrs={:?}", self.id, self.label, self.attributes.map);
    }
}
// ... Rest of file (ButtonGroup, CloseButton) ...
pub struct CloseButton { pub id: String, pub disabled: Signal<bool>, pub style: Style, pub attributes: Attributes }
impl CloseButton {
    pub fn new() -> Self { Self { id: generate_id(), disabled: Signal::new(false), style: Style::default(), attributes: Attributes::new() } }
    pub fn attr(mut self, key: impl Into<String>, value: impl Into<String>) -> Self { self.attributes.set(key, value); self }
}
impl Component for CloseButton { fn id(&self) -> &str { &self.id } fn render(&self) { log::debug!("Rendering CloseButton [{}]", self.id); } }

pub struct ButtonGroup { pub id: String, pub style: Style, pub attributes: Attributes, pub children: Children }
impl ButtonGroup {
    pub fn new() -> Self { Self { id: generate_id(), style: Style::default(), attributes: Attributes::new(), children: Children::new() } }
    pub fn attr(mut self, key: impl Into<String>, value: impl Into<String>) -> Self { self.attributes.set(key, value); self }
    pub fn child(mut self, button: Button) -> Self { self.children.add(Box::new(button)); self }
}
impl Component for ButtonGroup { fn id(&self) -> &str { &self.id } fn render(&self) { self.children.render_all(); } }
