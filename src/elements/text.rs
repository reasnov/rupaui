use crate::utils::{Style, StyleModifier, generate_id, Theme};
use crate::Component;

/// A semantic component for displaying text.
pub struct Text {
    pub id: String,
    pub content: String,
    pub style: Style,
}

impl Text {
    pub fn new(content: impl Into<String>) -> Self {
        let mut style = Style::default();
        Theme::current().apply_defaults(&mut style);

        Self {
            id: generate_id(),
            content: content.into(),
            style,
        }
    }

    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.id = id.into();
        self
    }

    pub fn style(mut self, modifier: impl StyleModifier) -> Self {
        modifier.apply(&mut self.style);
        self
    }
}

impl Component for Text {
    fn id(&self) -> &str { &self.id }
    fn render(&self) {
        log::debug!("Rendering Text [ID: {}]: '{}'", self.id, self.content);
    }
}
