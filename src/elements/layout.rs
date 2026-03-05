use crate::utils::{Style, StyleModifier, generate_id, Accessibility, Role, Theme};
use crate::Component;
use crate::container::Children;

#[derive(Clone, Debug, PartialEq, Default)]
pub enum ContainerVariant { #[default] Fixed, Fluid }

pub struct Container {
    pub id: String,
    pub variant: ContainerVariant,
    pub style: Style,
    pub accessibility: Accessibility,
    pub children: Children,
}

impl Container {
    pub fn new() -> Self {
        let mut style = Style::default();
        Theme::current().apply_defaults(&mut style);
        Self {
            id: generate_id(),
            variant: ContainerVariant::Fixed,
            style,
            accessibility: Accessibility { role: Role::None, ..Default::default() },
            children: Children::new(),
        }
    }
    pub fn style(mut self, modifier: impl StyleModifier) -> Self { modifier.apply(&mut self.style); self }
    pub fn child(mut self, child: Box<dyn Component>) -> Self { self.children.add(child); self }
}

impl Component for Container { fn id(&self) -> &str { &self.id } fn render(&self) { self.children.render_all(); } }

pub struct Row { pub id: String, pub style: Style, pub children: Children }
impl Row {
    pub fn new() -> Self {
        let mut style = Style::default();
        Theme::current().apply_defaults(&mut style);
        style.layout.display = crate::utils::Display::Flex;
        style.flex.direction = crate::utils::FlexDirection::Row;
        style.flex.wrap = crate::utils::FlexWrap::Wrap;
        Self { id: generate_id(), style, children: Children::new() }
    }
    pub fn child(mut self, child: Box<dyn Component>) -> Self { self.children.add(child); self }
}
impl Component for Row { fn id(&self) -> &str { &self.id } fn render(&self) { self.children.render_all(); } }

pub struct Col { pub id: String, pub span: u16, pub style: Style, pub children: Children }
impl Col {
    pub fn new(span: u16) -> Self {
        let mut style = Style::default();
        Theme::current().apply_defaults(&mut style);
        Self { id: generate_id(), span, style, children: Children::new() }
    }
    pub fn child(mut self, child: Box<dyn Component>) -> Self { self.children.add(child); self }
}
impl Component for Col { fn id(&self) -> &str { &self.id } fn render(&self) { self.children.render_all(); } }
