use crate::utils::{Signal, generate_id};
use crate::Component;
use crate::container::Children;
use std::fmt::Debug;

/// Conditional rendering component (if/else).
pub struct Show {
    pub id: String,
    pub when: Signal<bool>,
    pub fallback: Option<Box<dyn Component>>,
    pub children: Children,
}

impl Show {
    pub fn new(condition: Signal<bool>) -> Self {
        Self {
            id: generate_id(),
            when: condition,
            fallback: None,
            children: Children::new(),
        }
    }

    pub fn fallback(mut self, component: Box<dyn Component>) -> Self {
        self.fallback = Some(component);
        self
    }

    pub fn child(mut self, child: Box<dyn Component>) -> Self {
        self.children.add(child);
        self
    }
}

impl Component for Show {
    fn id(&self) -> &str { &self.id }
    fn render(&self) {
        if self.when.get() {
            self.children.render_all();
        } else if let Some(ref fallback) = self.fallback {
            fallback.render();
        }
    }
}

/// Dynamic list rendering component (Repeater).
/// Takes a Signal of Vec<T> and a mapping function.
pub struct ForEach<T> {
    pub id: String,
    pub items: Signal<Vec<T>>,
    pub render_item: Box<dyn Fn(&T) -> Box<dyn Component>>,
}

impl<T: Clone + Debug + 'static> ForEach<T> {
    pub fn new(items: Signal<Vec<T>>, render_item: impl Fn(&T) -> Box<dyn Component> + 'static) -> Self {
        Self {
            id: generate_id(),
            items,
            render_item: Box::new(render_item),
        }
    }
}

impl<T: Clone + Debug> Component for ForEach<T> {
    fn id(&self) -> &str { &self.id }
    fn render(&self) {
        let current_items = self.items.get();
        log::debug!("Rendering ForEach [{}] with {} items", self.id, current_items.len());
        for item in current_items.iter() {
            (self.render_item)(item).render();
        }
    }
}
