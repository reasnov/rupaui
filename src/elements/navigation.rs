use crate::utils::{Style, generate_id, StyleModifier, Theme, Color, Accessibility, Attributes, Signal};
use crate::Component;
use crate::container::Children;
use crate::renderer::Renderer;
use taffy::prelude::*;

pub struct Navbar {
    pub id: String, pub style: Style, pub accessibility: Accessibility, pub attributes: Attributes,
    pub start: Children, pub center: Children, pub end: Children,
}

impl Navbar {
    pub fn new() -> Self {
        let mut style = Style::default(); Theme::current().apply_defaults(&mut style);
        style.layout.display = crate::utils::Display::Flex;
        style.flex.flex_direction = crate::utils::FlexDirection::Row;
        style.padding = crate::utils::spacing::Spacing::all(16.0);
        style.background.color = Some(Color::Semantic("surface".into(), None));
        Self { id: generate_id(), style, accessibility: Accessibility { role: crate::utils::Role::Navigation, ..Default::default() }, attributes: Attributes::default(), start: Children::new(), center: Children::new(), end: Children::new() }
    }
    pub fn start(mut self, c: Box<dyn Component>) -> Self { self.start.add(c); self }
    pub fn center(mut self, c: Box<dyn Component>) -> Self { self.center.add(c); self }
    pub fn end(mut self, c: Box<dyn Component>) -> Self { self.end.add(c); self }
}

impl Component for Navbar {
    fn id(&self) -> &str { &self.id }
    fn layout(&self, taffy: &mut TaffyTree<()>, parent: Option<NodeId>) -> NodeId {
        let mut style = self.style.to_taffy(); style.justify_content = Some(JustifyContent::SpaceBetween);
        let node = taffy.new_with_children(style, &[]).unwrap();
        if let Some(p) = parent { taffy.add_child(p, node).unwrap(); }
        self.start.layout_all(taffy, node); self.center.layout_all(taffy, node); self.end.layout_all(taffy, node);
        node
    }
    fn paint(&self, renderer: &mut Renderer, taffy: &TaffyTree<()>, node: NodeId, is_group_hovered: bool, render_pass: &mut wgpu::RenderPass<'_>) {
        let layout = taffy.layout(node).unwrap();
        if let Some(color) = self.style.background.color.clone() { renderer.draw_rect(layout.location.x, layout.location.y, layout.size.width, layout.size.height, color.to_rgba(), 0.0); }
        self.start.paint_all(renderer, taffy, node, is_group_hovered, render_pass);
        self.center.paint_all(renderer, taffy, node, is_group_hovered, render_pass);
        self.end.paint_all(renderer, taffy, node, is_group_hovered, render_pass);
    }
    fn on_click(&self) {}
    fn on_scroll(&self, _d: f32) {}
    fn on_drag(&self, _d: crate::utils::Vec2) {}
}

pub struct Tab { pub title: String, pub content: Box<dyn Component> }

pub struct Tabs {
    pub id: String,
    pub tabs: Vec<Tab>,
    pub active_index: Signal<usize>,
}

impl Tabs {
    pub fn new(tabs: Vec<Tab>, active: Signal<usize>) -> Self {
        Self { id: generate_id(), tabs, active_index: active }
    }
}

impl Component for Tabs {
    fn id(&self) -> &str { &self.id }
    fn layout(&self, taffy: &mut TaffyTree<()>, parent: Option<NodeId>) -> NodeId {
        let node = taffy.new_with_children(taffy::style::Style::default(), &[]).unwrap();
        if let Some(p) = parent { taffy.add_child(p, node).unwrap(); }
        if let Some(tab) = self.tabs.get(self.active_index.get()) {
            tab.content.layout(taffy, Some(node));
        }
        node
    }
    fn paint(&self, renderer: &mut Renderer, taffy: &TaffyTree<()>, node: NodeId, is_group_hovered: bool, render_pass: &mut wgpu::RenderPass<'_>) {
        if let Some(tab) = self.tabs.get(self.active_index.get()) {
            let children = taffy.children(node).unwrap();
            if let Some(c_node) = children.get(0) {
                tab.content.paint(renderer, taffy, *c_node, is_group_hovered, render_pass);
            }
        }
    }
    fn on_click(&self) {}
    fn on_scroll(&self, _d: f32) {}
    fn on_drag(&self, _d: crate::utils::Vec2) {}
}
