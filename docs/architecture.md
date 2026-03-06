# Rupaui Architecture 🏛️

This document outlines the multi-layered architecture of the Rupaui framework, following international software engineering standards for UI systems, high-performance graphics, and reactive state management.

---

## 🏗️ Layered Stack Overview

Rupaui is built on a 9-layer stack, descending from the user-facing composition down to the hardware execution. The architecture is designed to be **Backend-Agnostic**, allowing the same UI logic to run on GPUs (GUI) or Terminals (TUI).

### Layer 9: Design System, UI Utilities & Ecosystem (DNA Visual)
*   **Purpose:** Standardizes aesthetics and provides the functional "Utility-First" API.
*   **Components:** Global Themes, Procedural Colors (OKLCH), Unified Scale, and functional modifiers.
*   **Internationalization (i18n):** Localization providers and Right-to-Left (RTL) layout support.

### Layer 8: Application Composition Layer
*   **Purpose:** High-level orchestration where developers assemble the UI.
*   **Components:** Application views, routing logic, and business logic integration.

### Layer 7: Semantic Component Layer (The Artisans)
*   **Purpose:** Meaningful UI elements with pre-defined semantic behavior.
*   **Components:** `Button`, `Input`, `Navbar`, `Modal`, `Alert`, `Table`.
*   **Accessibility (A11y):** Components implement semantic roles (e.g., `Role::Button`) for screen readers.

### Layer 6: Primitive UI Layer (Atomic)
*   **Purpose:** Atomic building blocks like `Div`, `Flex`, and `Grid`.
*   **Characteristics:** Structural containment without inherent semantic meaning.

### Layer 5: Component Architecture Layer (Logic & View)
*   **Purpose:** Enforces Separation of Concerns (SOC) via the Brain/Body split.
*   **Logic (Brain):** Reactive state, data validation, and Semantic Event Handlers.
*   **View (Body):** Styling metadata, layout nodes, and paint instructions.
*   **Interactivity Engine:** Focus Management, Event Dispatcher, and Hit-Testing.

### Layer 4: Reactivity & State Layer
*   **Purpose:** Manages fine-grained data flow and change propagation.
*   **Core:** `Signal<T>` and `Memo<T>`.
*   **Animation Engine:** Time-based signal interpolation (Tweens, Springs).

### Layer 3: Layout Engine Layer (Geometric)
*   **Purpose:** Resolves element sizing and positioning via **Taffy**.
*   **Agnosticism:** Calculates coordinates in a generic unit system (mapped to Pixels for GUI or Cells for TUI).

### Layer 2: Rendering Engine Layer (Multi-Backend)
*   **Purpose:** Translates geometric data into visual instructions for a specific backend.
*   **GUI Backend (WGPU):** 2D Batcher, WGSL Shaders, Glyphon Typography Engine.
*   **TUI Backend (Terminal):** ANSI Character Painter, Cell-based Buffer Management.
*   **Headless Backend:** Virtual buffer for automated testing and server-side rendering.

### Layer 1: Hardware Abstraction Layer (HAL)
*   **Purpose:** Native interface with the environment (OS or Terminal).
*   **GUI HAL:** WGPU Device & Winit Windowing.
*   **TUI HAL:** Terminal Raw Mode & Input Capture (via Crossterm/Termion).
*   **A11y Bridge:** Integration with OS Accessibility services (AccessKit).

---

## 🔄 The "Agnostic Bridge" Principle

The power of Rupaui lies in the boundary between **Layer 3** and **Layer 2**. 
- Layers 3 through 9 are **Platform-Agnostic**: They describe *what* the UI is and *how* it behaves.
- Layers 1 and 2 are **Platform-Specific**: They handle *how* the UI is actually drawn and *where* input comes from.

This modularity allows you to write a `Button` once and render it as a hardware-accelerated 4K button or a `[ Click Me ]` text string in a Linux terminal.

---

## 🛠 Engineering Principles

1.  **Utility-First, Semantic-Support:** Style Components (L7) using Utilities (L9).
2.  **Strict Separation of Concerns (SOC):** Logic (L5) must never depend on the Rendering Engine (L2).
3.  **Reactive Integrity:** UI updates must be a pure side-effect of Signal (L4) changes.
4.  **Accessible by Default:** Every component must expose metadata regardless of the backend (GUI/TUI).
5.  **Multi-Backend Scalability:** Adding support for new platforms (e.g., VR/AR) only requires implementing Layers 1 and 2.
