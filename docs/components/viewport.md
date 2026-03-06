# Viewport Component 🖼️

The **Viewport** component is an advanced container that provides a "window" into content that can be panned and zoomed. It is ideal for image editors, maps, or large canvas applications.

## 🌟 Key Features

- **Camera Panning**: Move content freely by dragging the mouse.
- **Dynamic Zooming**: Zoom in or out using the mouse wheel.
- **Reactive State**: Offset position and zoom level are stored as reactive `Signal` objects.
- **Control Toggles**: Ability to lock zoom or pan functionality independently.

## 🛠️ Basic Usage

```rust
use rupaui::elements::{Viewport, SvgCanvas};

let my_canvas = SvgCanvas::new();

Viewport::new(Box::new(my_canvas))
    .zoomable(true)
    .pannable(true)
```

## ⚙️ Attributes (Props)

| Attribute | Type | Default | Description |
| :--- | :--- | :--- | :--- |
| `zoomable` | `bool` | `true` | Enables or disables zooming via mouse wheel. |
| `pannable` | `bool` | `true` | Enables or disables panning via mouse drag. |
| `id` | `String` | Auto | Unique identity for the component. |

## 📐 Internal Logic

The Viewport works by manipulating the **World Matrix** on the `Renderer`. When the Viewport begins its drawing phase (`paint`), it will:
1. Save the current global camera state.
2. Inject its own `camera_offset` and `camera_zoom` into the Renderer.
3. Draw its content using the new world coordinates.
4. Restore the global camera state after finishing.

## ⌨️ Mouse Interaction

- **Mouse Wheel**: Adjusts the zoom level (0.1x to 10.0x).
- **Mouse Drag**: Offsets the camera position based on the mouse movement delta.
- **Click**: Click events are propagated to the content within the viewport.
