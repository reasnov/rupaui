use crate::utils::{Style, StyleModifier, generate_id, Attributes};
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
        Self {
            id: generate_id(),
            style: Style::default(),
            attributes: Attributes::new(),
            children: Children::new(),
        }
    }

    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.id = id.into();
        self
    }

    pub fn attr(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.attributes.set(key, value);
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

    pub fn children(mut self, children: Vec<Box<dyn Component>>) -> Self {
        self.children.append(children);
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
        log::debug!("Rendering Div [ID: {}] attrs={:?}", self.id, self.attributes.map);
        self.children.render_all();
    }
}
