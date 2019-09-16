#[derive(Debug, PartialEq)]
pub enum ValidatorOption {
    Must,
    Allow,
    NotAllow,
}

impl ValidatorOption {
    #[inline]
    pub fn allow(&self) -> bool {
        match self {
            ValidatorOption::Must => true,
            ValidatorOption::Allow => true,
            ValidatorOption::NotAllow => false,
        }
    }

    #[inline]
    pub fn not_allow(&self) -> bool {
        match self {
            ValidatorOption::Must => false,
            ValidatorOption::Allow => false,
            ValidatorOption::NotAllow => true,
        }
    }

    #[inline]
    pub fn must(&self) -> bool {
        match self {
            ValidatorOption::Must => true,
            ValidatorOption::Allow => false,
            ValidatorOption::NotAllow => false,
        }
    }
}
