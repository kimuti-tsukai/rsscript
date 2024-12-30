use super::number::Number;

pub struct JsString {
    value: String,
}

pub fn JsString(value: String) -> JsString {
    JsString { value }
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
}
