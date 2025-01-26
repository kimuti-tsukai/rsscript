use std::ops::Deref;

use super::{number::Number, Constructor, JsValue};

#[derive(Clone, PartialEq, Eq, Debug, Hash, PartialOrd, Ord)]
pub struct JsString {
    value: String,
}

impl Deref for JsString {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
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
                .map(|n| {
                    std::char::from_u32(f64::from(*n) as u32)
                        .unwrap_or_else(|| panic!("Invalid code point: {}", n))
                })
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

    pub fn charCodeAt(&self, index: Number) -> Option<Number> {
        if index < Number(0) {
            None
        } else {
            Some(Number::from(
                self.value.chars().nth(f64::from(index) as usize)? as u8 as f64,
            ))
        }
    }

    pub fn codePointAt(&self, index: Number) -> Option<Number> {
        if index < Number(0) {
            None
        } else {
            Some(Number::from(
                self.value.chars().nth(f64::from(index) as usize)? as u32 as f64,
            ))
        }
    }

    pub fn concat(&self, args: &[JsString]) -> Self {
        Self {
            value: self.value.clone() + &args.iter().map(|s| s.value.clone()).collect::<String>(),
        }
    }

    pub fn endsWith(&self, search: &str) -> bool {
        self.ends_with(search)
    }

    pub fn includes(&self, search: &str) -> bool {
        self.contains(search)
    }

    pub fn indexOf(&self, search: &str) -> Option<Number> {
        self.find(search).map(|i| Number::from(i as f64))
    }

    pub fn lastIndexOf(&self, search: &str) -> Option<Number> {
        self.rfind(search).map(|i| Number::from(i as f64))
    }

    // pub fn r#match(&self, regex: &str) -> Option<JsArray> {
    //     let re = regex::Regex::new(regex)?;
    //     let matches = re.find_iter(&self.value).collect::<Vec<_>>();
    //     Some(JsArray::from(
    //         matches
    //             .iter()
    //             .map(|m| str::new(m.as_str().to_string()))
    //             .collect::<Vec<_>>(),
    //     ))
    // }

    // pub fn r#matchAll(&self, regex: &str) -> Option<JsArray> {
    //     let re = regex::Regex::new(regex)?;
    //     let matches = re.find_iter(&self.value).collect::<Vec<_>>();
    //     Some(JsArray::from(
    //         matches
    //             .iter()
    //             .map(|m| str::new(m.as_str().to_string()))
    //             .collect::<Vec<_>>(),
    //     ))
    // }

    pub fn replace(&self, search: &str, replace: &str) -> Self {
        Self {
            value: self.value.replace(search, replace),
        }
    }

    pub fn replaceAll(&self, search: &str, replace: &str) -> Self {
        Self {
            value: self.value.replace(search, replace),
        }
    }

    pub fn search(&self, regex: &str) -> Option<Number> {
        let re = regex::Regex::new(regex).ok()?;
        Some(Number::from(re.find(&self.value)?.start() as f64))
    }

    // pub fn slice(&self, start: Number, end: Number) -> Self {
    //     let start = if start < 0 {}
    // }

    // pub fn split(&self, separator: &str) -> JsArray {
    //     JsArray::from(
    //         self.value
    //             .split(separator)
    //             .map(|s| JsString::new(s.to_string()))
    //             .collect::<Vec<_>>(),
    //     )
    // }
}

impl std::fmt::Display for JsString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl<T: JsValue> Constructor<fn(T) -> Self> for JsString {
    const constructor: fn(T) -> Self = String;
}
