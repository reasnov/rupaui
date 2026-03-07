# Infrastructure: Root & Body 🏛️

The Root and Body elements are the foundational layers of every Rupaui application. They provide the transition from the host Operating System into the Rupaui agnostik ecosystem.

---

## 🏗️ The Root (App Metadata Manifest)

The **Root** is managed by the `App` struct. It is not a visual element but an architectural manifest that synchronizes the application's identity with the host environment.

### 1. Metadata Manifest
Rupaui supports modern metadata standards (similar to Web PWAs), allowing you to define the app's personality beyond just a title.

| Property | Description | Platform Sync |
| :--- | :--- | :--- |
| **Title** | The display name of the app. | Window title / document.title |
| **Icon** | App logo (Path or Bytes). | Window icon / Favicon |
| **Theme Color** | Primary brand color. | Browser theme-color / Status bar |
| **Display Mode** | Viewing preference. | Fullscreen / Standalone (Web) |

---

## 🕺 The Body (Universal Container & Z-Stack)

The **Body** is the internal root-level visual component (`src/core/body.rs`). It acts as the ultimate viewport and manages the **Global Z-Stack** and **Viewport Context**.

### 1. The Multi-Layer Architecture
To handle complex UI scenarios like Modals and Tooltips, the Body maintains a dual-layer system:
- **Primary Content Layer**: Where your main application tree lives.
- **Global Overlay Layer**: A prioritized layer for elements that must appear above all content.

### 2. Global Viewport Context
The Body maintains a reactive **Viewport Signal**. This allows any component in the tree to listen to window size changes and adapt its layout (e.g., switching from a Sidebar to a Bottom Nav on mobile).

### 3. Pointer Cursor Management
Rupaui supports context-aware mouse cursors. 
- **Resolution**: During the hit-test phase, the `InputDispatcher` identifies the topmost hovered component and reads its `cursor` style.
- **Feedback Loop**: The dispatcher sends a request back to the **HAL Runner** to update the native OS cursor (e.g., changing to `Hand` when hovering over a button).

### 4. Reactive Safe Area
On mobile platforms, the Body automatically reacts to "notches" and "home indicators" by applying dynamic padding. This ensures that even the Overlay Layer respects hardware boundaries.

---

## 🔄 Interaction Flow

1.  **Hit-Testing Priority**: Input events are dispatched from the **top-down**. The Overlay Layer always has priority over the Content Layer.
2.  **Cursor Resolution**: After dispatching a move event, the framework determines the requested cursor shape from the hit component's style.
3.  **Viewport Sync**: Every OS `Resize` event is piped through the Body, updating the global Viewport Signal and triggering a reactive relayout where necessary.
4.  **Paint Order**: Content is painted first, followed by Overlays to ensure correct visual stacking.
