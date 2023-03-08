# dw-bytebuffer

- this is a bytebuffer tools



## example
```rust
let array: [u8; 4] = [0x01, 0x02, 0x01, 0x02];
let mut read_byte_buf = ByteReader::new_from( & array);

read_byte_buf.read_i8();
read_byte_buf.read_u8();
read_byte_buf.read_u16_le();
read_byte_buf.mark_read_index().read_i16_le();
read_byte_buf.reset_read_index();
read_byte_buf.read_i16_le();
read_byte_buf.read_i16_le();
```


## License
TIS is under the Apache2 License. See the [LICENSE](https://github.com/yanlongsix/rs-bytebuffer/blob/main/LICENSE) file for details.
