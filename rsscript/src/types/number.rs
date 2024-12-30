use std::fmt::Display;

#[derive(Clone, Copy, PartialEq, Debug, PartialOrd)]
pub struct Number {
    value: f64,
}

pub fn Number(value: f64) -> Number {
    Number { value }
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

    pub fn toExponential(self, fraction_digits: Option<Number>) -> String {
        match fraction_digits {
            Some(Self {
                value: fraction_digits,
            }) => format!("{:.*e}", fraction_digits as usize, self.value),
            None => format!("{:e}", self.value),
        }
    }

    pub fn toFixed(self, fraction_digits: Option<Number>) -> String {
        match fraction_digits {
            Some(Self {
                value: fraction_digits,
            }) => format!("{:.*}", fraction_digits as usize, self.value),
            None => format!("{:.0}", self.value),
        }
    }

    pub fn toPrecision(self, precision: Option<Number>) -> String {
        match precision {
            Some(Self { value: precision }) => format!("{:.*}", precision as usize, self.value),
            None => format!("{}", self.value),
        }
    }

    pub fn toString(self, radix: Option<Number>) -> String {
        match radix {
            Some(Self { value: radix }) => format!("{:.*}", radix as usize, self.value),
            None => format!("{}", self.value),
        }
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
