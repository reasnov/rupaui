use crate::utils::{Style, StyleModifier, generate_id, Accessibility, Role, Signal, Variant, Attributes, EventListeners, Theme, Color};
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
        let theme = Theme::current();
        
        let mut style = Style::default();
        style.border.width = theme.borders.width;
        style.border.style = theme.borders.style.clone();
        style.rounding = crate::utils::border::Rounding::all(theme.borders.radius);
        style.typography.size = Some(theme.typography.base_size);
        
        // Fixed: Color does not implement Default, so we provide a specific fallback
        let primary_color = theme.variants.get(&Variant::Primary).cloned().unwrap_or(Color::Semantic("primary".into(), None));
        style.background.color = Some(primary_color);

        Self {
            id: generate_id(),
            label: label.into(),
            variant: Variant::Primary,
            size: ButtonSize::Md,
            disabled: Signal::new(false),
            is_loading: Signal::new(false),
            style,
            attributes: Attributes::new(),
            events: EventListeners::default(),
            accessibility: Accessibility { role: Role::Button, ..Default::default() },
        }
    }

    pub fn variant(mut self, v: Variant) -> Self {
        self.variant = v;
        self.style.background.color = Some(Theme::variant(v));
        self
    }

    pub fn size(mut self, s: ButtonSize) -> Self { self.size = s; self }
    pub fn disabled(self, signal: Signal<bool>) -> Self { let mut s = self; s.disabled = signal; s }
    pub fn loading(self, signal: Signal<bool>) -> Self { let mut s = self; s.is_loading = signal; s }

    pub fn on_click(mut self, f: impl Fn() + Send + Sync + 'static) -> Self {
        self.events.on_click = Some(Arc::new(f));
        self
    }

    pub fn style(mut self, modifier: impl StyleModifier) -> Self {
        modifier.apply(&mut self.style);
        self
    }

    pub fn attr(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.attributes.set(key, value);
        self
    }
}

impl Component for Button {
    fn id(&self) -> &str { &self.id }
    fn render(&self) {
        log::debug!("Rendering Button [{}] '{}' variant={:?}", self.id, self.label, self.variant);
    }
}

pub struct CloseButton {
    pub id: String,
    pub disabled: Signal<bool>,
    pub style: Style,
    pub attributes: Attributes,
}

impl CloseButton {
    pub fn new() -> Self {
        Self {
            id: generate_id(),
            disabled: Signal::new(false),
            style: Style::default(),
            attributes: Attributes::new(),
        }
    }
    pub fn disabled(self, signal: Signal<bool>) -> Self { let mut s = self; s.disabled = signal; s }
    pub fn attr(mut self, key: impl Into<String>, value: impl Into<String>) -> Self { self.attributes.set(key, value); self }
    pub fn style(mut self, modifier: impl StyleModifier) -> Self { modifier.apply(&mut self.style); self }
}

impl Component for CloseButton {
    fn id(&self) -> &str { &self.id }
    fn render(&self) {
        log::debug!("Rendering CloseButton [{}]", self.id);
    }
}

pub struct ButtonGroup {
    pub id: String,
    pub style: Style,
    pub attributes: Attributes,
    pub children: Children,
}

impl ButtonGroup {
    pub fn new() -> Self {
        let mut style = Style::default();
        style.layout.display = crate::utils::Display::Flex;
        Self {
            id: generate_id(),
            style,
            attributes: Attributes::new(),
            children: Children::new(),
        }
    }

    pub fn child(mut self, button: Button) -> Self {
        self.children.add(Box::new(button));
        self
    }
    pub fn attr(mut self, key: impl Into<String>, value: impl Into<String>) -> Self { self.attributes.set(key, value); self }
    pub fn style(mut self, modifier: impl StyleModifier) -> Self { modifier.apply(&mut self.style); self }
}

impl Component for ButtonGroup {
    fn id(&self) -> &str { &self.id }
    fn render(&self) {
        log::debug!("Rendering ButtonGroup [{}]", self.id);
        self.children.render_all();
    }
}
