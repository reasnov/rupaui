use crate::utils::{Style, StyleModifier, generate_id, Attributes, Theme};
use crate::Component;
use crate::container::Children;
use crate::elements::Text;

/// A generic container for UI layout with dynamic attributes.
pub struct Div {
    pub id: String,
    pub style: Style,
    pub attributes: Attributes,
    pub children: Children,
}

impl Div {
    pub fn new() -> Self {
        let mut style = Style::default();
        Theme::current().apply_defaults(&mut style);

        Self {
            id: generate_id(),
            style,
            attributes: Attributes::new(),
            children: Children::new(),
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

    pub fn child(mut self, child: Box<dyn Component>) -> Self {
        self.children.add(child);
        self
    }

    pub fn text(mut self, content: impl Into<String>) -> Self {
        self.children.add(Box::new(Text::new(content)));
        self
    }
}

impl Component for Div {
    fn id(&self) -> &str { &self.id }
    fn render(&self) {
        log::debug!("Rendering Div [ID: {}]", self.id);
        self.children.render_all();
    }
}
