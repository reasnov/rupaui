# Module: Scene Core (`mod.rs`) 🌳

The Scene Core is the central database of Layer 3. It maintains the absolute truth regarding the UI's structural hierarchy and coordinates the high-level layout resolution process.

---

## 🏗️ Core Responsibilities

1.  **Scene Lifecycle:** Manages the transitions between `Empty` and `Resolved` states.
2.  **Resolution Coordination:** Acts as the high-level orchestrator that connects the Component Tree (L5) with the Layout Engine (L3) and the Text Measurer (L2).
3.  **Spatial Intelligence:** Provides the `find_target` method for hit-testing based on the latest resolved geometry.

---

## 🗝️ Key API Elements

### `struct SceneCore`
- `resolve(root, measurer, width, height)`: Orchestrates a full geometric resolution. It now accepts a `measurer` to allow for content-aware sizing.
- `state`: Holds the `SceneState` enum, ensuring semantic clarity about the readiness of the UI tree.

---

## 🔄 The Agnostic Pipeline

To maintain the Agnostic Bridge, `SceneCore` never implements rendering itself. Instead:
1.  It receives a `&dyn TextMeasurer` from the platform runner (L1).
2.  It passes this measurer down to the `LayoutEngine`.
3.  The final `SceneNode` produced is then passed to the `Renderer` (L2) for actual painting.

---

## 🔄 Interaction
- **L1 -> L3:** Platform Runners provide the measurer and window dimensions.
- **L3 -> L2:** Uses the measurer provided by the active backend.
