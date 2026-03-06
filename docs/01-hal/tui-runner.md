# Module: TUI Runner (`tui/mod.rs`) 🖥️🏃

This module implements the specific Hardware Abstraction for terminal environments. It replaces pixels with characters and windows with raw terminal buffers.

---

## 🏗️ Core Responsibilities

1.  **Terminal Handshake:** Manages switching to Raw Mode and the Alternate Screen buffer.
2.  **Polling Loop:** Runs a lightweight loop that waits for `crossterm` events (Keys, Mouse, Resize).
3.  **Input Decoding:** Translates ANSI escape sequences and terminal signals into Rupaui's `RawInputEvent`.
4.  **Automatic Cleanup:** Ensures the terminal is restored to its original state on application exit or panic.

---

## 🗝️ Key API Elements

### `struct TuiRunner`
The engine room of the Terminal backend:
- `run()`: Initializes the terminal and starts the polling loop.
- `handle_redraw()`: Calculates layout and prepares character buffers (L2).
- `dispatch_event()`: Feeds translated terminal input into the `EventDispatcher`.

---

## 🔄 Interaction
- **L1 (HAL) -> L2 (TUI Renderer):** Triggers the rendering of characters to the terminal stdout.
- **L1 (HAL) -> L3 (Layout):** Triggers Taffy recalculations when the terminal window is resized.
