use std::ops;

use super::Number;

macro_rules! impl_bin_ops {
    ($($($path: ident)::* , $method: ident);* $(;)?) => {
        $(
            impl $($path)::* for &Number {
                type Output = Number;

                fn $method(self, rhs: Self) -> Number {
                    Number {
                        value: self.value.$method(rhs.value),
                    }
                }
            }

            impl $($path)::* for Number {
                type Output = Self;

                fn $method(self, rhs: Self) -> Self {
                    (&self).$method(&rhs)
                }
            }

            impl $($path)::* <&Self> for Number {
                type Output = Self;

                fn $method(self, rhs: &Self) -> Self {
                    (&self).$method(rhs)
                }
            }

            impl $($path)::* <Number> for &Number {
                type Output = Number;

                fn $method(self, rhs: Number) -> Number {
                    self.$method(&rhs)
                }
            }
        )*
    };
}

impl_bin_ops!(
    ops::Add, add;
    ops::Sub, sub;
    ops::Mul, mul;
    ops::Div, div;
    ops::Rem, rem;
);

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

macro_rules! impl_bin_bit_ops {
    ($($($path: ident)::* , $method: ident);* $(;)?) => {
        $(
            impl $($path)::* for &Number {
                type Output = Number;

                fn $method(self, rhs: Self) -> Number {
                    Number {
                        value: (self.as_i64().$method(rhs.as_i64())) as f64,
                    }
                }
            }

            impl $($path)::* for Number {
                type Output = Self;

                fn $method(self, rhs: Self) -> Self {
                    (&self).$method(&rhs)
                }
            }

            impl $($path)::* <&Self> for Number {
                type Output = Self;

                fn $method(self, rhs: &Self) -> Self {
                    (&self).$method(rhs)
                }
            }

            impl $($path)::* <Number> for &Number {
                type Output = Number;

                fn $method(self, rhs: Number) -> Number {
                    self.$method(&rhs)
                }
            }
        )*
    };
}

impl_bin_bit_ops!(
    ops::BitAnd, bitand;
    ops::BitOr, bitor;
    ops::BitXor, bitxor;
    ops::Shl, shl;
    ops::Shr, shr;
);

macro_rules! impl_shl {
    ($($t:ty),* $(,)?) => ($(
        impl ops::Shl<$t> for Number {
            type Output = Self;

            fn shl(self, rhs: $t) -> Self {
                Self {
                    value: (self.as_i64() << rhs) as f64,
                }
            }
        }
    )*)
}

impl_shl!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize,);

macro_rules! impl_shr {
    ($($t:ty),* $(,)?) => ($(
        impl ops::Shr<$t> for Number {
            type Output = Self;

            fn shr(self, rhs: $t) -> Self {
                Self {
                    value: (self.as_i64() >> rhs) as f64,
                }
            }
        }
    )*)
}

impl_shr!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize,);

macro_rules! impl_assign_ops {
    ($($trait: path, $method: ident);* $(;)?) => {
        $(
            impl $trait for Number {
                fn $method(&mut self, rhs: Self) {
                    self.value.$method(rhs.value);
                }
            }
        )*
    };
}

impl_assign_ops!(
    ops::AddAssign, add_assign;
    ops::SubAssign, sub_assign;
    ops::MulAssign, mul_assign;
    ops::DivAssign, div_assign;
    ops::RemAssign, rem_assign;
);

macro_rules! impl_assign_bit_ops {
    ($($trait: path, $method: ident, $not_assign: tt);* $(;)?) => {
        $(
            impl $trait for Number {
                fn $method(&mut self, rhs: Self) {
                    self.value = (self.as_i64() $not_assign (rhs.as_i64())) as f64;
                }
            }
        )*
    };
}

impl_assign_bit_ops!(
    ops::BitAndAssign, bitand_assign, &;
    ops::BitOrAssign, bitor_assign, |;
    ops::BitXorAssign, bitxor_assign, ^;
    ops::ShlAssign, shl_assign, <<;
    ops::ShrAssign, shr_assign, >>;
);
