use crate::utils::{Style, StyleModifier, generate_id, Theme, Color, Attributes, Accessibility};
use crate::Component;
use crate::renderer::Renderer;
use taffy::prelude::*;

/// A semantic component for displaying text.
pub struct Text {
    pub id: String,
    pub content: String,
    pub style: Style,
    pub attributes: Attributes,
    pub accessibility: Accessibility,
}

impl Text {
    pub fn new(content: impl Into<String>) -> Self {
        let mut style = Style::default();
        Theme::current().apply_defaults(&mut style);

        Self {
            id: generate_id(),
            content: content.into(),
            style,
            attributes: Attributes::default(),
            accessibility: Accessibility::default(),
        }
    }

    pub fn id(mut self, id: impl Into<String>) -> Self { self.id = id.into(); self }
    pub fn a11y(mut self, acc: Accessibility) -> Self { self.accessibility = acc; self }
    pub fn attr(mut self, key: impl Into<String>, value: impl Into<String>) -> Self { self.attributes.set(key, value); self }
    pub fn get_attr(&self, key: &str) -> Option<&String> { self.attributes.get(key) }

    pub fn style(mut self, modifier: impl StyleModifier) -> Self {
        modifier.apply(&mut self.style);
        self
    }
}

impl Component for Text {
    fn id(&self) -> &str { &self.id }

    fn layout(&self, taffy: &mut TaffyTree<()>, parent: Option<NodeId>) -> NodeId {
        let node = taffy.new_leaf(self.style.to_taffy()).unwrap();
        if let Some(p) = parent { taffy.add_child(p, node).unwrap(); }
        node
    }

    fn paint(&self, renderer: &mut Renderer, taffy: &TaffyTree<()>, node: NodeId, is_group_hovered: bool) {
        let layout = taffy.layout(node).unwrap();
        let style = if is_group_hovered && self.style.group_hover.is_some() { self.style.group_hover.as_ref().unwrap() } else { &self.style };
        
        let color = style.typography.color.clone()
            .unwrap_or(Color::White(1.0))
            .to_rgba();
        
        let size = style.typography.base_size();

        renderer.draw_text(&self.content, layout.location.x, layout.location.y, size, color);
    }

    fn on_click(&self) {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_text_creation() {
        let txt = Text::new("Artisan");
        assert_eq!(txt.content, "Artisan");
    }

    #[test]
    fn test_text_attr() {
        let txt = Text::new("Hi").attr("data-role", "label");
        assert_eq!(txt.get_attr("data-role").unwrap(), "label");
    }
}
