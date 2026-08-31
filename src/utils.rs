pub trait ToHexString {
    fn to_hex_string(&self) -> String;
}

impl<T> ToHexString for T
where
    T: std::fmt::UpperHex,
{
    fn to_hex_string(&self) -> String {
        format!("0x{:02X}", self)
    }
}
