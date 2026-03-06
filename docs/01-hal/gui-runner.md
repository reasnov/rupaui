# Module: GUI Runner (`gui/mod.rs`) 🖼️🏃

This module implements the specific Hardware Abstraction for graphical environments (Desktop/Web). It manages the marriage between Winit's event loop and WGPU's rendering pipeline.

---

## 🏗️ Core Responsibilities

1.  **Winit Application Handling:** Implements the `ApplicationHandler` trait to manage OS window lifecycle events.
2.  **GPU Frame Orchestration:** Coordinates when the renderer should start and end a frame based on `RedrawRequested` signals.
3.  **Input Translation:** Converts native Winit events into Rupaui's `RawInputEvent` format.
4.  **DPI Management:** Synchronizes logical and physical scaling factors to keep the UI sharp.

---

## 🗝️ Key API Elements

### `struct RupauiRunner`
The engine room of the GUI backend:
- `handle_redraw()`: Triggers the Layout (L3) and Paint (L2) phases.
- `dispatch_event()`: Feeds translated events into the `EventDispatcher`.
- `run_app()`: The entry point that consumes the current thread and starts the Winit loop.

---

## 🔄 Interaction
- **L1 (HAL) -> L2 (Rendering):** Manages the WGPU device and triggers render passes.
- **L1 (HAL) -> L3 (Layout):** Triggers Taffy recalculations on window resize.
