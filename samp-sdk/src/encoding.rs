//! String encoding.
pub use encoding_rs::{WINDOWS_1251, WINDOWS_1252, WINDOWS_874};
use encoding_rs::Encoding;

static mut DEFAULT_ENCODING: &Encoding = WINDOWS_874;

pub fn set_default_encoding(encoding: &'static Encoding) {
    unsafe {
        DEFAULT_ENCODING = encoding;
    }
}

pub(crate) fn get() -> &'static Encoding {
    unsafe {
        DEFAULT_ENCODING
    }
}