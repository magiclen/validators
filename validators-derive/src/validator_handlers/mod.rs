mod base32;
mod base32_decoded;
mod base64;
mod base64_decoded;
mod base64_url;
mod base64_url_decoded;
mod boolean;

pub use base32::base32_handler;
pub use base32_decoded::base32_decoded_handler;
pub use base64::base64_handler;
pub use base64_decoded::base64_decoded_handler;
pub use base64_url::base64_url_handler;
pub use base64_url_decoded::base64_url_decoded_handler;
pub use boolean::boolean_handler;
