#![allow(dead_code)]

use core::fmt::{self, Debug, Formatter};

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum TypeEnum {
    String,
    VecU8,
    Boolean,
    U16,
    OptionU16,
}

impl TypeEnum {
    #[inline]
    fn as_str(&self) -> &'static str {
        match self {
            TypeEnum::String => "String",
            TypeEnum::VecU8 => "Vec<u8>",
            TypeEnum::Boolean => "bool",
            TypeEnum::U16 => "u16",
            TypeEnum::OptionU16 => "Option<u16>",
        }
    }
}

impl Debug for TypeEnum {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        f.write_str(self.as_str())
    }
}
