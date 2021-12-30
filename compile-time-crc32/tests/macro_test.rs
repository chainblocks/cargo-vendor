use compile_time_crc32::crc32;

#[test]
fn main() {
    println!("{:X}", crc32!(b"test"));
}
