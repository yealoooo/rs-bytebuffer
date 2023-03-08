# rs-bytebuffer

- byte 操作工具类



## 示例
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


## 许可协议
TIS is under the Apache2 License. See the [LICENSE](https://github.com/yealou/rs-bytebuffer/blob/main/LICENSE) file for details.

## 反馈
您在使用过程中有任何不满或者批评都请说出, 您提出的宝贵意见是对我们最大的支持和鼓励, [我要提建议](https://github.com/yealou/rs-bytebuffer/issues/new)

