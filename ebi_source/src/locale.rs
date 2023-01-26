#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Locale {
    Unknown = 0,
    PtBr,
}

impl std::convert::From<u32> for Locale {
    fn from(value: u32) -> Self {
        match value {
            1 => Self::PtBr,
            _ => Self::Unknown,
        }
    }
}

impl std::convert::From<String> for Locale {
    fn from(value: String) -> Self {
        match value.as_ref() {
            "pt_BR" => Self::PtBr,
            _ => Self::Unknown,
        }
    }
}

impl std::fmt::Display for Locale {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Unknown => "unknown",
                Self::PtBr => "pt_BR",
            }
        )
    }
}
