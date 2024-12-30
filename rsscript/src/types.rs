use std::fmt::Display;

use dyn_clone::DynClone;

pub mod number;

pub trait JsValue: DynClone + Display {
    #[allow(non_snake_case)]
    fn toString(self) -> String;
}

impl<T: DynClone + Display> JsValue for T {
    fn toString(self) -> String {
        self.to_string()
    }
}
