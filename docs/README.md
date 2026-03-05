# Rupaui Documentation Index

Welcome to the **Rupaui** documentation. Rupaui is a modern, cross-platform UI framework built with Rust, designed for artisans who value semantic structure and utility-first flexibility.

## 🏗 Core Foundations

1. [**Core Philosophy**](./core/philosophy.md) - "Utility-First, Semantic-Support" and the CBRA model.
2. [**State Management**](./core/state-management.md) - Reactive UI with Signals and Memos.
3. [**Control Flow**](./core/control-flow.md) - **NEW!** Conditional rendering and list repetition.
4. [**Extending Rupaui**](./core/extending.md) - Building your own utilities and elements.
5. [**Component Attributes**](./core/attributes.md) - Dynamic metadata and data-attributes.
6. [**Vector Math**](./core/vector-math.md) - High-precision calculations for paths and layouts.
7. [**WebAssembly & Platforms**](./core/platforms.md) - Deployment and cross-platform guides.

## 📚 Component Reference

1. [**Window**](./components/window.md) - Physical viewport and GPU surface.
2. [**Section**](./components/section.md) - Top-level semantic regions.
3. [**Div**](./components/div.md) - Generic grouping and containers.
4. [**Text**](./components/text.md) - Typography and content elements.
5. [**Forms**](./components/forms.md) - Reactive inputs and validation.
6. [**UI Elements**](./components/elements.md) - Buttons, Alerts, Modals, etc.
7. [**Brand**](./components/brand.md) - Responsive application identity.
8. [**SVG Drawing**](./components/svg-drawing.md) - Vector graphics and paths.

## 🎨 Styling & Design

1. [**Modular Styling**](./styling/modular-styling.md) - Functional and atomic styling API.
2. [**Interaction Variants**](./styling/variants.md) - Hover, Focus, and Active states.
3. [**Theming**](./styling/theme.md) - Global design tokens and color modes.
4. [**Artisan Colors**](./styling/styling.md) - Deep dive into OKLCH and the palette.
5. [**Layout, Flex & Grid**](./styling/layout.md) - Structural positioning.
6. [**Spacing & Sizing**](./styling/spacing-sizing.md) - Dimensions and box model.
7. [**Typography**](./styling/typography.md) - Text decoration and flow.
8. [**Background & Border**](./styling/background-border.md) - Visual containment.
9. [**Visual Effects**](./styling/effects.md) - Shadows, blending, and masking.
10. [**Filters & Backdrop**](./styling/filters.md) - GPU-accelerated image processing.
11. [**Motion & Transforms**](./styling/motion-transform.md) - 3D geometry and animations.
12. [**Helpers**](./styling/helpers.md) - Stacks, Ratios, and shortcuts.
13. [**Tables**](./styling/tables.md) - Structured data layout.
14. [**Interactivity & SVG**](./styling/interactivity-svg.md) - Input feedback and vector styling.

## 🚀 Quick Start (List Rendering)

```rust
use rupaui::prelude::*;

fn main() {
    let users = Signal::new(vec!["Alice", "Bob", "Charlie"]);
    
    let user_list = ForEach::new(users.clone(), |name| {
        Box::new(Text::new(*name).style(p(8.0)))
    });
}
```
