# Module: Input Events (`events.rs`) ⌨️🖱️

This module defines the "Universal Language" of input. It acts as the semantic filter that cleans raw hardware signals into structured, typed data.

---

## 🧠 Internal Anatomy

### 1. The Normalization Schema
Input events are represented by the `InputEvent` enum. It uses `f32` coordinates (logical units) and standardized `KeyCode` variants to eliminate differences between OS keyboard layouts and mouse drivers.

### 2. Physical to Logical Mapping
The HAL runners (GUI/TUI) are responsible for the conversion:
- **GUI:** `Physical Pos / Scale Factor = Logical Pos`.
- **TUI:** `Terminal Column/Row = Logical Integer Grid`.

---

## 🗝️ API Anatomy: `enum InputEvent`

- **Pointer Events:** `PointerMove`, `PointerButton`, `PointerScroll`.
- **Keyboard Events:** `Key { key, state, modifiers }`.
- **System Events:** `Resize`, `SafeArea`, `Focus`, `Quit`.

---

## 🛡️ Reactive Design
Input events are non-blocking. They are captured by Layer 1 and pushed into the **InputDispatcher**, which then routes them into the reactive component tree.
