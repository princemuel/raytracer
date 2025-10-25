use crate::cmp::epsilon::EPSILON;
use crate::math;

pub const fn is_equal(a: f64, b: f64) -> bool {
    // Fast path: exact equality (handles infinities, zeros, and exact matches)
    if a == b {
        return true;
    }

    // Handle NaN cases - NaN should never equal anything
    if a.is_nan() || b.is_nan() {
        return false;
    }

    // Handle infinite cases that aren't exactly equal
    if a.is_infinite() || b.is_infinite() {
        return false; // Different infinities or one infinite, one finite
    }

    let diff = math::abs(a - b);

    // For very small numbers near zero, use absolute epsilon
    if math::max(a, math::abs(b)) < 1.0 {
        return diff < EPSILON;
    }

    // For larger numbers, use relative epsilon to maintain precision
    // This prevents issues when comparing large coordinate values
    let relative_epsilon = EPSILON * math::max(a, math::abs(b));

    // Use the larger of absolute and relative epsilon
    // This handles edge cases around 1.0 and ensures consistent behavior
    diff < math::max(EPSILON, relative_epsilon)
}
