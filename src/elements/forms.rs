use crate::utils::{Style, StyleModifier, generate_id, Accessibility, Role, Signal, Variant, Attributes, Theme};
use crate::Component;
use crate::container::Children;

#[derive(Clone, Debug, PartialEq, Default)]
pub enum FormState { #[default] Normal, Valid, Invalid }

pub struct Input { pub id: String, pub label: Option<String>, pub placeholder: String, pub value: Signal<String>, pub state: Signal<FormState>, pub disabled: Signal<bool>, pub style: Style, pub attributes: Attributes }
impl Input {
    pub fn new(placeholder: impl Into<String>) -> Self {
        let mut style = Style::default();
        Theme::current().apply_defaults(&mut style);
        Self { id: generate_id(), label: None, placeholder: placeholder.into(), value: Signal::new(String::new()), state: Signal::new(FormState::Normal), disabled: Signal::new(false), style, attributes: Attributes::new() }
    }
    pub fn disabled(self, signal: Signal<bool>) -> Self { let mut s = self; s.disabled = signal; s }
}
impl Component for Input { fn id(&self) -> &str { &self.id } fn render(&self) { log::debug!("Input [{}] value='{}'", self.id, self.value.get()); } }

pub struct Select { pub id: String, pub options: Vec<(String, String)>, pub selected: Signal<String>, pub disabled: Signal<bool>, pub style: Style, pub attributes: Attributes }
impl Select {
    pub fn new(options: Vec<(String, String)>) -> Self {
        let mut style = Style::default();
        Theme::current().apply_defaults(&mut style);
        Self { id: generate_id(), options, selected: Signal::new(String::new()), disabled: Signal::new(false), style, attributes: Attributes::new() }
    }
}
impl Component for Select { fn id(&self) -> &str { &self.id } fn render(&self) { log::debug!("Select [{}]", self.id); } }

pub struct Check { pub id: String, pub label: String, pub checked: Signal<bool>, pub is_radio: bool, pub disabled: Signal<bool>, pub style: Style, pub attributes: Attributes }
impl Check {
    pub fn checkbox(label: impl Into<String>) -> Self {
        let mut style = Style::default();
        Theme::current().apply_defaults(&mut style);
        Self { id: generate_id(), label: label.into(), checked: Signal::new(false), is_radio: false, disabled: Signal::new(false), style, attributes: Attributes::new() }
    }
    pub fn radio(label: impl Into<String>) -> Self { let mut s = Self::checkbox(label); s.is_radio = true; s }
}
impl Component for Check { fn id(&self) -> &str { &self.id } fn render(&self) { log::debug!("Check [{}]", self.id); } }

pub struct Range { pub id: String, pub min: f32, pub max: f32, pub value: Signal<f32>, pub disabled: Signal<bool>, pub style: Style, pub attributes: Attributes }
impl Range {
    pub fn new(min: f32, max: f32) -> Self {
        let mut style = Style::default();
        Theme::current().apply_defaults(&mut style);
        Self { id: generate_id(), min, max, value: Signal::new(min), disabled: Signal::new(false), style, attributes: Attributes::new() }
    }
}
impl Component for Range { fn id(&self) -> &str { &self.id } fn render(&self) { log::debug!("Range [{}]", self.id); } }
