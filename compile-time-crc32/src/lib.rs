use proc_macro_hack::proc_macro_hack;

/// Compile time crc32
///
/// Example:
/// ```rust
/// use compile_time_crc32::crc32;
///
/// fn main() {
///     assert_eq!(0xD87F7E0C, crc32!("test"));
/// }
/// ```
#[proc_macro_hack]
pub use compile_time_crc32_impl::crc32;
