use crate::utils::{Style, StyleModifier, generate_id, Theme, Display};
use crate::Component;
use crate::container::Children;

/// A semantic component dedicated to Grid layouts.
pub struct Grid {
    pub id: String,
    pub style: Style,
    pub children: Children,
}

impl Grid {
    pub fn new() -> Self {
        let mut style = Style::default();
        Theme::current().apply_defaults(&mut style);
        style.layout.display = Display::Grid;

        Self {
            id: generate_id(),
            style,
            children: Children::new(),
        }
    }

    pub fn style(mut self, modifier: impl StyleModifier) -> Self {
        modifier.apply(&mut self.style);
        self
    }

    pub fn child(mut self, child: Box<dyn Component>) -> Self {
        self.children.add(child);
        self
    }
}

impl Component for Grid {
    fn id(&self) -> &str { &self.id }
    fn render(&self) {
        log::debug!("Rendering Grid Component [{}]", self.id);
        self.children.render_all();
    }
}
