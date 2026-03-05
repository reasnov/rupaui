# Rupaui 🎨

**Rupaui** is a modern, cross-platform UI framework for Rust, built for artisans who demand both structured semantic components and total utility-first freedom.

Inspired by the efficiency of TailwindCSS and the reliability of Bootstrap, Rupaui provides a high-performance rendering engine built directly on top of **WGPU**.

## 🌟 Key Features

- **Utility-First API**: Compose complex styles with chainable methods (e.g., `.px(4).bg(0.1, 0.2, 0.1)`).
- **Semantic Components**: Out-of-the-box support for meaningful UI elements like `Button`, `Panel`, and `Grid`.
- **WGPU Powered**: GPU-accelerated rendering for buttery smooth 60/120 FPS interfaces.
- **Deterministic Layout**: Uses the **Taffy** engine for lightning-fast Flexbox and CSS Grid calculations.
- **Cross-Platform**: Designed to run natively on Windows, macOS, Linux, and eventually WebAssembly.

## 💻 Example Usage

```rust
use rupaui::elements::Div;
use rupaui::utils::Style;

fn main() {
    let my_style = Style::new()
        .p(10.0)
        .bg(0.9, 0.9, 0.8, 1.0)
        .rounded(4.0);

    Div::new()
        .with_style(my_style)
        .child("Hello from Rupaui!");
}
```

## 🏗 Architecture

Rupaui is designed with a strict **Separation of Concerns**:
1. **Windowing**: Leverages `winit` for hardware-agnostic events.
2. **Layout**: Uses `taffy` for geometric resolution.
3. **Paint**: A specialized `wgpu` backend for low-level drawing calls.
4. **Input**: A normalized tactile layer for consistent cross-OS feedback.

## 🛠 Status

Rupaui is currently in **Active Development**. It is being used as the primary engine for the [Rupa Pixel Editor](https://github.com/reasnov/rupa-pixel-editor).

---
Built with ❤️ by Reas Vyn.
