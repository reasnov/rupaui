use crate::utils::{Style, StyleModifier, generate_id, Accessibility, Role, Signal, Variant, Attributes};
use crate::Component;
use crate::container::Children;

#[derive(Clone, Debug, PartialEq, Default)]
pub enum FormState { #[default] Normal, Valid, Invalid }

/// A semantic component for text inputs and textareas.
pub struct Input {
    pub id: String,
    pub label: Option<String>,
    pub placeholder: String,
    pub value: Signal<String>,
    pub state: Signal<FormState>,
    pub is_textarea: bool,
    pub disabled: Signal<bool>,
    pub is_readonly: Signal<bool>,
    pub autofocus: bool,
    pub autocomplete: Option<String>,
    pub required: bool,
    pub style: Style,
    pub attributes: Attributes,
    pub accessibility: Accessibility,
}

impl Input {
    pub fn new(placeholder: impl Into<String>) -> Self {
        Self {
            id: generate_id(),
            label: None,
            placeholder: placeholder.into(),
            value: Signal::new(String::new()),
            state: Signal::new(FormState::Normal),
            is_textarea: false,
            disabled: Signal::new(false),
            is_readonly: Signal::new(false),
            autofocus: false,
            autocomplete: None,
            required: false,
            style: Style::default(),
            attributes: Attributes::new(),
            accessibility: Accessibility { role: Role::None, ..Default::default() },
        }
    }

    pub fn attr(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.attributes.set(key, value);
        self
    }

    pub fn autofocus(mut self) -> Self { self.autofocus = true; self }
    pub fn required(mut self) -> Self { self.required = true; self }
    pub fn autocomplete(mut self, val: impl Into<String>) -> Self { self.autocomplete = Some(val.into()); self }
    pub fn disabled(self, signal: Signal<bool>) -> Self { let mut s = self; s.disabled = signal; s }
    pub fn readonly(self, signal: Signal<bool>) -> Self { let mut s = self; s.is_readonly = signal; s }
    pub fn label(mut self, text: impl Into<String>) -> Self { self.label = Some(text.into()); self }
    pub fn textarea(mut self) -> Self { self.is_textarea = true; self }
    pub fn value(self, signal: Signal<String>) -> Self { let mut s = self; s.value = signal; s }
    pub fn state(self, signal: Signal<FormState>) -> Self { let mut s = self; s.state = signal; s }
    pub fn style(mut self, modifier: impl StyleModifier) -> Self { modifier.apply(&mut self.style); self }
}

impl Component for Input {
    fn id(&self) -> &str { &self.id }
    fn render(&self) {
        log::debug!("Rendering Input [{}] value='{}' attrs={:?}", self.id, self.value.get(), self.attributes.map);
    }
}

// ... Select, Check, Range updates follow same pattern ...
pub struct Select { pub id: String, pub options: Vec<(String, String)>, pub selected: Signal<String>, pub disabled: Signal<bool>, pub autofocus: bool, pub required: bool, pub style: Style, pub attributes: Attributes }
impl Select {
    pub fn new(options: Vec<(String, String)>) -> Self { Self { id: generate_id(), options, selected: Signal::new(String::new()), disabled: Signal::new(false), autofocus: false, required: false, style: Style::default(), attributes: Attributes::new() } }
    pub fn attr(mut self, key: impl Into<String>, value: impl Into<String>) -> Self { self.attributes.set(key, value); self }
    pub fn autofocus(mut self) -> Self { self.autofocus = true; self }
    pub fn disabled(self, signal: Signal<bool>) -> Self { let mut s = self; s.disabled = signal; s }
}
impl Component for Select { fn id(&self) -> &str { &self.id } fn render(&self) { log::debug!("Rendering Select [{}] attrs={:?}", self.id, self.attributes.map); } }

pub struct Check { pub id: String, pub label: String, pub checked: Signal<bool>, pub is_radio: bool, pub disabled: Signal<bool>, pub required: bool, pub style: Style, pub attributes: Attributes }
impl Check {
    pub fn checkbox(label: impl Into<String>) -> Self { Self { id: generate_id(), label: label.into(), checked: Signal::new(false), is_radio: false, disabled: Signal::new(false), required: false, style: Style::default(), attributes: Attributes::new() } }
    pub fn radio(label: impl Into<String>) -> Self { let mut s = Self::checkbox(label); s.is_radio = true; s }
    pub fn attr(mut self, key: impl Into<String>, value: impl Into<String>) -> Self { self.attributes.set(key, value); self }
    pub fn disabled(self, signal: Signal<bool>) -> Self { let mut s = self; s.disabled = signal; s }
}
impl Component for Check { fn id(&self) -> &str { &self.id } fn render(&self) { log::debug!("Rendering Check [{}] attrs={:?}", self.id, self.attributes.map); } }

pub struct Range { pub id: String, pub min: f32, pub max: f32, pub value: Signal<f32>, pub disabled: Signal<bool>, pub style: Style, pub attributes: Attributes }
impl Range {
    pub fn new(min: f32, max: f32) -> Self { Self { id: generate_id(), min, max, value: Signal::new(min), disabled: Signal::new(false), style: Style::default(), attributes: Attributes::new() } }
    pub fn attr(mut self, key: impl Into<String>, value: impl Into<String>) -> Self { self.attributes.set(key, value); self }
    pub fn disabled(self, signal: Signal<bool>) -> Self { let mut s = self; s.disabled = signal; s }
}
impl Component for Range { fn id(&self) -> &str { &self.id } fn render(&self) { log::debug!("Rendering Range [{}] attrs={:?}", self.id, self.attributes.map); } }
