# Module: GUI Window (`gui/window.rs`) 🪟

A low-level wrapper around the `winit::window::Window` object. It handles the specific details of OS-level window creation and state.

---

## 🏗️ Core Responsibilities

1.  **Window Creation:** Handles the handshake with the OS to obtain a physical surface.
2.  **State Tracking:** Maintains the current window title, size, and logical-to-physical scale factor.
3.  **Redraw Signaling:** Provides a clean `request_redraw()` method that proxies to the underlying Winit window.

---

## 🗝️ Key API Elements

### `struct Window`
- `new(...)`: Initializes a window with specific attributes (title, width, height).
- `raw()`: Provides access to the underlying Winit window handle (required by WGPU).
- `size()`: Returns the logical dimensions of the window.
- `scale_factor()`: Returns the current DPI scaling factor.

---

## 🔄 Interaction
- **L1 (HAL) -> Layer 1 (HAL Runner):** Provides the window surface needed to initialize the GPU Renderer.
