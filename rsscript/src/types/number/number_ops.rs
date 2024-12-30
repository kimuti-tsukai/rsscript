use std::ops;

use super::Number;

impl ops::Add for Number {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            value: self.value + rhs.value,
        }
    }
}

impl ops::Sub for Number {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self {
            value: self.value - rhs.value,
        }
    }
}

impl ops::Mul for Number {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Self {
            value: self.value * rhs.value,
        }
    }
}

impl ops::Div for Number {
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        Self {
            value: self.value / rhs.value,
        }
    }
}

impl ops::Rem for Number {
    type Output = Self;

    fn rem(self, rhs: Self) -> Self {
        Self {
            value: self.value % rhs.value,
        }
    }
}

impl ops::Neg for Number {
    type Output = Self;

    fn neg(self) -> Self {
        Self { value: -self.value }
    }
}

impl ops::Not for Number {
    type Output = Self;

    fn not(self) -> Self {
        Self {
            value: !self.as_i64() as f64,
        }
    }
}

impl ops::BitAnd for Number {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        Self {
            value: (self.as_i64() & rhs.as_i64()) as f64,
        }
    }
}

impl ops::BitOr for Number {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self {
        Self {
            value: (self.as_i64() | rhs.as_i64()) as f64,
        }
    }
}

impl ops::BitXor for Number {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self {
        Self {
            value: (self.as_i64() ^ rhs.as_i64()) as f64,
        }
    }
}


