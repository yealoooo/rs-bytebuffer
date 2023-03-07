pub use error::{ByteBufResult, ErrorType};
pub use read_byte_buf::ReadByteBuf;

pub mod error;
pub mod read_byte_buf;

pub trait ByteBuf {
    fn capacity(&self) -> usize;
    fn readable(&self) -> usize;
    fn mark_read_index(&mut self) -> &mut Self;
    fn reset_read_index(&mut self);


    // get
    fn get_bytes_of_write(&self, target: &mut [u8]) -> ByteBufResult<()>;
    fn get_bytes_of_length(&self, length: usize) -> ByteBufResult<Vec<u8>>;


    fn get_u8(&self) -> ByteBufResult<u8>;
    fn get_i8(&self) -> ByteBufResult<i8>;


    fn get_u16_be(&self) -> ByteBufResult<u16>;
    fn get_u16_le(&self) -> ByteBufResult<u16>;
    fn get_i16_be(&self) -> ByteBufResult<i16>;
    fn get_i16_le(&self) -> ByteBufResult<i16>;


    fn get_u32_be(&self) -> ByteBufResult<u32>;
    fn get_u32_le(&self) -> ByteBufResult<u32>;
    fn get_i32_be(&self) -> ByteBufResult<i32>;
    fn get_i32_le(&self) -> ByteBufResult<i32>;


    fn get_u64_be(&self) -> ByteBufResult<u64>;
    fn get_u64_le(&self) -> ByteBufResult<u64>;
    fn get_i64_be(&self) -> ByteBufResult<i64>;
    fn get_i64_le(&self) -> ByteBufResult<i64>;

    // read

    fn read_bytes_of_write(&mut self, target: &mut [u8]) -> ByteBufResult<()>;
    fn read_bytes_of_length(&mut self, length: usize) -> ByteBufResult<Vec<u8>>;

    fn read_u8(&mut self) -> ByteBufResult<u8>;
    fn read_i8(&mut self) -> ByteBufResult<i8>;

    fn read_u16_be(&mut self) -> ByteBufResult<u16>;
    fn read_u16_le(&mut self) -> ByteBufResult<u16>;
    fn read_i16_be(&mut self) -> ByteBufResult<i16>;
    fn read_i16_le(&mut self) -> ByteBufResult<i16>;


    fn read_u32_be(&mut self) -> ByteBufResult<u32>;
    fn read_u32_le(&mut self) -> ByteBufResult<u32>;
    fn read_i32_be(&mut self) -> ByteBufResult<i32>;
    fn read_i32_le(&mut self) -> ByteBufResult<i32>;


    fn read_u64_be(&mut self) -> ByteBufResult<u64>;
    fn read_u64_le(&mut self) -> ByteBufResult<u64>;
    fn read_i64_be(&mut self) -> ByteBufResult<i64>;
    fn read_i64_le(&mut self) -> ByteBufResult<i64>;
}
