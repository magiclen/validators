#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum SeparatorOption {
    Must(u8),
    Allow(u8),
    Disallow,
}

impl SeparatorOption {
    #[inline]
    pub fn allow(self) -> Option<u8> {
        match self {
            Self::Must(c) | Self::Allow(c) => Some(c),
            Self::Disallow => None,
        }
    }

    #[inline]
    pub const fn disallow(self) -> bool {
        match self {
            Self::Must(_) => false,
            Self::Allow(_) => false,
            Self::Disallow => true,
        }
    }

    #[inline]
    pub const fn must(self) -> Option<u8> {
        match self {
            Self::Must(c) => Some(c),
            Self::Allow(_) => None,
            Self::Disallow => None,
        }
    }
}
