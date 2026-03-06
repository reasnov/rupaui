# Module: Renderer Interface (`mod.rs`) 🎼

This module defines the universal contract for all visual output in Rupaui. It provides the abstractions necessary to ensure that higher layers (3-9) remain backend-agnostic.

---

## 🏗️ Core Responsibilities

1.  **RenderCore Struct:** Centralizes the shared state for all renderers, such as the viewport size, camera offset, and zoom level. This enables "Composition over Inheritance".
2.  **Renderer Trait:** The primary interface for issuing draw commands. It abstracts away the platform-specific details of GPU buffers or Terminal ANSI codes.
3.  **Backend Management:** Dynamically exposes the `gui` and `tui` sub-modules based on compilation features.

---

## 🗝️ Key API Elements

### `struct RenderCore`
- `logical_size`: The dimensions of the available drawing area in logical units.
- `camera_offset`: 2D vector for panning the UI.
- `camera_zoom`: Scaling factor for zooming.

### `trait Renderer`
The source of truth for drawing:
- `draw_rect(...)`: Commands the backend to draw a quad.
- `draw_text(...)`: Commands the backend to render shaped text.
- `push_clip(...)` / `pop_clip(...)`: Defines a sub-region for drawing.
- `present()`: Finalizes the frame and sends it to the hardware.

---

## 🔄 Interaction
- **L2 -> L3:** Provides spatial data to the Layout Engine.
- **L2 -> L5:** Receives paint instructions from UI Components.
