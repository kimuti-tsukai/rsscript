use std::ops::Add;

use crate::types::{Number, JsString, JsValue};

impl Add<&JsString> for &Number {
    type Output = JsString;

    fn add(self, rhs: &JsString) -> Self::Output {
        JsValue::toString(self) + rhs
    }
}

impl Add<JsString> for &Number {
    type Output = JsString;

    fn add(self, rhs: JsString) -> Self::Output {
        self + &rhs
    }
}

impl Add<&JsString> for Number {
    type Output = JsString;

    fn add(self, rhs: &JsString) -> Self::Output {
        (&self).add(rhs)
    }
}

impl Add<JsString> for Number {
    type Output = JsString;

    fn add(self, rhs: JsString) -> Self::Output {
        &self + &rhs
    }
}
