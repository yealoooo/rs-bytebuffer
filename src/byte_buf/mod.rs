//! # Art
//!
//! 这是一个接口

pub use byte_reader::ByteReader;
pub use error::{ByteBufResult, ErrorType};

pub mod error;
pub mod byte_reader;
mod common;


