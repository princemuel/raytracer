#[macro_export]
macro_rules! impl_op {
    ($t:ident, $trt:ident, $func:ident) => {
        impl core::ops::$trt<&Self> for $t {
            type Output = Self;

            #[inline]
            fn $func(self, rhs: &Self) -> Self::Output { self.$func(*rhs) }
        }
        impl core::ops::$trt<&$t> for &$t {
            type Output = $t;

            #[inline]
            fn $func(self, rhs: &$t) -> Self::Output { (*self).$func(*rhs) }
        }
        impl core::ops::$trt<$t> for &$t {
            type Output = $t;

            #[inline]
            fn $func(self, rhs: $t) -> Self::Output { (*self).$func(rhs) }
        }
    };
    ($t:ty, $elem:ident, $trt:ident, $func:ident) => {
        impl core::ops::$trt<&$t> for $elem {
            type Output = $elem;

            #[inline]
            fn $func(self, rhs: &$t) -> Self::Output { self.$func(*rhs) }
        }
        impl core::ops::$trt<&$t> for &$elem {
            type Output = $elem;

            #[inline]
            fn $func(self, rhs: &$t) -> Self::Output { (*self).$func(*rhs) }
        }
        impl core::ops::$trt<$t> for &$elem {
            type Output = $elem;

            #[inline]
            fn $func(self, rhs: $t) -> Self::Output { (*self).$func(rhs) }
        }
    };
}
