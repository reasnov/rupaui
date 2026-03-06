# Module: Input Events (`events.rs`) ⌨️🖱️

This module defines the "Universal Language" of input for the Rupaui framework. It abstracts away raw OS or terminal sequences into platform-agnostic data structures.

---

## 🏗️ Core Responsibilities

1.  **Unified Input Schema:** Provides a single source of truth for what an "interaction" looks like in Rupaui.
2.  **Platform Decoupling:** Ensures that higher layers (L3-L9) never have to import platform-specific types like `winit::event::WindowEvent` or `crossterm::event::Event`.

---

## 🗝️ Key API Elements

### `enum RawInputEvent`
The central enum containing all standardized signals:
- `PointerMove`: Normalized cursor or touch coordinates.
- `PointerButton`: Primary, Secondary, or Auxiliary button states.
- `PointerScroll`: Scroll deltas (Vector-based).
- `Key`: Standardized KeyCodes (Enter, Esc, Chars, etc).
- `Resize`: Window or Terminal size changes.
- `Quit`: Application termination signal.

### `struct Modifiers`
A bit-flag-like structure tracking the state of `Shift`, `Ctrl`, `Alt`, and `Logo` keys across all platforms.

---

## 🔄 Interaction
- **L1 (Backend) -> L1 (Events):** Backends translate their native bytes into these types.
- **L1 (Events) -> L1 (Dispatcher):** Normalized events are fed into the dispatcher for tree propagation.
