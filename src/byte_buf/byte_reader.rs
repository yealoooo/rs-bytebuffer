//! # ByteReader 只供读取的buffer
//!
//! ## 示例
//! ```rust
//! use crate::rs_bytebuffer::byte_buf::ByteReader;
//!
//! let array: [u8; 4] = [0x01, 0x02, 0x01, 0x02];
//! let mut read_byte_buf = ByteReader::new_from( & array);
//!
//! read_byte_buf.read_i8();
//! read_byte_buf.read_u8();
//! read_byte_buf.read_u16_le();
//! read_byte_buf.mark_read_index().read_i16_le();
//! read_byte_buf.reset_read_index();
//! read_byte_buf.read_i16_le();
//! read_byte_buf.read_i16_le();
//! ```


use byteorder::{BigEndian, ByteOrder, LittleEndian};

use crate::{get_number, read_number};
use crate::byte_buf::{ByteBufResult, ErrorType};

pub struct ByteReader {
    byte_array: Vec<u8>,
    capacity: usize,
    read_index: usize,
    read_mark: isize,
}


impl ByteReader {
    pub fn new_from(src: &[u8]) -> Self {
        ByteReader {
            byte_array: Vec::from(src),
            capacity: src.len(),
            read_index: 0,
            read_mark: -1,
        }
    }

    pub fn capacity(&self) -> usize {
        self.capacity
    }

    pub fn readable(&self) -> usize {
        self.capacity - self.read_index
    }

    pub fn mark_read_index(&mut self) -> &mut Self {
        self.read_mark = self.read_index as isize;
        self
    }

    pub fn reset_read_index(&mut self) {
        if self.read_mark == -1 {
            return;
        }
        self.read_index = self.read_mark as usize;
        self.read_mark = -1;
    }

    pub fn get_bytes_of_write(&self, target: &mut [u8]) -> ByteBufResult<()> {
        if self.readable() < target.len() {
            return Err(ErrorType::ReadableShortage);
        }

        for i in self.read_index..target.len() {
            target[i] = self.byte_array[i]
        }

        Ok(())
    }

    pub fn get_bytes_of_length(&self, length: usize) -> ByteBufResult<Vec<u8>> {
        if self.readable() < length {
            return Err(ErrorType::ReadableShortage);
        }

        let bytes = &self.byte_array.as_slice()[self.read_index..self.read_index + length];

        Ok(Vec::from(bytes))
    }

    pub fn get_u8(&self) -> ByteBufResult<u8> {
        return Ok(self.byte_array[self.read_index]);
    }

    pub fn get_i8(&self) -> ByteBufResult<i8> {
        return Ok(self.byte_array[self.read_index] as i8);
    }

    pub fn get_u16_be(&self) -> ByteBufResult<u16> {
        get_number!(self, read_u16, 2, true)
    }

    pub fn get_u16_le(&self) -> ByteBufResult<u16> {
        get_number!(self, read_u16, 2, false)
    }

    pub fn get_i16_be(&self) -> ByteBufResult<i16> {
        get_number!(self, read_i16, 2, true)
    }

    pub fn get_i16_le(&self) -> ByteBufResult<i16> {
        get_number!(self, read_i16, 2, true)
    }

    pub fn get_u32_be(&self) -> ByteBufResult<u32> {
        get_number!(self, read_u32, 2, true)
    }

    pub fn get_u32_le(&self) -> ByteBufResult<u32> {
        get_number!(self, read_u32, 4, false)
    }

    pub fn get_i32_be(&self) -> ByteBufResult<i32> {
        get_number!(self, read_i32, 4, true)
    }

    pub fn get_i32_le(&self) -> ByteBufResult<i32> {
        get_number!(self, read_i32, 4, false)
    }

    pub fn get_u64_be(&self) -> ByteBufResult<u64> {
        get_number!(self, read_u64, 8, true)
    }

    pub fn get_u64_le(&self) -> ByteBufResult<u64> {
        get_number!(self, read_u64, 8, false)
    }

    pub fn get_i64_be(&self) -> ByteBufResult<i64> {
        get_number!(self, read_i64, 8, true)
    }

    pub fn get_i64_le(&self) -> ByteBufResult<i64> {
        get_number!(self, read_i64, 8, false)
    }

    pub fn read_bytes_of_write(&mut self, target: &mut [u8]) -> ByteBufResult<()> {
        return match self.get_bytes_of_write(target) {
            Ok(_) => {
                self.read_index += target.len();
                Ok(())
            }
            Err(error_type) => { Err(error_type) }
        };
    }

    pub fn read_bytes_of_length(&mut self, length: usize) -> ByteBufResult<Vec<u8>> {
        return match self.get_bytes_of_length(length) {
            Ok(vec) => {
                self.read_index += length;
                Ok(vec)
            }
            Err(error_type) => {
                Err(error_type)
            }
        };
    }

    pub fn read_u8(&mut self) -> ByteBufResult<u8> {
        read_number!(self, get_u8, 1)
    }

    pub fn read_i8(&mut self) -> ByteBufResult<i8> {
        read_number!(self, get_i8, 1)
    }

    pub fn read_u16_be(&mut self) -> ByteBufResult<u16> {
        read_number!(self, get_u16_be, 2)
    }

    pub fn read_u16_le(&mut self) -> ByteBufResult<u16> {
        read_number!(self, get_u16_le, 2)
    }

    pub fn read_i16_be(&mut self) -> ByteBufResult<i16> {
        read_number!(self, get_i16_be, 2)
    }

    pub fn read_i16_le(&mut self) -> ByteBufResult<i16> {
        read_number!(self, get_i16_le, 2)
    }

    pub fn read_u32_be(&mut self) -> ByteBufResult<u32> {
        read_number!(self, get_u32_be, 2)
    }

    pub fn read_u32_le(&mut self) -> ByteBufResult<u32> {
        read_number!(self, get_u32_le, 2)
    }

    pub fn read_i32_be(&mut self) -> ByteBufResult<i32> {
        read_number!(self, get_i32_be, 2)
    }

    pub fn read_i32_le(&mut self) -> ByteBufResult<i32> {
        read_number!(self, get_i32_le, 2)
    }

    pub fn read_u64_be(&mut self) -> ByteBufResult<u64> {
        read_number!(self, get_u64_be, 2)
    }

    pub fn read_u64_le(&mut self) -> ByteBufResult<u64> {
        read_number!(self, get_u64_le, 2)
    }

    pub fn read_i64_be(&mut self) -> ByteBufResult<i64> {
        read_number!(self, get_i64_be, 2)
    }

    pub fn read_i64_le(&mut self) -> ByteBufResult<i64> {
        read_number!(self, get_i64_le, 2)
    }
}