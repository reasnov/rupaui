# Module: GUI Batcher (`gui/batcher.rs`) 🟦

The Batcher is the efficiency engine of the GUI backend. It minimizes CPU-to-GPU communication by grouping similar geometric shapes into monolithic buffers.

---

## 🏗️ Core Responsibilities

1.  **Vertex Management:** Manages internal vectors for `Vertex` and `Index` data.
2.  **GPU Buffer Provisioning:** Creates and updates high-speed VRAM buffers (`vertex_buffer`, `index_buffer`).
3.  **Automatic Indexing:** Automatically generates triangle indices for 2D quads (rectangles).

---

## 🗝️ Key API Elements

### `struct Batcher`
- `new(device, max_batch_size)`: Allocates fixed-size GPU buffers based on the requested capacity.
- `add_rect(vertices)`: Appends a 4-vertex quad to the current pending batch.
- `flush(queue, render_pass)`: Writes pending data to the GPU and executes the draw command.

### `struct Vertex`
The binary layout of a single point in space:
- `position`: Normalized device coordinates.
- `color`: RGBA array.
- `rect_size` & `radius`: Metadata for SDF corner rounding.

---

## 🔄 Interaction
- **L2 (Renderer) -> L2 (Batcher):** Feeds calculated vertex data from components.
