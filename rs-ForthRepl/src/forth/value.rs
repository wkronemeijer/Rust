use std::fmt::Display;

use super::error::ForthResult;

#[derive(Debug, Copy, Clone)]
pub enum ForthValue {
    Int(i32),
}

impl ForthValue {
    // as? into? try_into_int? open for suggestion
    pub fn as_int(self) -> ForthResult<i32> {
        match self {
            ForthValue::Int(i) => Ok(i),
        }
    }
}

impl Display for ForthValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ForthValue::Int(i) => write!(f, "{}", i),
        }
    }
}
