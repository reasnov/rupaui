# Rupaui Philosophy

**Rupaui** is designed with one primary vision: to give developers full control to build reactive, high-performance interfaces with high visual integrity (DNA Visual).

## 1. Utility-First, Semantic-Support
We believe functional utilities (like Tailwind) are great for speed, but semantic structures (like Bootstrap) are essential for long-term maintenance. Rupaui combines both.

## 2. Signal-Based Reactivity
Rupaui uses a **Signal** and **Memo** system for state management. Every state change automatically triggers layout updates and redraws only where necessary, ensuring the application stays at 60+ FPS.

## 3. Hardware-Accelerated (WGPU)
Unlike CPU-based UI frameworks, Rupaui is built directly on top of **WGPU**. This means every rectangle, text, and image is drawn using the GPU, allowing advanced visual effects like SDF Rounded Corners and shader filters without taxing the main thread.

## 4. Industrial Layout Engine (Taffy)
We use **Taffy**, an industry-standard layout engine that fully supports Flexbox and CSS Grid. This guarantees consistent layout behavior across different platforms (Native & Web).

## 5. Unified Scale
Visual integrity is maintained through a 10-step scaling system (`Xs` to `6xl`) applied consistently across margins, paddings, component sizes, and breakpoints.
