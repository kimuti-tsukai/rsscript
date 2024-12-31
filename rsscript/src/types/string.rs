use super::{number::Number, Constructor, JsValue};

#[derive(Clone, PartialEq, Eq, Debug, Hash, PartialOrd, Ord)]
pub struct JsString {
    value: String,
}

pub fn String(value: impl JsValue) -> JsString {
    JsString {
        value: value.to_string(),
    }
}

impl JsString {
    pub fn new(value: String) -> Self {
        Self { value }
    }

    pub fn fromCharCode(code: &[Number]) -> Self {
        Self {
            value: code.iter().map(|n| f64::from(*n) as u8 as char).collect(),
        }
    }

    pub fn fromCodePoint(code: &[Number]) -> Self {
        Self {
            value: code
                .iter()
                .map(|n| std::char::from_u32(f64::from(*n) as u32).unwrap())
                .collect(),
        }
    }

    pub fn at(&self, index: Number) -> Option<Self> {
        if index < Number(0) {
            Some(Self {
                value: self
                    .value
                    .chars()
                    .nth(self.value.len() - f64::from(index) as usize)?
                    .to_string(),
            })
        } else {
            Some(Self {
                value: self
                    .value
                    .chars()
                    .nth(f64::from(index) as usize)?
                    .to_string(),
            })
        }
    }

    pub fn charAt(&self, index: Number) -> Option<Self> {
        if index < Number(0) {
            None
        } else {
            Some(Self {
                value: self
                    .value
                    .chars()
                    .nth(f64::from(index) as usize)?
                    .to_string(),
            })
        }
    }
}

impl std::fmt::Display for JsString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl<T: JsValue> Constructor<fn(T) -> Self> for JsString {
    const constructor: fn(T) -> Self = String;
}
