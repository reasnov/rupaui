# Module: GUI Text Renderer (`gui/text_renderer.rs`) 🔤

This module handles hardware-accelerated typography using the **Glyphon** and **Cosmic-text** ecosystems.

---

## 🏗️ Core Responsibilities

1.  **Text Shaping Pipeline:** Coordinates the layout of glyphs using the HarfBuzz-based `FontSystem`.
2.  **Glyph Caching:** Integrates with `TextAtlas` to store rasterized characters in GPU memory.
3.  **Viewport Synchronization:** Updates the internal text projection when the window size or camera zoom changes.

---

## 🗝️ Key API Elements

### `struct TextRenderer`
- `new(...)`: Initializes the font system, shaping cache, and text atlas.
- `prepare(...)`: Precalculates glyph positions and uploads them to the GPU atlas.
- `render(render_pass)`: Finalizes the drawing of all text areas within a specific render pass.

---

## 🔄 Interaction
- **L2 (Renderer) -> L2 (TextRenderer):** Submits UTF-8 strings and position data for shaping.
