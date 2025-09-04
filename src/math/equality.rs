pub trait ApproxEqual<Rhs = Self> {
    fn is_equal(&self, other: Rhs) -> bool;
}

impl ApproxEqual<f32> for f32 {
    #[inline]
    fn is_equal(&self, other: Self) -> bool { (self - other).abs() < f32::EPSILON }
}

impl ApproxEqual<f64> for f64 {
    #[inline]
    fn is_equal(&self, other: Self) -> bool { (self - other).abs() < f64::EPSILON }
}
