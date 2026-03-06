# Spacing & Sizing Utilities

Rupaui provides a flexible system for controlling the dimensions and spacing of semantic components, including support for CSS-like shorthands and logical properties.

## 📈 Unified Scale System

Rupaui utilizes a standardized 10-step scale for consistent sizing across components and utilities. This scale applies to width, height, padding, margin, and component-specific sizes (like buttons).

### Available Scales
| Scale | Multiplier | Description |
| :--- | :--- | :--- |
| `Xs` | 0.75 | Extra Small |
| `Sm` | 0.875 | Small |
| `Md` | 1.0 | Medium (Default) |
| `Lg` | 1.125 | Large |
| `Xl` | 1.25 | Extra Large |
| `Xl2` | 1.5 | 2XL |
| `Xl3` | 1.875 | 3XL |
| `Xl4` | 2.25 | 4XL |
| `Xl5` | 3.0 | 5XL |
| `Xl6` | 3.75 | 6XL |

### Scale-Based Utilities
Instead of passing raw pixel values, you can use semantic scale utilities:
- `.w_scale(Scale::Lg)`: Set width based on Large scale.
- `.h_scale(Scale::Sm)`: Set height based on Small scale.
- `.p_scale(Scale::Md)`: Set padding based on Medium scale.
- `.m_scale(Scale::Xl)`: Set margin based on Extra Large scale.
- `.rounded_scale(Scale::Xs)`: Set corner radius based on Extra Small scale.

```rust
use rupaui::utils::Scale;

Button::new("Artisan")
    .size(Scale::Xl2)
    .style(p_scale(Scale::Md))
```

---

## 📏 Sizing

Control the physical and logical dimensions of elements.

### Physical Sizing
- `.w(f32)` / `.h(f32)`: Set width and height.
- `.min_w(f32)` / `.max_w(f32)`: Set minimum and maximum width.
- `.min_h(f32)` / `.max_h(f32)`: Set minimum and maximum height.

### Logical Sizing (Writing-mode aware)
- `.inline_size(f32)`: Set size in the inline direction (maps to width in horizontal mode).
- `.block_size(f32)`: Set size in the block direction (maps to height in horizontal mode).

---

## ↔️ Spacing (Margin & Padding)

Rupaui supports flexible input types for spacing, allowing you to set 1, 2, or 4 values at once.

### Input Formats
- `f32`: Sets all sides (e.g., `.p(10.0)`).
- `(f32, f32)`: Sets (Vertical, Horizontal) (e.g., `.p((8.0, 16.0))`).
- `(f32, f32, f32, f32)`: Sets (Top, Right, Bottom, Left) (e.g., `.p((10.0, 5.0, 15.0, 2.0))`).

### Shorthands
- `.m()` / `.p()`: Main margin and padding methods.
- `.mx()` / `.px()`: Horizontal only.
- `.my()` / `.py()`: Vertical only.
