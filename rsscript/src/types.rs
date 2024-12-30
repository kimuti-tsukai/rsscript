#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]

use std::fmt::Display;

use dyn_clone::DynClone;

pub mod number;

pub mod string;

pub mod object;

pub trait JsValue: DynClone + Display {
    #[allow(non_snake_case)]
    fn toString(self) -> String;
}

impl<T: DynClone + Display> JsValue for T {
    fn toString(self) -> String {
        self.to_string()
    }
}
