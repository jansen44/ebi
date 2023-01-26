use std::ffi::{c_char, CString};

use crate::{locale, Source};

#[repr(C)]
pub struct ABISource {
    pub identifier: *mut c_char,
    pub title: *mut c_char,
    pub description: *mut c_char,
    pub locale: u32,
}

impl std::convert::Into<Source> for ABISource {
    fn into(self) -> Source {
        let identifier = unsafe { CString::from_raw(self.identifier) };
        let title = unsafe { CString::from_raw(self.title) };
        let description = unsafe { CString::from_raw(self.description) };

        let identifier = identifier.to_string_lossy().into_owned();
        let title = title.to_string_lossy().into_owned();
        let description = description.to_string_lossy().into_owned();
        let locale = locale::Locale::from(self.locale);

        Source {
            identifier,
            title,
            description,
            locale,
        }
    }
}

impl std::convert::From<Source> for ABISource {
    fn from(value: Source) -> Self {
        let identifier = CString::new(value.identifier).unwrap();
        let title = CString::new(value.title).unwrap();
        let description = CString::new(value.description).unwrap();

        Self {
            identifier: identifier.into_raw(),
            title: title.into_raw(),
            description: description.into_raw(),
            locale: value.locale as u32,
        }
    }
}
