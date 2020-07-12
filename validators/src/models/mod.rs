#[cfg(all(feature = "std", feature = "idna"))]
mod host;

#[cfg(all(feature = "std", feature = "idna"))]
pub use host::*;
