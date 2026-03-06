# Module: Event Dispatcher (`dispatcher.rs`) 🎯

The Event Dispatcher is the logic bridge that connects raw hardware signals to specific UI components. It determines "who" was interacted with and "what" action should be triggered.

---

## 🏗️ Core Responsibilities

1.  **Hit-Testing:** Recursively traverses the element tree (using coordinates from Taffy Layer 3) to find the deepest component under the pointer.
2.  **Event Propagation:** Bubbles events from the target up to the root, allowing for event consumption (stoppage).
3.  **Semantic Translation:** Converts `RawInputEvent` into semantic component methods like `on_click`, `on_scroll`, and `on_drag`.

---

## 🗝️ Key API Elements

### `struct EventDispatcher`
- `hit_test(...)`: Performs the geometric search for components.
- `dispatch(...)`: The main entry point that processes a `RawInputEvent` and triggers the appropriate component lifecycle hooks.

### `struct UIEvent`
The context object passed to components:
- `local_pos`: The coordinate relative to the component's top-left corner.
- `consume()`: Stops the event from bubbling further up the tree.

---

## 🔄 Interaction
- **L1 (Events) -> L1 (Dispatcher):** Receives the standardized input.
- **L1 (Dispatcher) -> L3 (Layout):** Queries Taffy for component positions and sizes.
- **L1 (Dispatcher) -> L5 (Architecture):** Calls methods on the `Component` trait.
