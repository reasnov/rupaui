use crate::Component;
use crate::renderer::Renderer;
use crate::utils::Vec2;
use taffy::prelude::*;

/// A utility to manage parent-child relationships in a standardized way.
pub struct Children {
    pub list: Vec<Box<dyn Component>>,
}

impl Children {
    pub fn new() -> Self {
        Self { list: Vec::new() }
    }

    pub fn add(&mut self, child: Box<dyn Component>) {
        self.list.push(child);
    }

    pub fn append(&mut self, mut children: Vec<Box<dyn Component>>) {
        self.list.append(&mut children);
    }

    pub fn layout_all(&self, taffy: &mut TaffyTree<()>, parent: NodeId) -> Vec<NodeId> {
        self.list.iter()
            .map(|child| child.layout(taffy, Some(parent)))
            .collect()
    }

    pub fn paint_all(&self, renderer: &mut Renderer, taffy: &TaffyTree<()>, parent_node: NodeId, is_group_hovered: bool, render_pass: &mut wgpu::RenderPass<'_>) {
        let children_nodes = taffy.children(parent_node).unwrap();
        for (i, child) in self.list.iter().enumerate() {
            if let Some(node) = children_nodes.get(i) {
                child.paint(renderer, taffy, *node, is_group_hovered, render_pass);
            }
        }
    }

    /// Recursively dispatches click events to children based on hit-testing.
    pub fn dispatch_click(&self, taffy: &TaffyTree<()>, parent_node: NodeId, cursor_pos: Vec2) -> bool {
        let children_nodes = taffy.children(parent_node).unwrap();
        // Check in reverse order (top-most children first)
        for (i, child) in self.list.iter().enumerate().rev() {
            if let Some(node) = children_nodes.get(i) {
                let layout = taffy.layout(*node).unwrap();
                let x = layout.location.x;
                let y = layout.location.y;
                let w = layout.size.width;
                let h = layout.size.height;

                if cursor_pos.x >= x && cursor_pos.x <= x + w &&
                   cursor_pos.y >= y && cursor_pos.y <= y + h {
                    
                    // Possible hit. Containers should delegate further.
                    // We'll call on_click on the child.
                    // A better way is to have on_click take cursor_pos or taffy ref,
                    // but for now, we'll let the child handle it.
                    child.on_click();
                    return true; // Hit consumed
                }
            }
        }
        false
    }

    pub fn render_all(&self) {
        for child in &self.list {
            child.render();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::elements::Text;

    #[test]
    fn test_children_append() {
        let mut children = Children::new();
        children.add(Box::new(Text::new("One")));
        children.add(Box::new(Text::new("Two")));
        assert_eq!(children.list.len(), 2);
    }
}
