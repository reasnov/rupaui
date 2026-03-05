# Semantic Layout Components

Rupaui provides high-level semantic components for structuring your interface. These components ensure that your layout code is declarative, readable, and highly performant.

## 🤸 Flexbox Component (`Flex`)

The `Flex` component is a dedicated container for one-dimensional layouts. It defaults to `display: flex`.

```rust
use rupaui::elements::Flex;
use rupaui::utils::{row, gap, items};

Flex::new()
    .style((row(), gap(12.0), items(AlignItems::Center)))
    .child(Box::new(Text::new("Item 1")))
    .child(Box::new(Text::new("Item 2")));
```

---

## 🏁 Grid Component (`Grid`)

The `Grid` component is a dedicated container for two-dimensional layouts. It defaults to `display: grid`.

```rust
use rupaui::elements::Grid;
use rupaui::utils::{grid_cols, gap};

Grid::new()
    .style((
        grid_cols(vec!["1fr", "2fr"]),
        gap(20.0)
    ))
    .child(Box::new(Div::new().text("Sidebar")))
    .child(Box::new(Div::new().text("Main Content")));
```

---

## 📦 Container, Row & Col

For traditional grid-based layouts (similar to Bootstrap), Rupaui provides these standard structural units:

- **`Container`**: Centers and constraints content width.
- **`Row`**: A horizontal wrapper for columns (Flex-based).
- **`Col`**: A grid cell with a specific span (1-12).

```rust
Container::new()
    .child(Box::new(
        Row::new()
            .child(Box::new(Col::new(6).text("50% Width")))
            .child(Box::new(Col::new(6).text("50% Width")))
    ))
```

## 🗝 Practical Guidance
- Use **`Flex`** or **`Grid`** for specialized, custom layouts.
- Use **`Container/Row/Col`** for standard page or form layouts.
- Every layout component is **Theme-Aware** and inherits global DNA defaults.
