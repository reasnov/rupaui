use crate::utils::{Style, StyleModifier, generate_id, Accessibility, Role, Signal, Variant, Attributes, EventListeners, Theme};
use crate::Component;
use crate::container::Children;
use std::sync::Arc;

pub struct Button {
    pub id: String,
    pub label: String,
    pub variant: Variant,
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
        
        // 1. Get default visual from Theme DNA
        let mut style = Style::default();
        style.border.width = theme.borders.width;
        style.border.style = theme.borders.style.clone();
        style.rounding = crate::utils::border::Rounding::all(theme.borders.radius);
        style.typography.size = Some(theme.typography.base_size);
        
        // 2. Apply variant-specific color from Theme
        style.background.color = Some(theme.variants.get(&Variant::Primary).cloned().unwrap_or_default());

        Self {
            id: generate_id(),
            label: label.into(),
            variant: Variant::Primary,
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
        // Update background color based on new variant from theme
        self.style.background.color = Some(Theme::variant(v));
        self
    }

    pub fn on_click(mut self, f: impl Fn() + Send + Sync + 'static) -> Self {
        self.events.on_click = Some(Arc::new(f));
        self
    }

    pub fn style(mut self, modifier: impl StyleModifier) -> Self {
        modifier.apply(&mut self.style);
        self
    }
}

impl Component for Button {
    fn id(&self) -> &str { &self.id }
    fn render(&self) {
        log::debug!("Rendering Button [{}] '{}' variant={:?}", self.id, self.label, self.variant);
    }
}
