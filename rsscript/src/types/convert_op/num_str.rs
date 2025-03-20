use std::ops::{Add, Sub};

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

impl Sub<&Number> for &JsString {
    type Output = Number;
    
    fn sub(self, rhs: &Number) -> Self::Output {
        Number(self) - rhs
    }
}

impl Sub<&JsString> for &Number {
    type Output = Number;
    
    fn sub(self, rhs: &JsString) -> Self::Output {
        self - Number(rhs)
    }
}

impl Sub<&Number> for JsString {
    type Output = Number;

    fn sub(self, rhs: &Number) -> Self::Output {
        Number(self) - rhs
    }
}

impl Sub<&JsString> for Number {
    type Output = Number;

    fn sub(self, rhs: &JsString) -> Self::Output {
        self - Number(rhs)
    }
}

impl Sub<Number> for JsString {
    type Output = Number;

    fn sub(self, rhs: Number) -> Self::Output {
        Number(self) - rhs
    }
}

impl Sub<JsString> for Number {
    type Output = Number;

    fn sub(self, rhs: JsString) -> Self::Output {
        self - Number(rhs)
    }
}
