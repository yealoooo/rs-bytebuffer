#[macro_export]
macro_rules! get_number {
    ($self:ident, $name:ident, $length:expr, $isBe:expr) => {{

        if $self.readable() < $length { return Err(ErrorType::ReadableShortage); }

        if $isBe {
            return Ok(BigEndian::$name(&$self.byte_array[$self.read_index..$self.read_index + $length]));
        } else {
            return Ok(LittleEndian::$name(&$self.byte_array[$self.read_index..$self.read_index + $length]));
        }
    }};
}

#[macro_export]
macro_rules! read_number {
    ($self:ident, $name:ident, $length:expr) => {{
        return match $self.$name() {
            Ok(v) => {
                $self.read_index += $length;
                Ok(v)
            },
            Err(error_type) => {Err(error_type)}
        }
    }};
}