#[derive(Debug, Copy, Clone)]
pub enum Locale {
    PtBr,
}

impl std::fmt::Display for Locale {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::PtBr => "pt_BR",
            }
        )
    }
}
