# Rupaui Documentation Index 🎨

Welcome to the **Rupaui Open Blueprint**. This documentation is structured to mirror the 9-layer architecture of the framework, providing total transparency from hardware abstraction to design system composition.

---

## 🏁 Getting Started
- [Project Overview](./overview.md) - Vision, key features, and tech stack.
- [Philosophy](./philosophy.md) - Design principles and the 5 Artisan Pillars.
- [Architecture Overview](./architecture.md) - Conceptual map of the 9-layer stack.

---

## 🏗️ The 9-Layer Blueprint

### [01] Hardware Abstraction (HAL)
- [Platform Orchestrator](./01-hal/platform-orchestrator.md) - `mod.rs` (App entry & bootstrap)
- [Input Events](./01-hal/input-events.md) - `events.rs` (Universal schema)
- [Event Dispatcher](./01-hal/event-dispatcher.md) - `dispatcher.rs` (Hit-testing & propagation)
- [GUI Runner](./01-hal/gui-runner.md) - `gui/mod.rs` (Winit & frame loop)
- [GUI Window](./01-hal/gui-window.md) - `gui/window.rs` (OS window management)
- [TUI Runner](./01-hal/tui-runner.md) - `tui/mod.rs` (Crossterm & terminal loop)

### [02] Rendering Engine
- [Text Engine](./02-rendering/text-engine.md) - Accelerated typography via Glyphon.
- [Geometry](./02-rendering/geometry.md) - Backgrounds, borders, and SDF shapes.
- [Effects & Shadows](./02-rendering/effects.md) - Real-time GPU visual effects.
- [Filters](./02-rendering/filters.md) - Color and image processing shaders.
- [Motion](./02-rendering/motion.md) - Transforms and GPU animations.
- [Vector Math](./02-rendering/vector-math.md) - Core geometric primitives.

### [03] Layout Engine
- [Flexbox & Grid](./03-layout/flexbox-grid.md) - Resolution via Taffy algorithm.

### [04] Reactivity Layer
- [Reactivity System](./04-reactivity/reactivity.md) - Signals, Memos, and change propagation.

### [05] Component Architecture
- [Attributes](./05-architecture/attributes.md) - Component metadata and metadata handling.
- [Logic & View Pattern](./architecture.md#layer-5-component-architecture-layer-logic--view) - The core SOC standard.

### [06] UI Primitives
- [Div](./06-primitives/div.md) - The atomic layout block.

### [07] Semantic Components
- [Elements Library](./07-components/elements.md) - Standard UI kit (Buttons, Badge, etc).
- [Forms & Input](./07-components/forms.md) - Reactive user input controls.
- [Navigation](./07-components/navigation.md) - Navbar, Tabs, and Breadcrumbs.
- [Content & Layout](./07-components/layout.md) - Sections and semantic grouping.
- [Data Tables](./07-components/content.md) - Structured data displays.
- [Vector Graphics](./07-components/svg.md) - SVG paths and icons.
- [Theme Switcher](./07-components/theme-switcher.md) - Dynamic mode toggling.
- [Identity](./07-components/brand.md) - Branding and logos.

### [08] Composition Layer
- [Control Flow](./08-composition/control-flow.md) - Conditional rendering and loops.
- [Viewports](./08-composition/viewports.md) - Infinite canvas, zoom, and pan.

### [09] Ecosystem & Design System
- [Styling API](./09-ecosystem/styling-api.md) - Functional, utility-first styling.
- [Theme Engine](./09-ecosystem/theme-engine.md) - DNA Visual and global themes.
- [Artisan Scale](./09-ecosystem/scale-system.md) - The 10-step unified scale.
- [Modular Styling](./09-ecosystem/modular-styling.md) - Style composition with tuples.
- [Interactivity](./09-ecosystem/interactivity.md) - Events, hover, and active states.
- [Plugins](./09-ecosystem/plugins.md) - Extending the framework.
- [Extending](./09-ecosystem/extending.md) - Guide for library developers.
- [Helpers](./09-ecosystem/helpers.md) - Design system utility helpers.
- [Variants](./09-ecosystem/variants.md) - Semantic variant system (Primary, Danger, etc).
