//! Floating-point math primitives for `no_std` consumers.
//!
//! The public API belongs to `ample`; `libm` is currently an implementation
//! detail and can be replaced without changing downstream crates.

/// Returns the sine of `value` in radians.
#[inline]
pub fn sin(value: f32) -> f32 {
    libm::sinf(value)
}

/// Returns the largest integer less than or equal to `value`.
#[inline]
pub fn floor(value: f32) -> f32 {
    libm::floorf(value)
}

/// Returns `value` rounded to the nearest integer.
#[inline]
pub fn round(value: f32) -> f32 {
    libm::roundf(value)
}

/// Raises `base` to the floating-point power `exponent`.
#[inline]
pub fn pow(base: f32, exponent: f32) -> f32 {
    libm::powf(base, exponent)
}
