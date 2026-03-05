# Interaction Variants (Hover, Focus, etc.)

Rupaui supports state-driven styling similar to TailwindCSS, allowing you to define different visual properties for different interaction states using a fluent Rust API.

## 🖱 Supported Variants

- `.hover(Modifier)`: Applied when the mouse pointer is over the element.
- `.focus(Modifier)`: Applied when the element has keyboard focus.
- `.active(Modifier)`: Applied when the element is being pressed/clicked.
- `.disabled_style(Modifier)`: Applied when the component's `is_disabled` signal is true.

---

## 🚀 Basic Usage

You can pass any `StyleModifier` (atomic functions, tuples, or full Style objects) to a variant method.

```rust
use rupaui::utils::{bg, color, rounded, p};

Button::new("Interactive Artisan")
    .style((
        bg("zinc-800"),
        p(12.0),
        // Hover state
        hover(bg("indigo-600")),
        // Active/Pressed state
        active((
            bg("indigo-700"),
            translate(0.0, 2.0, 0.0)
        )),
        // Focus state
        focus(outline(2.0, BorderStyle::Solid, "white"))
    ));
```

---

## 🏗 How it Works

1.  **Conditional Rendering**: During the `Paint` phase, Rupaui checks the current interaction state of the component.
2.  **Style Merging**: If a state is active (e.g., the user is hovering), the properties defined in `.hover()` are merged on top of the base style.
3.  **Performant Storage**: Variants are stored as `Option<Box<Style>>`, meaning they consume zero extra memory if not defined.

## 💡 Best Practices
- Use `.hover()` for feedback on clickable elements.
- Always use `.focus()` for accessibility (A11y) to help keyboard users navigate.
- Combine with `.transition()` for smooth visual changes between states.
