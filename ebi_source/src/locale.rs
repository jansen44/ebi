use serde::{Deserialize, Serialize};

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub enum Locale {
    Unknown = 0,
    EnUs,
    PtBr,
}

impl std::convert::From<u32> for Locale {
    fn from(value: u32) -> Self {
        match value {
            1 => Self::EnUs,
            2 => Self::PtBr,
            _ => Self::Unknown,
        }
    }
}

impl std::str::FromStr for Locale {
    type Err = ();

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Ok(match value {
            "en_US" => Self::EnUs,
            "pt_BR" => Self::PtBr,
            _ => Self::Unknown,
        })
    }
}

impl std::fmt::Display for Locale {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Unknown => "unknown",
                Self::EnUs => "en_US",
                Self::PtBr => "pt_BR",
            }
        )
    }
}
