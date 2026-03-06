# Module: GUI Renderer (`gui/renderer.rs`) 🎨

The GUI Renderer is the WGPU-based implementation of the `Renderer` trait. It coordinates high-performance hardware-accelerated drawing for desktop and web environments.

---

## 🏗️ Core Responsibilities

1.  **WGPU Coordination:** Manages the `Device`, `Queue`, and `Surface` connection to the physical GPU.
2.  **Frame Orchestration:** Handles the transition between geometry and text render passes.
3.  **Camera & Transformation:** Applies `RenderCore` transforms (zoom/offset) to all vertex data before submission to the GPU.
4.  **Resource Cleanup:** Ensures that staging belts and encoders are cleared correctly after every frame.

---

## 🗝️ Key API Elements

### `struct GuiRenderer`
- `new(window)`: Asynchronously initializes the WGPU state.
- `begin_frame()`: Prepares the command encoder and acquires the next surface texture.
- `present()`: Flushes the `Batcher`, prepares `TextRenderer`, and submits the final command buffer.

---

## 🔄 Interaction
- **L2 (HAL Runner) -> L2 (GUI Renderer):** Triggers the frame lifecycle.
- **L2 (GUI Renderer) -> L2 (Batcher/Text):** Delegates specific drawing tasks to sub-engines.
