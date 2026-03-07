# Module: GUI Backend (`gui/mod.rs`) 🛠️

The central hub for all graphical rendering logic. It organizes the specialized engines required for WGPU-based output.

---

## 🏗️ Core Responsibilities

1.  **Module Aggregation:** Exposes sub-modules like `batcher`, `pipeline`, and `text_renderer` to the rest of the framework.
2.  **Public API Smoothing:** Provides clean re-exports of common types (`Renderer`, `Vertex`, `Texture`) so that Layer 1 and 2 can communicate efficiently.

---

## 🗝️ Key API Elements

- `pub use renderer::Renderer`: The concrete GUI implementation of the universal interface.
- `pub use texture::Texture`: The primary handle for image assets.
- `pub use batcher::{Batcher, Vertex}`: Low-level access to the GPU buffer manager.

---

## 🔄 Interaction
- **L2 (Orchestrator) -> L2 (GUI Backend):** Used when the `gui` feature is active.
