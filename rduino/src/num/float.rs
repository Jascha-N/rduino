use libc;

use core::f32;
use core::f64;
use core::intrinsics;
use core::num;

/// Extension trait for floating point types.
///
/// This trait mimics the methods available on floating point primitives in `std`. Deprecated or
/// unstable methods are not implemented. Implementations for `f32` and `f64` are provided.
///
/// For documentation on these methods see [`f32`][f32] and [`f64`][f64] of the Rust Standard
/// Library documentation.
///
/// [f32]: https://doc.rust-lang.org/std/primitive.f32.html
/// [f64]: https://doc.rust-lang.org/std/primitive.f64.html
#[allow(missing_docs)]
pub trait Float: num::Float {
    #[inline] fn is_nan(self) -> bool { num::Float::is_nan(self) }
    #[inline] fn is_infinite(self) -> bool { num::Float::is_infinite(self) }
    #[inline] fn is_finite(self) -> bool { num::Float::is_finite(self) }
    #[inline] fn is_normal(self) -> bool { num::Float::is_normal(self) }
    #[inline] fn classify(self) -> num::FpCategory { num::Float::classify(self) }
    fn floor(self) -> Self;
    fn ceil(self) -> Self;
    fn round(self) -> Self;
    fn trunc(self) -> Self;
    fn fract(self) -> Self;
    #[inline] fn abs(self) -> Self { num::Float::abs(self) }
    #[inline] fn signum(self) -> Self { num::Float::signum(self) }
    #[inline] fn is_sign_positive(self) -> bool { num::Float::is_sign_positive(self) }
    #[inline] fn is_sign_negative(self) -> bool { num::Float::is_sign_negative(self) }
    fn mul_add(self, a: Self, b: Self) -> Self;
    #[inline] fn recip(self) -> Self { num::Float::recip(self) }
    #[inline] fn powi(self, n: i32) -> Self { num::Float::powi(self, n) }
    fn powf(self, n: Self) -> Self;
    fn sqrt(self) -> Self;
    fn exp(self) -> Self;
    fn exp2(self) -> Self;
    fn ln(self) -> Self;
    fn log(self, base: Self) -> Self;
    fn log2(self) -> Self;
    fn log10(self) -> Self;
    #[inline] fn to_degrees(self) -> Self { num::Float::to_degrees(self) }
    #[inline] fn to_radians(self) -> Self { num::Float::to_radians(self) }
    fn max(self, other: Self) -> Self;
    fn min(self, other: Self) -> Self;
    fn cbrt(self) -> Self;
    fn hypot(self, other: Self) -> Self;
    fn sin(self) -> Self;
    fn cos(self) -> Self;
    fn tan(self) -> Self;
    fn asin(self) -> Self;
    fn acos(self) -> Self;
    fn atan(self) -> Self;
    fn atan2(self, other: Self) -> Self;
    fn sin_cos(self) -> (Self, Self);
    fn exp_m1(self) -> Self;
    fn ln_1p(self) -> Self;
    fn sinh(self) -> Self;
    fn cosh(self) -> Self;
    fn tanh(self) -> Self;
    fn asinh(self) -> Self;
    fn acosh(self) -> Self;
    fn atanh(self) -> Self;
}

impl Float for f32 {
    #[inline]
    fn floor(self) -> f32 {
        unsafe { intrinsics::floorf32(self) }
    }

    #[inline]
    fn ceil(self) -> f32 {
        unsafe { intrinsics::ceilf32(self) }
    }

    #[inline]
    fn round(self) -> f32 {
        unsafe { intrinsics::roundf32(self) }
    }

    #[inline]
    fn trunc(self) -> f32 {
        unsafe { intrinsics::truncf32(self) }
    }

    #[inline]
    fn fract(self) -> f32 {
        self - self.trunc()
    }

    #[inline]
    fn mul_add(self, a: f32, b: f32) -> f32 {
        unsafe { intrinsics::fmaf32(self, a, b) }
    }

    #[inline]
    fn powf(self, n: f32) -> f32 {
        unsafe { intrinsics::powf32(self, n) }
    }

    #[inline]
    fn sqrt(self) -> f32 {
        if self < 0.0 {
            f32::NAN
        } else {
            unsafe { intrinsics::sqrtf32(self) }
        }
    }

    #[inline]
    fn exp(self) -> f32 {
        unsafe { intrinsics::expf32(self) }
    }

    #[inline]
    fn exp2(self) -> f32 {
        unsafe { intrinsics::exp2f32(self) }
    }

    #[inline]
    fn ln(self) -> f32 {
        unsafe { intrinsics::logf32(self) }
    }

    #[inline]
    fn log(self, base: f32) -> f32 {
        self.ln() / base.ln()
    }

    #[inline]
    fn log2(self) -> f32 {
        unsafe { intrinsics::log2f32(self) }
    }

    #[inline]
    fn log10(self) -> f32 {
        unsafe { intrinsics::log10f32(self) }
    }

    #[inline]
    fn max(self, other: f32) -> f32 {
        unsafe { libc::fmaxf(self, other) }
    }

    #[inline]
    fn min(self, other: f32) -> f32 {
        unsafe { libc::fminf(self, other) }
    }

    #[inline]
    fn cbrt(self) -> f32 {
        unsafe { libc::cbrtf(self) }
    }

    #[inline]
    fn hypot(self, other: f32) -> f32 {
        unsafe { libc::hypotf(self, other) }
    }

    #[inline]
    fn sin(self) -> f32 {
        unsafe { intrinsics::sinf32(self) }
    }

    #[inline]
    fn cos(self) -> f32 {
        unsafe { intrinsics::cosf32(self) }
    }

    #[inline]
    fn tan(self) -> f32 {
        unsafe { libc::tanf(self) }
    }

    #[inline]
    fn asin(self) -> f32 {
        unsafe { libc::asinf(self) }
    }

    #[inline]
    fn acos(self) -> f32 {
        unsafe { libc::acosf(self) }
    }

    #[inline]
    fn atan(self) -> f32 {
        unsafe { libc::atanf(self) }
    }

    #[inline]
    fn atan2(self, other: f32) -> f32 {
        unsafe { libc::atan2f(self, other) }
    }

    #[inline]
    fn sin_cos(self) -> (f32, f32) {
        (self.sin(), self.cos())
    }

    #[inline]
    fn exp_m1(self) -> f32 {
        unsafe { libc::expm1f(self) }
    }

    #[inline]
    fn ln_1p(self) -> f32 {
        unsafe { libc::log1pf(self) }
    }

    #[inline]
    fn sinh(self) -> f32 {
        unsafe { libc::sinhf(self) }
    }

    #[inline]
    fn cosh(self) -> f32 {
        unsafe { libc::coshf(self) }
    }

    #[inline]
    fn tanh(self) -> f32 {
        unsafe { libc::tanhf(self) }
    }

    #[inline]
    fn asinh(self) -> f32 {
        if self == f32::NEG_INFINITY {
            f32::NEG_INFINITY
        } else {
            (self + ((self * self) + 1.0).sqrt()).ln()
        }
    }

    #[inline]
    fn acosh(self) -> f32 {
        match self {
            x if x < 1.0 => f32::NAN,
            x => (x + ((x * x) - 1.0).sqrt()).ln(),
        }
    }

    #[inline]
    fn atanh(self) -> f32 {
        0.5 * ((2.0 * self) / (1.0 - self)).ln_1p()
    }
}

impl Float for f64 {
    #[inline]
    fn floor(self) -> f64 {
        unsafe { intrinsics::floorf64(self) }
    }

    #[inline]
    fn ceil(self) -> f64 {
        unsafe { intrinsics::ceilf64(self) }
    }

    #[inline]
    fn round(self) -> f64 {
        unsafe { intrinsics::roundf64(self) }
    }

    #[inline]
    fn trunc(self) -> f64 {
        unsafe { intrinsics::truncf64(self) }
    }

    #[inline]
    fn fract(self) -> f64 {
        self - self.trunc()
    }

    #[inline]
    fn mul_add(self, a: f64, b: f64) -> f64 {
        unsafe { intrinsics::fmaf64(self, a, b) }
    }

    #[inline]
    fn powf(self, n: f64) -> f64 {
        unsafe { intrinsics::powf64(self, n) }
    }

    #[inline]
    fn sqrt(self) -> f64 {
        if self < 0.0 {
            f64::NAN
        } else {
            unsafe { intrinsics::sqrtf64(self) }
        }
    }

    #[inline]
    fn exp(self) -> f64 {
        unsafe { intrinsics::expf64(self) }
    }

    #[inline]
    fn exp2(self) -> f64 {
        unsafe { intrinsics::exp2f64(self) }
    }

    #[inline]
    fn ln(self) -> f64 {
        unsafe { intrinsics::logf64(self) }
    }

    #[inline]
    fn log(self, base: f64) -> f64 {
        self.ln() / base.ln()
    }

    #[inline]
    fn log2(self) -> f64 {
        unsafe { intrinsics::log2f64(self) }
    }

    #[inline]
    fn log10(self) -> f64 {
        unsafe { intrinsics::log10f64(self) }
    }

    #[inline]
    fn max(self, other: f64) -> f64 {
        unsafe { libc::fmax(self, other) }
    }

    #[inline]
    fn min(self, other: f64) -> f64 {
        unsafe { libc::fmin(self, other) }
    }

    #[inline]
    fn cbrt(self) -> f64 {
        unsafe { libc::cbrt(self) }
    }

    #[inline]
    fn hypot(self, other: f64) -> f64 {
        unsafe { libc::hypot(self, other) }
    }

    #[inline]
    fn sin(self) -> f64 {
        unsafe { intrinsics::sinf64(self) }
    }

    #[inline]
    fn cos(self) -> f64 {
        unsafe { intrinsics::cosf64(self) }
    }

    #[inline]
    fn tan(self) -> f64 {
        unsafe { libc::tan(self) }
    }

    #[inline]
    fn asin(self) -> f64 {
        unsafe { libc::asin(self) }
    }

    #[inline]
    fn acos(self) -> f64 {
        unsafe { libc::acos(self) }
    }

    #[inline]
    fn atan(self) -> f64 {
        unsafe { libc::atan(self) }
    }

    #[inline]
    fn atan2(self, other: f64) -> f64 {
        unsafe { libc::atan2(self, other) }
    }

    #[inline]
    fn sin_cos(self) -> (f64, f64) {
        (self.sin(), self.cos())
    }

    #[inline]
    fn exp_m1(self) -> f64 {
        unsafe { libc::expm1(self) }
    }

    #[inline]
    fn ln_1p(self) -> f64 {
        unsafe { libc::log1p(self) }
    }

    #[inline]
    fn sinh(self) -> f64 {
        unsafe { libc::sinh(self) }
    }

    #[inline]
    fn cosh(self) -> f64 {
        unsafe { libc::cosh(self) }
    }

    #[inline]
    fn tanh(self) -> f64 {
        unsafe { libc::tanh(self) }
    }

    #[inline]
    fn asinh(self) -> f64 {
        if self == f64::NEG_INFINITY {
            f64::NEG_INFINITY
        } else {
            (self + ((self * self) + 1.0).sqrt()).ln()
        }
    }

    #[inline]
    fn acosh(self) -> f64 {
        match self {
            x if x < 1.0 => f64::NAN,
            x => (x + ((x * x) - 1.0).sqrt()).ln(),
        }
    }

    #[inline]
    fn atanh(self) -> f64 {
        0.5 * ((2.0 * self) / (1.0 - self)).ln_1p()
    }
}
