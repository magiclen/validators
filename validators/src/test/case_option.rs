#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum CaseOption {
    Any,
    Upper,
    Lower,
}

impl CaseOption {
    #[inline]
    pub const fn any(self) -> bool {
        match self {
            Self::Any => true,
            Self::Upper => false,
            Self::Lower => false,
        }
    }

    #[inline]
    pub const fn upper(self) -> bool {
        match self {
            Self::Any => true,
            Self::Upper => true,
            Self::Lower => false,
        }
    }

    #[inline]
    pub const fn lower(self) -> bool {
        match self {
            Self::Any => true,
            Self::Upper => false,
            Self::Lower => true,
        }
    }
}
