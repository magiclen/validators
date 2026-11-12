#[allow(dead_code)]
pub(crate) mod attributes;
#[allow(dead_code)]
pub(crate) mod flag_options;
#[allow(dead_code)]
pub(crate) mod rocket_options;
#[allow(dead_code)]
pub(crate) mod serde_options;
#[allow(dead_code)]
pub(crate) mod type_enum;

#[cfg(any(feature = "serde", feature = "json"))]
#[allow(dead_code)]
pub(crate) mod ident_generator;

#[cfg(any(
    feature = "test",
    feature = "domain",
    feature = "email",
    feature = "number",
    feature = "signed_integer",
    feature = "unsigned_integer",
))]
#[allow(dead_code)]
pub(crate) mod allow;
#[cfg(any(
    feature = "test",
    feature = "bit",
    feature = "byte",
    feature = "number",
    feature = "signed_integer",
    feature = "unsigned_integer",
))]
#[allow(dead_code)]
pub(crate) mod boolean;
#[cfg(any(
    feature = "test",
    feature = "hex_color",
    feature = "hex_decoded",
    feature = "mac_address",
    feature = "uuid"
))]
#[allow(dead_code)]
pub(crate) mod case_option;
#[cfg(any(feature = "line", feature = "text"))]
#[allow(dead_code)]
pub(crate) mod length;
#[cfg(any(
    feature = "test",
    feature = "bit",
    feature = "byte",
    feature = "length",
    feature = "line",
    feature = "number",
    feature = "signed_integer",
    feature = "unsigned_integer",
    feature = "text",
))]
#[allow(dead_code)]
pub(crate) mod number;
#[cfg(any(
    feature = "test",
    feature = "bit",
    feature = "byte",
    feature = "number",
    feature = "signed_integer",
    feature = "unsigned_integer"
))]
#[allow(dead_code)]
pub(crate) mod range;
#[cfg(any(
    feature = "test",
    feature = "number",
    feature = "signed_integer",
    feature = "unsigned_integer"
))]
#[allow(dead_code)]
pub(crate) mod range_option;
#[cfg(any(feature = "test", feature = "mac_address", feature = "uuid"))]
#[allow(dead_code)]
pub(crate) mod separator_option;
#[cfg(any(
    feature = "test",
    feature = "base32",
    feature = "base32_decoded",
    feature = "base64",
    feature = "base64_decoded",
    feature = "base64_url",
    feature = "base64_url_decoded",
    feature = "domain",
    feature = "email",
    feature = "hex_color",
    feature = "host",
    feature = "http_url",
    feature = "http_ftp_url",
    feature = "ip",
    feature = "ipv4",
    feature = "ipv6",
    feature = "number",
    feature = "signed_integer",
    feature = "unsigned_integer",
))]
#[allow(dead_code)]
pub(crate) mod tri_allow;

#[cfg(feature = "rocket")]
#[allow(dead_code)]
pub(crate) mod rocket;

#[cfg(feature = "test")]
pub(crate) mod test;

use quote::ToTokens;
use syn::{Expr, Path};

#[inline]
pub(crate) fn path_to_string(path: &Path) -> String {
    path.into_token_stream().to_string().replace(' ', "")
}

/// Check whether a struct has the fields that a validator expects.
///
/// `Some(names)` means named fields with exactly these names in any order, and `None` means one unnamed field.
#[cfg(any(
    feature = "domain",
    feature = "email",
    feature = "host",
    feature = "ip",
    feature = "ipv4",
    feature = "ipv6"
))]
pub(crate) fn fields_match(fields: &syn::Fields, names: Option<&[&str]>) -> bool {
    match names {
        // Field names cannot repeat, so the same number of known names means the same names.
        Some(names) => {
            matches!(fields, syn::Fields::Named(_))
                && fields.len() == names.len()
                && fields.iter().all(|field| {
                    field.ident.as_ref().is_some_and(|ident| names.iter().any(|name| ident == name))
                })
        },
        None => matches!(fields, syn::Fields::Unnamed(_)) && fields.len() == 1,
    }
}

/// Remove the invisible groups that `macro_rules!` puts around fragments such as `$v:expr`.
#[inline]
pub(crate) fn peel_expr_groups(mut expr: &Expr) -> &Expr {
    while let Expr::Group(group) = expr {
        expr = &group.expr;
    }

    expr
}
