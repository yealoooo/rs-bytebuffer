use std::fmt::{Debug, Formatter};
use std::result::Result;

pub enum ErrorType {
    ReadableShortage
}

impl ErrorType {
    fn as_str(&self) -> &'static str{
        match *self {
            ErrorType::ReadableShortage => "可读长度不够"
        }
    }
}

impl Debug for ErrorType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self.as_str(),  f)
    }
}

pub type ByteBufResult<T> = Result<T, ErrorType>;