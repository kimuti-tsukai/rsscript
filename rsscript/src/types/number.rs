use std::fmt::Display;

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Number {
    value: f64,
}

impl Display for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl Number {
    pub const EPSILON: Self = Self {
        value: f64::EPSILON,
    };

    pub const MAX_SAFE_INTEGER: Self = Self {
        value: 9007199254740991f64,
    };

    pub const MAX_VALUE: Self = Self { value: f64::MAX };

    pub const MIN_SAFE_INTEGER: Self = Self {
        value: -9007199254740991f64,
    };

    pub const MIN_VALUE: Self = Self { value: f64::MIN };

    pub const NaN: Self = Self { value: f64::NAN };

    pub const NEGATIVE_INFINITY: Self = Self {
        value: f64::NEG_INFINITY,
    };

    pub const POSITIVE_INFINITY: Self = Self {
        value: f64::INFINITY,
    };

    pub const fn isFinit(self) -> bool {
        self.value.is_finite()
    }

    pub const fn isInteger(self) -> bool {
        self.value == (self.value as i64 as f64)
    }

    pub const fn isNan(self) -> bool {
        self.value.is_nan()
    }
}

impl From<f64> for Number {
    fn from(value: f64) -> Self {
        Self { value }
    }
}

impl From<Number> for f64 {
    fn from(value: Number) -> f64 {
        value.value
    }
}
