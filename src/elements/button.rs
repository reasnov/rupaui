use crate::utils::{Style, StyleModifier, generate_id, Accessibility, Role, Signal, Variant, Attributes, EventListeners};
use crate::Component;
use crate::container::Children;
use std::sync::Arc;

#[derive(Clone, Debug, PartialEq, Default)]
pub enum ButtonSize { Sm, #[default] Md, Lg }

pub struct Button {
    pub id: String,
    pub label: String,
    pub variant: Variant,
    pub size: ButtonSize,
    pub disabled: Signal<bool>,
    pub is_loading: Signal<bool>,
    pub style: Style,
    pub attributes: Attributes,
    pub events: EventListeners,
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
            events: EventListeners::default(),
            accessibility: Accessibility { role: Role::Button, ..Default::default() },
        }
    }

    /// Event listener for click interaction.
    pub fn on_click(mut self, f: impl Fn() + Send + Sync + 'static) -> Self {
        self.events.on_click = Some(Arc::new(f));
        self
    }

    pub fn variant(mut self, v: Variant) -> Self { self.variant = v; self }
    pub fn disabled(self, signal: Signal<bool>) -> Self { let mut s = self; s.disabled = signal; s }
    pub fn style(mut self, modifier: impl StyleModifier) -> Self { modifier.apply(&mut self.style); self }
}

impl Component for Button {
    fn id(&self) -> &str { &self.id }
    fn render(&self) {
        log::debug!("Rendering Button [{}] '{}'", self.id, self.label);
    }
}
// ... ButtonGroup and CloseButton can follow same pattern ...
