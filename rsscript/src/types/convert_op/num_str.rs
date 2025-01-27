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

impl Add<&Number> for &JsString {
    type Output = JsString;

    fn add(self, rhs: &Number) -> Self::Output {
        self + &JsValue::toString(rhs)
    }
}

impl Add<Number> for &JsString {
    type Output = JsString;

    fn add(self, rhs: Number) -> Self::Output {
        self.add(&rhs)
    }
}

impl Add<&Number> for JsString {
    type Output = JsString;

    fn add(self, rhs: &Number) -> Self::Output {
        (&self).add(rhs)
    }
}

impl Add<Number> for JsString {
    type Output = JsString;

    fn add(self, rhs: Number) -> Self::Output {
        (&self).add(&rhs)
    }
}
