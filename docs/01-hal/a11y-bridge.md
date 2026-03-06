# Accessibility Bridge (A11y) ♿

Accessibility is a first-class citizen in Rupaui. Layer 1 provides the bridge between the framework's semantic components and the Operating System's accessibility services.

---

## 🌉 AccessKit Integration

Rupaui integrates with **AccessKit**, a cross-platform accessibility infrastructure for Rust. This allows Rupaui to:
1.  **Expose the UI Tree:** Send a simplified version of the component tree to the OS.
2.  **Assign Roles:** Map components to OS roles like `Button`, `Checkbox`, or `Heading`.
3.  **Semantic Metadata:** Provide descriptions, labels, and state information (e.g., "Checked") to Screen Readers.

---

## 🔊 Screen Reader Support

By providing a native accessibility bridge, Rupaui applications become usable via:
- **VoiceOver** (MacOS / iOS)
- **NVDA & JAWS** (Windows)
- **TalkBack** (Android)
- **Orca** (Linux)

---

## 🛠️ Internal Module Reference
- `src/core/component.rs`: `Accessibility` struct and metadata.
- `src/platform/mod.rs`: Initializing the AccessKit adapter within the Winit loop.
- `src/style/utilities/accessibility.rs`: Definitions for semantic roles and aria-like attributes.
