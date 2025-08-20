use std::f64::consts::PI;
use std::ops::Deref;

// pub const EPSILON: f64 = f64::EPSILON;
pub const EPSILON: f64 = 1e-5; // or e-4
const DEGREE_TO_RADIAN: f64 = PI / 180.0;
const RADIAN_TO_DEGREE: f64 = 180.0 / PI;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Degree(pub f64);
impl Degree {
    pub fn new(value: f64) -> Self { Self(value) }

    /// Normalize degree to [0, 360) range
    pub fn normalize(&self) -> Self { Self(self.rem_euclid(360.0)) }
}

impl Deref for Degree {
    type Target = f64;

    fn deref(&self) -> &Self::Target { &self.0 }
}
impl From<Radian> for Degree {
    fn from(value: Radian) -> Self { Self(value.0 * RADIAN_TO_DEGREE) }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Radian(pub f64);
impl Radian {
    pub fn new(value: f64) -> Self { Self(value) }

    /// Normalize radian to [0, 2π) range
    pub fn normalize(&self) -> Self { Self(self.rem_euclid(2.0 * PI)) }
}

impl Deref for Radian {
    type Target = f64;

    fn deref(&self) -> &Self::Target { &self.0 }
}
impl From<Degree> for Radian {
    fn from(value: Degree) -> Self { Self(value.0 * DEGREE_TO_RADIAN) }
}

pub fn approximately(a: f64, b: f64) -> bool { (a - b).abs() < EPSILON }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn approximately_returns_true_when_approximately_equal() {
        assert!(approximately(1.0, 1.0));
        assert!(approximately(1.0 + 0.000001, 1.0));
        assert!(approximately(1.0 + 1e-11, 1.0));
        assert!(approximately(1.0, 10.0 / 10.0));

        assert!(approximately(0.0, 1e-9));
        // assert!(approximately(PI, 22.0 / 7.0));
    }

    #[test]
    fn approximately_returns_false_when_not_approximately_equal() {
        assert!(!approximately(1.0 + 0.001, 1.0));
        assert!(!approximately(1.0, 1.1));
    }

    #[test]
    fn test_degree_deref() {
        let deg = Degree::new(45.0);
        assert_eq!(*deg, 45.0);
    }

    #[test]
    fn test_radian_deref() {
        let rad = Radian::new(PI / 4.0);
        assert!(approximately(*rad, PI / 4.0));
    }

    #[test]
    fn test_degree_to_radian_conversion() {
        let deg = Degree::new(180.0);
        let rad: Radian = deg.into();
        assert!(approximately(*rad, PI));

        let deg90 = Degree::new(90.0);
        let rad90: Radian = deg90.into();
        assert!(approximately(*rad90, PI / 2.0));

        let deg01 = Degree::new(1.0);
        let rad01: Radian = deg01.into();
        assert!(approximately(*rad01, 0.0174533));
    }

    #[test]
    fn test_radian_to_degree_conversion() {
        let rad = Radian::new(PI);
        let deg: Degree = rad.into();
        assert!(approximately(*deg, 180.0));

        let rad_quarter = Radian::new(PI / 2.0);
        let deg_quarter: Degree = rad_quarter.into();
        assert!(approximately(*deg_quarter, 90.0));

        let rad01 = Radian::new(1.0);
        let deg01: Degree = rad01.into();
        assert!(approximately(*deg01, 57.29577));
    }

    #[test]
    fn test_normalization() {
        let deg_over = Degree::new(450.0);
        assert!(approximately(*deg_over.normalize(), 90.0));

        let deg_negative = Degree::new(-90.0);
        assert!(approximately(*deg_negative.normalize(), 270.0));

        let rad_over = Radian::new(3.0 * PI);
        assert!(approximately(*rad_over.normalize(), PI));

        let rad_negative = Radian::new(-PI / 2.0);
        assert!(approximately(*rad_negative.normalize(), 3.0 * PI / 2.0));
    }

    #[test]
    fn test_round_trip_conversions() {
        let original_deg = Degree::new(127.5);
        let converted: Degree = Radian::from(original_deg).into();
        assert!(approximately(*original_deg, *converted));

        let original_rad = Radian::new(PI / 3.0);
        let converted: Radian = Degree::from(original_rad).into();
        assert!(approximately(*original_rad, *converted));
    }

    #[test]
    fn test_edge_cases() {
        let zero_deg: Radian = Degree::new(0.0).into();
        assert!(approximately(*zero_deg, 0.0));

        let full_circle: Radian = Degree::new(360.0).into();
        assert!(approximately(*full_circle, 2.0 * PI));
    }
}
