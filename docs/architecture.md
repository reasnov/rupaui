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

### Layer 3: Geometric Scene Layer
*   **Purpose:** Manages the visual tree (Scene Graph) and resolves spatial properties.
*   **Scene Graph:** Orchestrates the parent-child hierarchy and lifecycle of UI elements.
*   **Geometric Resolution:** Utilizes **Taffy** to transform abstract layout rules into precise coordinates.
*   **Spatial Awareness:** Maps global coordinates to local component spaces, essential for hit-testing and clipping.

### Layer 2: Rendering Engine Layer (Multi-Backend)
*   **Purpose:** A modular visual pipeline that translates geometric data into specific backend instructions.
*   **Composition Core (`RenderCore`):** Shared internal state for all renderers (Viewport, Camera, Logical Size).
*   **Universal Interface (`trait Renderer`):** Agnostic contract for drawing primitives, text, and managing clipping.
*   **Sub-systems:** GUI Renderer (WGPU), TUI Renderer (Terminal), and Headless testing buffer.

### Layer 1: Hardware Abstraction Layer (HAL)
*   **Purpose:** Native interface with the environment (OS or Terminal).
*   **Composition Core (`PlatformCore`):** Shared state for all platforms (App root, Scene reference, Cursor).
*   **Standardized Lifecycle (`trait PlatformRunner`):** Unified initialization and execution loop.
*   **GUI HAL:** WGPU Device & Winit Windowing.
*   **TUI HAL:** Terminal Raw Mode & Input Capture.

---

## 🔄 Architectural Principles

1.  **Composition over Inheritance:** High-level platform shells (GUI/TUI) compose low-level cores (`PlatformCore`, `RenderCore`) to eliminate redundancy.
2.  **Agnostic Bridge:** Layers 3-9 are platform-independent; only Layers 1-2 contain hardware-specific code.
3.  **Universal Language:** Standardized enums like `InputEvent` and traits like `Renderer` ensure the framework speaks a single language regardless of the output target.
4.  **Strict SOC:** Logic (Brain) never touches Rendering (Body).
5.  **Spatial Integrity:** Layer 3 acts as the single source of truth for all geometry and structural relationships.
6.  **Reactive Integrity:** UI updates are side-effects of Signal changes, managed by the fine-grained reactivity layer.
