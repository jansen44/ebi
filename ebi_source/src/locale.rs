use std::str::FromStr;

use serde::{de::Visitor, Deserialize, Serialize};

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Locale {
    Unknown,
    EnUs,
    PtBr,
}

impl std::convert::From<u8> for Locale {
    fn from(value: u8) -> Self {
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

struct LocaleVisitor;

impl<'de> Visitor<'de> for LocaleVisitor {
    type Value = Locale;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid locale string, e.g., \"en_US\", \"pt_BR\"")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Locale::from_str(v).map_err(|_| E::custom(format!("invalid locale: {}", v)))
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Locale::from_str(&v).map_err(|_| E::custom(format!("invalid locale: {}", v)))
    }

    fn visit_u8<E>(self, v: u8) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(Locale::from(v))
    }
}

impl Serialize for Locale {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for Locale {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_any(LocaleVisitor)
    }
}
