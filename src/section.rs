use crate::utils::{Style, StyleModifier, generate_id, Theme};
use crate::Component;
use crate::container::Children;

/// Represents a distinct semantic part of the interface.
pub struct Section {
    pub id: String,
    pub name: String,
    pub style: Style,
    pub children: Children,
}

impl Section {
    pub fn new(name: impl Into<String>) -> Self {
        let mut style = Style::default();
        Theme::current().apply_defaults(&mut style);
        
        Self {
            id: generate_id(),
            name: name.into(),
            style,
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

    pub fn children(mut self, children: Vec<Box<dyn Component>>) -> Self {
        self.children.append(children);
        self
    }
}

impl Component for Section {
    fn id(&self) -> &str { &self.id }
    fn render(&self) {
        log::debug!("Rendering Section [ID: {}]: {}", self.id, self.name);
        self.children.render_all();
    }
}
