# Platform Support (Native vs Web) 💻🌐

Rupaui is built to be truly cross-platform. While the higher layers (3-9) are platform-agnostic, Layer 1 provides specific implementations for Native Desktop and WebAssembly (Wasm).

---

## 🖥️ Native Target (Desktop)

On Windows, Linux, and MacOS, Rupaui compiles directly to machine code.
- **Backend:** WGPU uses Vulkan, DX12, or Metal.
- **Windowing:** Native OS windows managed by Winit.
- **Performance:** Direct access to GPU memory and multi-threading.

---

## 🌐 Web Target (Wasm)

Rupaui can run in any modern browser via WebAssembly and WebGPU (or WebGL2 fallback).
- **Surface:** The framework binds to an HTML5 `<canvas>` element.
- **Loop:** Instead of a blocking event loop, it uses `requestAnimationFrame` to sync with the browser's refresh rate.

---

## ⌨️ Terminal Target (TUI)

Rupaui supports running directly in terminal emulators.
- **Surface:** A grid-based cell buffer.
- **Input:** Standard `stdin` stream with ANSI escape sequence decoding.
- **Portability:** Works over SSH and in minimal CLI environments (e.g., Linux VT, headless servers).
- **Integration:** Rupaui can coexist with other JavaScript libraries and CSS on the same page.

---

## 🏗️ Future Platforms (Mobile)

Support for Android and iOS is part of our long-term roadmap. Since both Winit and WGPU support mobile backends (GLES and Vulkan/Metal), Layer 1 is already structurally prepared for mobile expansion.

---

## 🛠️ Internal Module Reference
- `Cargo.toml`: Features for `web-sys` and `wasm-bindgen`.
- `src/platform/dispatcher.rs`: Specific event mapping for web vs native.
