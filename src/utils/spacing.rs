#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Spacing {
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
    pub left: f32,
}

impl Spacing {
    pub fn all(val: f32) -> Self {
        Self { top: val, right: val, bottom: val, left: val }
    }

    pub fn x(mut self, val: f32) -> Self {
        self.left = val;
        self.right = val;
        self
    }

    pub fn y(mut self, val: f32) -> Self {
        self.top = val;
        self.bottom = val;
        self
    }
}

pub trait IntoSpacing {
    fn into_spacing(self) -> Spacing;
}

impl IntoSpacing for f32 {
    fn into_spacing(self) -> Spacing {
        Spacing::all(self)
    }
}

impl IntoSpacing for (f32, f32) {
    fn into_spacing(self) -> Spacing {
        Spacing { top: self.0, right: self.1, bottom: self.0, left: self.1 }
    }
}

impl IntoSpacing for (f32, f32, f32, f32) {
    fn into_spacing(self) -> Spacing {
        Spacing { top: self.0, right: self.1, bottom: self.2, left: self.3 }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spacing_all() {
        let s = Spacing::all(10.0);
        assert_eq!(s.top, 10.0);
        assert_eq!(s.left, 10.0);
    }

    #[test]
    fn test_spacing_tuple_2() {
        let s = (10.0, 20.0).into_spacing();
        assert_eq!(s.top, 10.0);
        assert_eq!(s.right, 20.0);
        assert_eq!(s.bottom, 10.0);
        assert_eq!(s.left, 20.0);
    }

    #[test]
    fn test_spacing_tuple_4() {
        let s = (1.0, 2.0, 3.0, 4.0).into_spacing();
        assert_eq!(s.top, 1.0);
        assert_eq!(s.right, 2.0);
        assert_eq!(s.bottom, 3.0);
        assert_eq!(s.left, 4.0);
    }
}
