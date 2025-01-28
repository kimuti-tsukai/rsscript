#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]

use std::fmt::Display;

use dyn_clone::DynClone;
use number::Number;
use string::{JsString, String};

pub mod number;

pub mod string;

pub mod object;

pub mod convert_op;

pub trait JsValue: DynClone + Display {
    #[allow(non_snake_case)]
    fn toString(self) -> JsString;

    #[allow(non_snake_case)]
    fn toNumber(self) -> Number
    where
        Self: Sized,
    {
        Number::from(self.toString())
    }
}

impl<T: DynClone + Display> JsValue for T {
    fn toString(self) -> JsString {
        String(self)
    }
}

pub trait Constructor<F> {
    const constructor: F;
}
