# Extending Rupaui

Rupaui is designed to be an extensible framework. You can freely create your own **Utilities** and **Semantic Components** without modifying the core library.

## 🛠 Creating Custom Utilities

To create a reusable style utility, simply write a function that returns `impl StyleModifier`.

```rust
use rupaui::utils::{Style, StyleModifier};

pub fn glass_effect() -> impl StyleModifier {
    move |s: &mut Style| {
        s.background.color = Some("white".a(0.1));
        s.filters.backdrop.push(Filter::Blur(10.0));
        s.rounding = Rounding::all(12.0);
    }
}

// Usage
Div::new().style(glass_effect());
```

---

## 🏗 Creating Custom Semantic Components

Every component in Rupaui implements the `Component` trait. You can create your own components by following this pattern:

```rust
use rupaui::prelude::*;
use rupaui::utils::{generate_id, Style, StyleModifier};

pub struct MyWidget {
    pub id: String,
    pub style: Style,
}

impl MyWidget {
    pub fn new() -> Self {
        Self {
            id: generate_id(),
            style: Style::default(),
        }
    }

    pub fn style(mut self, modifier: impl StyleModifier) -> Self {
        modifier.apply(&mut self.style);
        self
    }
}

impl Component for MyWidget {
    fn id(&self) -> &str { &self.id }
    fn render(&self) {
        log::debug!("Rendering MyWidget [{}]", self.id);
        // Add your WGPU or logic here
    }
}
```

---

## 🗝 Design Tokens Extension

You can also use the `Theme` module to store non-visual metadata that your components can consume.

```rust
Theme::update(|t| {
    t.custom.insert("api-endpoint".into(), "https://api.rupa.art".into());
});

let url = Theme::current().custom.get("api-endpoint");
```

## 🚀 Reusability
- **Modular**: Package your custom elements and utilities into their own Rust modules or crates.
- **Composable**: Custom modifiers can be combined using tuples: `.style((glass_effect(), p(20.0)))`.
- **Type-Safe**: All extensions benefit from Rust's compile-time checks.
