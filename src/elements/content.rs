use crate::utils::{Style, generate_id, StyleModifier, Theme, Color, Accessibility, Attributes, Signal};
use crate::Component;
use crate::container::Children;
use crate::renderer::Renderer;
use taffy::prelude::*;

pub struct Card {
    pub id: String, pub style: Style, pub accessibility: Accessibility, pub attributes: Attributes,
    pub header: Option<Box<dyn Component>>, pub body: Children, pub footer: Option<Box<dyn Component>>,
}

impl Card {
    pub fn new() -> Self {
        let mut style = Style::default(); Theme::current().apply_defaults(&mut style);
        style.background.color = Some(Color::Semantic("surface".into(), None));
        style.padding = crate::utils::spacing::Spacing::all(16.0);
        style.rounding = crate::utils::border::Rounding::all(8.0);
        Self { id: generate_id(), style, accessibility: Accessibility::default(), attributes: Attributes::default(), header: None, body: Children::new(), footer: None }
    }
    pub fn header(mut self, h: Box<dyn Component>) -> Self { self.header = Some(h); self }
    pub fn child(mut self, c: Box<dyn Component>) -> Self { self.body.add(c); self }
}

impl Component for Card {
    fn id(&self) -> &str { &self.id }
    fn layout(&self, taffy: &mut TaffyTree<()>, parent: Option<NodeId>) -> NodeId {
        let node = taffy.new_with_children(self.style.to_taffy(), &[]).unwrap();
        if let Some(p) = parent { taffy.add_child(p, node).unwrap(); }
        if let Some(ref h) = self.header { h.layout(taffy, Some(node)); }
        self.body.layout_all(taffy, node);
        node
    }
    fn paint(&self, renderer: &mut Renderer, taffy: &TaffyTree<()>, node: NodeId, is_group_hovered: bool, render_pass: &mut wgpu::RenderPass<'_>) {
        let layout = taffy.layout(node).unwrap();
        if let Some(color) = self.style.background.color.clone() { renderer.draw_rect(layout.location.x, layout.location.y, layout.size.width, layout.size.height, color.to_rgba(), 8.0); }
        self.body.paint_all(renderer, taffy, node, is_group_hovered, render_pass);
    }
    fn on_click(&self) {}
    fn on_scroll(&self, _d: f32) {}
    fn on_drag(&self, _d: crate::utils::Vec2) {}
}

pub struct TableRow { pub cells: Vec<String> }

pub struct Table {
    pub id: String,
    pub headers: Vec<String>,
    pub rows: Vec<TableRow>,
    pub style: Style,
}

impl Table {
    pub fn new(headers: Vec<String>, rows: Vec<TableRow>) -> Self {
        Self { id: generate_id(), headers, rows, style: Style::default() }
    }
}

impl Component for Table {
    fn id(&self) -> &str { &self.id }
    fn layout(&self, taffy: &mut TaffyTree<()>, parent: Option<NodeId>) -> NodeId {
        let node = taffy.new_with_children(self.style.to_taffy(), &[]).unwrap();
        if let Some(p) = parent { taffy.add_child(p, node).unwrap(); }
        node
    }
    fn paint(&self, renderer: &mut Renderer, taffy: &TaffyTree<()>, node: NodeId, _is_group_hovered: bool, _render_pass: &mut wgpu::RenderPass<'_>) {
        let layout = taffy.layout(node).unwrap();
        let mut y_offset = 0.0;
        for header in &self.headers {
            renderer.draw_text(header, layout.location.x, layout.location.y + y_offset, 14.0, [0.6, 0.6, 0.6, 1.0], crate::utils::TextAlign::Left);
        }
        y_offset += 30.0;
        for row in &self.rows {
            let mut x_offset = 0.0;
            for cell in &row.cells {
                renderer.draw_text(cell, layout.location.x + x_offset, layout.location.y + y_offset, 14.0, [1.0, 1.0, 1.0, 1.0], crate::utils::TextAlign::Left);
                x_offset += 100.0;
            }
            y_offset += 25.0;
        }
    }
    fn on_click(&self) {}
    fn on_scroll(&self, _d: f32) {}
    fn on_drag(&self, _d: crate::utils::Vec2) {}
}

pub struct Accordion { pub id: String, pub title: String, pub is_expanded: Signal<bool>, pub children: Children }
impl Accordion {
    pub fn new(title: impl Into<String>, expanded: Signal<bool>) -> Self { Self { id: generate_id(), title: title.into(), is_expanded: expanded, children: Children::new() } }
    pub fn child(mut self, c: Box<dyn Component>) -> Self { self.children.add(c); self }
}
impl Component for Accordion {
    fn id(&self) -> &str { &self.id }
    fn layout(&self, taffy: &mut TaffyTree<()>, parent: Option<NodeId>) -> NodeId {
        let node = taffy.new_with_children(taffy::style::Style::default(), &[]).unwrap();
        if let Some(p) = parent { taffy.add_child(p, node).unwrap(); }
        if self.is_expanded.get() { self.children.layout_all(taffy, node); }
        node
    }
    fn paint(&self, renderer: &mut Renderer, taffy: &TaffyTree<()>, node: NodeId, is_group_hovered: bool, render_pass: &mut wgpu::RenderPass<'_>) {
        let layout = taffy.layout(node).unwrap();
        renderer.draw_text(&self.title, layout.location.x, layout.location.y, 14.0, [1.0, 1.0, 1.0, 1.0], crate::utils::TextAlign::Left);
        if self.is_expanded.get() { self.children.paint_all(renderer, taffy, node, is_group_hovered, render_pass); }
    }
    fn on_click(&self) { self.is_expanded.update(|v| *v = !*v); }
    fn on_scroll(&self, _d: f32) {}
    fn on_drag(&self, _d: crate::utils::Vec2) {}
}
