#[derive(Clone, Debug, PartialEq)]
pub enum Color {
    Rgba(f32, f32, f32, f32),
    Oklch(f32, f32, f32, f32), // Lightness, Chroma, Hue, Alpha
    Semantic(String, Option<f32>),
    Transparent,
}

impl Color {
    pub fn to_rgba(&self) -> [f32; 4] {
        match self {
            Color::Rgba(r, g, b, a) => [*r, *g, *b, *a],
            Color::Oklch(l, c, h, a) => {
                let rgba = Self::oklch_to_rgba(*l, *c, *h);
                [rgba.0, rgba.1, rgba.2, *a]
            },
            Color::Semantic(name, alpha) => {
                // Fallback for semantic colors if theme is not available
                match name.as_str() {
                    "primary" => [0.39, 0.45, 1.0, alpha.unwrap_or(1.0)],
                    "background" => [0.05, 0.05, 0.05, alpha.unwrap_or(1.0)],
                    "surface" => [0.12, 0.12, 0.12, alpha.unwrap_or(1.0)],
                    _ => [1.0, 1.0, 1.0, alpha.unwrap_or(1.0)],
                }
            },
            Color::Transparent => [0.0, 0.0, 0.0, 0.0],
        }
    }

    /// Approximate OKLCH to RGBA conversion
    fn oklch_to_rgba(l: f32, c: f32, h_deg: f32) -> (f32, f32, f32) {
        let h = h_deg.to_radians();
        let a = c * h.cos();
        let b = c * h.sin();

        let l_ = l + 0.3963377774 * a + 0.2158037573 * b;
        let m_ = l - 0.1055613458 * a - 0.0638541728 * b;
        let s_ = l - 0.0894841775 * a - 1.2914855480 * b;

        let l_r = l_ * l_ * l_;
        let m_r = m_ * m_ * m_;
        let s_r = s_ * s_ * s_;

        let r = 4.0767416621 * l_r - 3.3077115913 * m_r + 0.2309699292 * s_r;
        let g = -1.2684380046 * l_r + 2.6097574011 * m_r - 0.3413193965 * s_r;
        let b_ = -0.0041960863 * l_r - 0.7034186147 * m_r + 1.7076147010 * s_r;

        (r.clamp(0.0, 1.0), g.clamp(0.0, 1.0), b_.clamp(0.0, 1.0))
    }

    pub fn white(a: f32) -> Self { Color::Rgba(1.0, 1.0, 1.0, a) }
    pub fn black(a: f32) -> Self { Color::Rgba(0.0, 0.0, 0.0, a) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rgba_to_array() {
        let c = Color::Rgba(1.0, 0.5, 0.2, 1.0);
        assert_eq!(c.to_rgba(), [1.0, 0.5, 0.2, 1.0]);
    }

    #[test]
    fn test_oklch_to_rgba() {
        // White in OKLCH: L=1.0, C=0.0
        let white = Color::Oklch(1.0, 0.0, 0.0, 1.0);
        let rgba = white.to_rgba();
        assert!(rgba[0] > 0.99 && rgba[1] > 0.99 && rgba[2] > 0.99);
        
        // Black in OKLCH: L=0.0
        let black = Color::Oklch(0.0, 0.0, 0.0, 1.0);
        let rgba_b = black.to_rgba();
        assert!(rgba_b[0] < 0.01 && rgba_b[1] < 0.01 && rgba_b[2] < 0.01);
    }

    #[test]
    fn test_transparent() {
        let t = Color::Transparent;
        assert_eq!(t.to_rgba(), [0.0, 0.0, 0.0, 0.0]);
    }
}

impl From<&str> for Color {
    fn from(s: &str) -> Self {
        Color::Semantic(s.to_string(), None)
    }
}
