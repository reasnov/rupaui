# Theming & Global Configuration

The `Theme` module is the **Authoritative Configuration Engine** for Rupaui. It defines the "Visual DNA" of your application by setting factory defaults for all components.

## 🧬 Structural Defaults

You can define how components are born by configuring global border and typography standards.

```rust
Theme::update(|t| {
    t.borders.width = 2.0;      // All new components will have 2px borders
    t.borders.radius = 8.0;     // All new components will be rounded by 8px
    t.typography.base_size = 18.0; // Standardize font size
});
```

---

## 🔤 Font Stacks & Fallbacks

Rupaui supports semantic font stacks, ensuring that your typography remains consistent even if a specific font fails to load.

```rust
Theme::update(|t| {
    t.typography.font_stacks.insert("sans".into(), vec![
        "Inter".into(), 
        "Roboto".into(), 
        "sans-serif".into()
    ]);
});
```

Standard stacks included: `sans`, `serif`, `mono`.

---

## 🏷 Semantic Variants

Variants (`Primary`, `Success`, `Danger`, etc.) are mapped to specific colors in the Artisan Palette. Changing a variant color in the theme updates all components using that variant.

```rust
Theme::update(|t| {
    // Change the global "Primary" identity to Rose 600
    t.variants.insert(Variant::Primary, Color::Rose(600));
});
```

---

## 🏗 Component Presets

For complex configurations, you can store full `Style` objects as presets.

```rust
let glass = Style::new().backdrop_blur(10.0).bg(Color::White(0.1));
Theme::update(|t| {
    t.component_presets.insert("glass-panel".into(), glass);
});

// Usage
Div::new().style(Theme::style("glass-panel"));
```

## 🚀 Performance
Theme data is stored in an `Arc<RwLock>`, allowing for safe, concurrent read access across multiple UI threads while enabling dynamic updates (like live theme switching) at runtime.
