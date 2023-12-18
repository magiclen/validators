#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum TriAllow {
    Must,
    Allow,
    Disallow,
}

impl TriAllow {
    #[inline]
    pub const fn allow(self) -> bool {
        match self {
            Self::Must => true,
            Self::Allow => true,
            Self::Disallow => false,
        }
    }

    #[inline]
    pub const fn disallow(self) -> bool {
        match self {
            Self::Must => false,
            Self::Allow => false,
            Self::Disallow => true,
        }
    }

    #[inline]
    pub const fn must(self) -> bool {
        match self {
            Self::Must => true,
            Self::Allow => false,
            Self::Disallow => false,
        }
    }
}
