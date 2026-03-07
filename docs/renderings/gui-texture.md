# Module: GUI Texture (`gui/texture.rs`) 🖼️

The Texture module manages raw pixel data within VRAM. It acts as the "Storage Container" for images and icons used throughout the GUI.

---

## 🏗️ Core Responsibilities

1.  **GPU Memory Allocation:** Creates `wgpu::Texture` objects with optimal usage flags (Binding & Copying).
2.  **Asset Uploading:** Writes raw byte buffers into GPU memory using the `Queue`.
3.  **Sampler Management:** Configures how the GPU interpolates pixels (e.g., Nearest Neighbor for crisp pixel art or Linear for smooth scaling).
4.  **Bind Group Creation:** Automates the creation of WGPU bind groups required for shaders to access texture data.

---

## 🗝️ Key API Elements

### `struct Texture`
- `from_image(device, queue, img, ...)`: Creates a fully ready GPU texture from a dynamic image.
- `view`: The logical view into the texture data.
- `bind_group`: The handle passed to the render pipeline during drawing.

---

## 🔄 Interaction
- **L2 (Renderer) -> L2 (Texture):** Supplies textures to the `Batcher` for drawing image-based quads.
