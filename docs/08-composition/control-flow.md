# Control Flow & Dynamic Rendering

Rupaui brings modern declarative patterns to Rust, allowing you to handle conditional rendering and list repetition with high-performance semantic components.

## 🎭 Conditional Rendering (`Show`)

The `Show` component renders its children only when a specific `Signal<bool>` is true. It also supports an optional fallback (else) component.

```rust
use rupaui::elements::Show;

let is_authenticated = Signal::new(false);

Show::new(is_authenticated.clone())
    .child(Box::new(Text::new("Welcome back!")))
    .fallback(Box::new(Button::new("Login")));
```

---

## 🔁 List Rendering (`ForEach`)

The `ForEach` component (Repeater) maps a collection of data to a list of components. It is optimized to work with `Signal<Vec<T>>`.

```rust
use rupaui::elements::ForEach;

let items = Signal::new(vec!["Apple", "Banana", "Cherry"]);

ForEach::new(items.clone(), |name| {
    Box::new(Text::new(*name))
});
```

---

## ⚡ Event Listeners

Components can respond to user interactions using event listeners. These are integrated with the **CBRA** model to trigger state changes.

```rust
let count = Signal::new(0);

Button::new("Increment")
    .on_click({
        let count = count.clone();
        move || count.update(|v| *v += 1)
    });
```

## 🗝 Implementation Standards
- **Fine-Grained**: Only the specific `Show` or `ForEach` component re-renders when its source signal changes.
- **Thread-Safe**: Event callbacks use `Arc` and `Send + Sync` to ensure compatibility with multi-threaded or async environments.
- **Zero Virtual DOM**: No heavy reconciliation process; Rupaui directly updates the necessary segments of the UI tree.
