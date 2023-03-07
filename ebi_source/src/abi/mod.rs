pub mod chapter;
pub mod manga;
pub mod source;

pub mod primitives {
    use std::ffi::{c_char, c_void};
    use std::mem::ManuallyDrop;
    use std::str::FromStr;

    use crate::error::SourceError;

    #[repr(C)]
    pub struct FFIArray {
        ptr: *const c_void,
        len: usize,
        cap: usize,
    }

    impl FFIArray {
        pub fn null() -> Self {
            Self {
                ptr: std::ptr::null(),
                len: 0,
                cap: 0,
            }
        }

        pub fn is_null(&self) -> bool {
            self.ptr.is_null()
        }
    }

    impl<T> From<Vec<T>> for FFIArray {
        fn from(v: Vec<T>) -> Self {
            let mut v = ManuallyDrop::new(v);
            let ptr = v.as_mut_ptr() as *const c_void;
            let len = v.len();
            let cap = v.capacity();
            Self { ptr, len, cap }
        }
    }

    impl<T> TryInto<Vec<T>> for FFIArray {
        type Error = SourceError;

        fn try_into(self) -> Result<Vec<T>, Self::Error> {
            if self.is_null() {
                return Err(SourceError::ABINullConversion);
            }

            match self.len {
                0 => Ok(Vec::new()),
                _ => unsafe {
                    let mut v = Vec::from_raw_parts(self.ptr as *mut T, self.len, self.cap);
                    v.shrink_to_fit();
                    Ok(v)
                },
            }
        }
    }

    #[repr(C)]
    pub struct FFIString {
        ptr: *const c_char,
        len: usize,
        cap: usize,
    }

    impl FFIString {
        pub fn null() -> Self {
            Self {
                ptr: std::ptr::null(),
                len: 0,
                cap: 0,
            }
        }

        pub fn is_null(&self) -> bool {
            self.ptr.is_null()
        }
    }

    impl From<String> for FFIString {
        fn from(v: String) -> Self {
            let mut v = ManuallyDrop::new(v);
            Self {
                ptr: v.as_mut_ptr() as *const c_char,
                len: v.len(),
                cap: v.len(),
            }
        }
    }

    impl TryInto<String> for FFIString {
        type Error = SourceError;

        fn try_into(self) -> Result<String, Self::Error> {
            if self.is_null() {
                return Err(SourceError::ABINullConversion);
            }

            match self.len {
                0 => Ok(String::new()),
                _ => unsafe {
                    let mut v = String::from_raw_parts(self.ptr as *mut u8, self.len, self.cap);
                    v.shrink_to_fit();
                    Ok(v)
                },
            }
        }
    }

    #[repr(C)]
    pub struct ABIResultArray {
        pub result: FFIArray,
        pub err: FFIString,
    }

    impl<T> Into<Result<Vec<T>, SourceError>> for ABIResultArray {
        fn into(self) -> Result<Vec<T>, SourceError> {
            if self.err.is_null() {
                return self.result.try_into();
            }

            let err: String = self.err.try_into()?;
            Err(SourceError::from_str(&err).unwrap()) // safe to unwrap
        }
    }

    impl<T> From<Result<Vec<T>, SourceError>> for ABIResultArray {
        fn from(value: Result<Vec<T>, SourceError>) -> Self {
            match value {
                Ok(arr) => ABIResultArray {
                    result: FFIArray::from(arr),
                    err: FFIString::null(),
                },
                Err(e) => ABIResultArray {
                    result: FFIArray::null(),
                    err: FFIString::from(e.to_string()),
                },
            }
        }
    }
}
