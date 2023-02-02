pub mod manga;
pub mod source;

use std::ffi::c_char;

pub type JSONResourceFn = extern "C" fn() -> *mut c_char;
