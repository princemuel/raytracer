use cucumber::Parameter;
use derive_more::{Deref, FromStr};

pub fn _to_float<S: AsRef<str>>(s: S) -> f64 {
    s.as_ref()
        .trim()
        .parse::<f64>()
        .expect("failed to parse number from feature")
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Deref, FromStr, Parameter)]
#[param(name = "f64", regex = r"[-+]?(?:\d+(?:\.\d*)?|\.\d+)(?:[eE][-+]?\d+)?")]
pub struct Float64(f64);
