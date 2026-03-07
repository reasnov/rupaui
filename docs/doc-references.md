# Rupaui Documentation Index 📚

Welcome to the **Rupaui Open Blueprint**. This document serves as the central directory for all technical documentation, organized by the framework's 9-layer architecture.

---

## 🏁 Getting Started & Vision
Foundational documents for understanding Rupaui's purpose and standards.

| Document | Description |
| :--- | :--- |
| **[Project Overview](./overview.md)** | High-level vision, key features, and core tech stack. |
| **[Philosophy](./philosophy.md)** | The "Artisan Pillars" and the reasoning behind Rupaui's design. |
| **[Engineering Standards](./engineering-standards.md)** | Mandatory coding conventions, naming rules, and SOC mandates. |
| **[Architecture Overview](./architecture.md)** | A conceptual map of the 9-layer infrastructure. |
| **[Platform References](./platform-references.md)** | Standardized entry-points for Desktop, Web, Terminal, and Mobile. |

---

## 🏗️ Technical References
Comprehensive indices for components, utilities, and infrastructure.

| Document | Description |
| :--- | :--- |
| **[Component Reference](./component-references.md)** | Index of all UI elements from Layout to Overlays. |
| **[Support Reference](./support-references.md)** | Index of Math, Reactivity, A11y, and Error handling utilities. |

---

## 🔬 The 9-Layer Blueprint
Detailed technical specifications for every internal framework module.

### [01] Hardware Abstraction (HAL)
| Module | Description |
| :--- | :--- |
| **[Platform Orchestrator](./01-hal/platform-orchestrator.md)** | App lifecycle management and the Agnostik Bridge. |
| **[Input Events](./01-hal/input-events.md)** | Universal event schema across all platforms. |
| **[Input Dispatcher](./01-hal/input-dispatcher.md)** | Event normalization and hit-testing logic. |
| **[Desktop Runner](./01-hal/desktop-runner.md)** | Winit & WGPU shell for Desktop (macOS, Win, Linux). |
| **[Terminal Runner](./01-hal/terminal-runner.md)** | Crossterm shell for CLI/TUI applications. |
| **[Web Runner](./01-hal/web-runner.md)** | WASM & Canvas shell for browser environments. |
| **[Mobile Runner](./01-hal/mobile-runner.md)** | Android & iOS shell with lifecycle management. |

### [02] Rendering Engine
| Module | Description |
| :--- | :--- |
| **[Renderer Interface](./02-rendering/renderer-interface.md)** | The universal contract for all visual backends. |
| **[GUI Backend](./02-rendering/gui-backend.md)** | Aggregation of WGPU-specific rendering logic. |
| **[TUI Renderer](./02-rendering/tui-renderer.md)** | ANSI/Character-grid based terminal painter. |

### [03] Geometric Scene Layer
| Module | Description |
| :--- | :--- |
| **[Scene Core](./03-layout/scene-core.md)** | The spatial Single Source of Truth (SSOT). |
| **[Layout Engine](./03-layout/layout-engine.md)** | Integration with Taffy for Flexbox and Grid. |
| **[Scene Node](./03-layout/scene-node.md)** | Platform-agnostic handle for geometric objects. |

### [04] Reactivity Layer
| Module | Description |
| :--- | :--- |
| **[Signals & Memos](./04-reactivity/signals.md)** | The reactive nucleus of the framework. |
| **[Fine-Grained Updates](./04-reactivity/fine-grained-updates.md)** | Performance strategy for targeted UI refreshes. |

### [05] Component Architecture
| Module | Description |
| :--- | :--- |
| **[Component Trait](./05-architecture/component-trait.md)** | The core contract for all UI elements. |
| **[View Core](./05-architecture/view-core.md)** | Anatomical standard for component infrastructure. |
| **[Logic & View Pattern](./05-architecture/logic-and-view.md)** | Strict Separation of Concerns (SOC) standard. |
| **[Module Standard](./05-architecture/module-standard.md)** | Directory and naming conventions for elements. |

### [06] UI Primitives (Atomic)
| Module | Description |
| :--- | :--- |
| **[Primitive Design](./06-primitives/primitive-design.md)** | The design standard for Layer 6 atomic blocks. |
| **[Overlay](./06-primitives/overlay.md)** | Absolute positioning and z-index management. |

### [07] Semantic Components (Artisan)
| Module | Description |
| :--- | :--- |
| **[Component Design](./07-components/component-design.md)** | The design standard for Layer 7 semantic elements. |
| **[Theme Control](./07-components/theme-switcher.md)** | Standardized Light/Dark mode switching logic. |

### [08] Composition Layer
| Module | Description |
| :--- | :--- |
| **[App Bootstrap](./08-composition/app-bootstrap.md)** | Orchestration logic from App::new() to Runner::run(). |
| **[Control Flow](./08-composition/control-flow.md)** | Logic components for conditional and list rendering. |

### [09] Ecosystem & Visual DNA
| Module | Description |
| :--- | :--- |
| **[Styling API](./09-ecosystem/styling-api.md)** | Functional "Utility-First" API reference. |
| **[Theme Engine](./09-ecosystem/theme-engine.md)** | DNA Visual standard for consistent aesthetics. |
| **[Color Math](./09-ecosystem/color-math.md)** | OKLCH standards for perceptual uniformity. |
