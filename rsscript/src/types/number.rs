use std::{fmt::Display, ops::Deref};

use super::{string::{JsString, String}, Constructor, JsValue};

pub mod number_ops;

#[derive(Clone, Copy, PartialEq, Debug, PartialOrd)]
pub struct Number {
    value: f64,
}

impl Deref for Number {
    type Target = f64;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

pub fn Number(value: impl JsValue) -> Number {
    Number {
        value: value.to_string().parse().unwrap(),
    }
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

    pub fn toExponential(self, fraction_digits: Option<Number>) -> JsString {
        match fraction_digits {
            Some(Self {
                value: fraction_digits,
            }) => String(format!("{:.*e}", fraction_digits as usize, self.value)),
            None => String(format!("{:e}", self.value)),
        }
    }

    pub fn toFixed(self, fraction_digits: Option<Number>) -> JsString {
        match fraction_digits {
            Some(Self {
                value: fraction_digits,
            }) => String(format!("{:.*}", fraction_digits as usize, self.value)),
            None => String(format!("{:.0}", self.value)),
        }
    }

    pub fn toPrecision(self, precision: Option<Number>) -> JsString {
        match precision {
            Some(Self { value: precision }) => String(format!("{:.*}", precision as usize, self.value)),
            None => String(format!("{}", self.value)),
        }
    }

    pub fn toString(self, radix: Option<Number>) -> JsString {
        match radix {
            Some(Self { value: radix }) => String(format!("{:.*}", radix as usize, self.value)),
            None => String(format!("{}", self.value)),
        }
    }
    
    pub const fn as_f64(self) -> f64 {
        self.value
    }

    const fn as_i64(self) -> i64 {
        self.value as i64
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

impl From<JsString> for Number {
    fn from(value: JsString) -> Self {
        Self {
            value: value.to_string().parse().unwrap_or(f64::NAN),
        }
    }
}

impl<T: JsValue> Constructor<fn(T) -> Self> for Number {
    const constructor: fn(T) -> Self = Number;
}
