use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use once_cell::sync::Lazy;
use crate::utils::color::Color;
use crate::utils::style::Style;
use crate::utils::typography::{FontWeight, FontStyle};
use crate::utils::border::BorderStyle;

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum ColorMode { #[default] Dark, Light, System }

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Variant { Primary, Secondary, Success, Danger, Warning, Info, Light, Dark, Link }

#[derive(Debug, Clone)]
pub struct TypographyConfig {
    pub base_size: f32,
    pub scale_ratio: f32,
    pub default_weight: FontWeight,
    pub font_stacks: HashMap<String, Vec<String>>,
}

#[derive(Debug, Clone)]
pub struct BorderConfig {
    pub width: f32,
    pub style: BorderStyle,
    pub radius: f32,
}

#[derive(Debug, Clone)]
pub struct Theme {
    pub mode: ColorMode,
    pub colors: HashMap<String, Color>,
    pub variants: HashMap<Variant, Color>,
    pub spacing: HashMap<String, f32>,
    pub typography: TypographyConfig,
    pub borders: BorderConfig,
    pub component_presets: HashMap<String, Style>,
}

static CURRENT_THEME: Lazy<Arc<RwLock<Theme>>> = Lazy::new(|| {
    Arc::new(RwLock::new(Theme::default_artisan()))
});

impl Theme {
    pub fn current() -> Theme { CURRENT_THEME.read().unwrap().clone() }
    pub fn update<F>(f: F) where F: FnOnce(&mut Theme) {
        let mut theme = CURRENT_THEME.write().unwrap();
        f(&mut theme);
    }

    /// Applies global defaults (borders, typography) to a Style object.
    pub fn apply_defaults(&self, style: &mut Style) {
        style.border.width = self.borders.width;
        style.border.style = self.borders.style.clone();
        style.rounding = crate::utils::border::Rounding::all(self.borders.radius);
        style.typography.size = Some(self.typography.base_size);
        style.typography.weight = self.typography.default_weight.clone();
    }

    pub fn default_artisan() -> Self {
        let mut colors = HashMap::new();
        let mut variants = HashMap::new();
        let mut font_stacks = HashMap::new();
        font_stacks.insert("sans".into(), vec!["Inter".into(), "system-ui".into(), "sans-serif".into()]);
        variants.insert(Variant::Primary, Color::Indigo(600));
        variants.insert(Variant::Secondary, Color::Slate(600));
        variants.insert(Variant::Success, Color::Emerald(500));
        variants.insert(Variant::Danger, Color::Red(500));
        colors.insert("background".into(), Color::Slate(950));
        colors.insert("text".into(), Color::Slate(50));

        Self {
            mode: ColorMode::Dark,
            colors,
            variants,
            spacing: HashMap::new(),
            typography: TypographyConfig {
                base_size: 16.0,
                scale_ratio: 1.25,
                default_weight: FontWeight::Regular,
                font_stacks,
            },
            borders: BorderConfig { width: 1.0, style: BorderStyle::Solid, radius: 4.0 },
            component_presets: HashMap::new(),
        }
    }

    pub fn variant(v: Variant) -> Color {
        let theme = CURRENT_THEME.read().unwrap();
        theme.variants.get(&v).cloned().unwrap_or(Color::Slate(500))
    }
}
