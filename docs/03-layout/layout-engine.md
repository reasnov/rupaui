# Module: Layout Engine (`layout.rs`) 📐

The Layout Engine is the mathematical brain of Layer 3. It translates abstract artisan styles into concrete physical coordinates, now with support for dynamic content measurement.

---

## 🏗️ Core Responsibilities

1.  **Taffy Orchestration:** Manages the lifecycle of the Taffy tree and triggers geometric resolution.
2.  **Content-Aware Sizing:** Integrates with the **Text Measurement Pipeline** to ensure that components containing text (like Buttons or Labels) calculate their size based on actual glyph dimensions.
3.  **Recursive Tree Traversal:** Coordinates the `layout` calls across the component hierarchy, passing the necessary measurers down the tree.

---

## 🗝️ Key API Elements

### `struct LayoutEngine`
- `compute(root, measurer, width, height)`: The primary execution point. It now requires a `&dyn TextMeasurer` to resolve content-dependent sizes.

---

## 🔄 The Measurement Bridge

Rupaui solves the "Blind Layout" problem by using a bridge between Layer 3 and Layer 2:
1.  **Layout Phase:** Taffy encounters a Text-based node.
2.  **Callback:** It triggers a `MeasureFunc` stored within the node.
3.  **Measurement:** The `MeasureFunc` uses the provided `TextMeasurer` (from Layer 2) to get the real pixel/cell width and height.
4.  **Resolution:** Taffy uses these dimensions to finalize the positions of parent and sibling elements.

---

## 🔄 Interaction
- **L3 (Layout Engine) -> L2 (TextMeasurer):** Requests dimensions for specific strings and font sizes.
- **L3 (Layout Engine) -> L5 (Component):** Provides the context needed for components to define their own measurement logic.
