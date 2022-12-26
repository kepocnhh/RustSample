fn main() {
    // integer
    let integer_i8: i8 = i8::MAX;
    let integer_u8: u8 = u8::MAX;
    let integer_i16: i16 = i16::MAX;
    let integer_u16: u16 = u16::MAX;
    let integer_i32: i32 = i32::MAX;
    let integer_u32: u32 = u32::MAX;
    let integer_i64: i64 = i64::MAX;
    let integer_u64: u64 = u64::MAX;
    let integer_i128: i128 = i128::MAX;
    let integer_u128: u128 = u128::MAX;
    let integer_isize: isize = isize::MAX;
    let integer_usize: usize = usize::MAX;
    println!("i8 max: {integer_i8}");
    println!("u8 max: {integer_u8}");
    println!("i16 max: {integer_i16}");
    println!("u16 max: {integer_u16}");
    println!("i32 max: {integer_i32}");
    println!("u32 max: {integer_u32}");
    println!("i64 max: {integer_i64}");
    println!("u64 max: {integer_u64}");
    println!("i128 max: {integer_i128}");
    println!("u128 max: {integer_u128}");
    println!("isize max: {integer_isize}");
    println!("usize max: {integer_usize}");

    // Number literals
    const INTEGER_DECIMAL: isize = 98_222;
    const INTEGER_HEX: isize = 0xff;
    const INTEGER_OCTAL: isize = 0o77;
    const INTEGER_BINARY: isize = 0b1111_0000;
    const INTEGER_BYTE: u8 = b'A';
    println!("INTEGER_DECIMAL: {INTEGER_DECIMAL}");
    println!("INTEGER_HEX: {INTEGER_HEX}");
    println!("INTEGER_OCTAL: {INTEGER_OCTAL}");
    println!("INTEGER_BINARY: {INTEGER_BINARY}");
    println!("INTEGER_BYTE: {INTEGER_BYTE}");
}
