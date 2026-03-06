# Module: Platform Orchestrator (`mod.rs`) 🏛️

This is the entry point of Layer 1. It acts as the grand conductor that decides which hardware abstraction (GUI or TUI) to boot based on the environment and compilation flags.

---

## 🏗️ Core Responsibilities

1.  **App Struct:** The primary builder used by end-users to bootstrap their application.
2.  **PlatformCore Struct:** The shared internal heart of all platforms, managing root components, layout calculations, and cursor state.
3.  **Bootstrap Logic:** Orchestrates the injection of plugins and the initial theme setup before the platform runner takes over.
4.  **Cross-Platform Redraw:** Provides the global `request_redraw()` function which communicates with the active backend proxy.
5.  **Platform Abstraction:** Defines the `PlatformRunner` trait that all backends must implement to ensure a uniform lifecycle.

---

## 🗝️ Key API Elements

### `struct App`
The user-facing API for defining the application name and root component.
- `.new(name)`: Creates a new instance.
- `.root(component)`: Attaches the top-level UI element.
- `.run()`: Starts the GUI backend (Default).
- `.run_tui()`: Starts the Terminal backend.

### `struct PlatformCore`
The internal core used via **Composition** in every platform runner.
- `compute_layout(w, h)`: Common logic to update the Taffy tree.

### `trait PlatformRunner`
The internal contract for backends:
- `initialize()`: Hardware handshake.
- `run()`: Starts the event loop.
- `request_redraw()`: Signals the need for a new frame.

---

## 🔄 Interaction
- **L1 -> L8:** Orchestrates the high-level application flow.
- **L1 -> L9:** Injects Design System defaults during bootstrap.
