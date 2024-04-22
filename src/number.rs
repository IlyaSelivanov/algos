/// The `Number` trait represents a numeric type that provides common operations such as minimum, maximum, and zero values.
pub trait Number {
    /// Returns the minimum value of the numeric type.
    fn min() -> Self;

    /// Returns the maximum value of the numeric type.
    fn max() -> Self;

    /// Returns the zero value of the numeric type.
    fn zero() -> Self;
}

impl Number for i32 {
    /// Returns the minimum value of the `i32` type.
    fn min() -> Self {
        i32::MIN
    }

    /// Returns the maximum value of the `i32` type.
    fn max() -> Self {
        i32::MAX
    }

    /// Returns the zero value of the `i32` type.
    fn zero() -> Self {
        0
    }
}

impl Number for f32 {
    /// Returns the minimum value of the `f32` type.
    fn min() -> Self {
        f32::MIN
    }

    /// Returns the maximum value of the `f32` type.
    fn max() -> Self {
        f32::MAX
    }

    /// Returns the zero value of the `f32` type.
    fn zero() -> Self {
        0.0
    }
}

impl Number for f64 {
    /// Returns the minimum value of the `f64` type.
    fn min() -> Self {
        f64::MIN
    }

    /// Returns the maximum value of the `f64` type.
    fn max() -> Self {
        f64::MAX
    }

    /// Returns the zero value of the `f64` type.
    fn zero() -> Self {
        0.0
    }
}

impl Number for i64 {
    /// Returns the minimum value of the `i64` type.
    fn min() -> Self {
        i64::MIN
    }

    /// Returns the maximum value of the `i64` type.
    fn max() -> Self {
        i64::MAX
    }

    /// Returns the zero value of the `i64` type.
    fn zero() -> Self {
        0
    }
}
