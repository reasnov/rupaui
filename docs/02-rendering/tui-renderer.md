# Module: TUI Renderer (`tui/mod.rs`) ⌨️

The TUI Renderer implements character-based drawing for terminal environments. It replaces pixels with Unicode characters and ANSI colors.

---

## 🏗️ Core Responsibilities

1.  **Grid Management:** Manages a 2D buffer of cells, each containing a character and its foreground/background colors.
2.  **Diff-Rendering:** Maintains a "Previous Frame" buffer to calculate only the changes, minimizing flickering and terminal bandwidth usage.
3.  **Box Drawing:** Translates rectangular geometry into Unicode box-drawing sequences (`┌`, `─`, `┐`, etc).
4.  **ANSI Translation:** Converts high-precision OKLCH colors into their nearest 24-bit ANSI equivalents for terminal display.

---

## 🗝️ Key API Elements

### `struct TuiRenderer`
- `new(width, height)`: Initializes the cell buffers.
- `draw_rect(...)`: Paints box-drawing characters into the current buffer.
- `draw_text(...)`: Writes strings into the buffer at specific coordinates.
- `present()`: Synchronizes the buffer with the terminal's `stdout`.

---

## 🔄 Interaction
- **L2 (HAL Runner) -> L2 (TUI Renderer):** Triggers the grid resolution and output phase.
