/// Determine whether the input domain is localhost or one of its subdomains.
#[inline]
pub fn is_local_domain<S: AsRef<str>>(s: S) -> bool {
    let domain = s.as_ref();
    let domain = domain.strip_suffix('.').unwrap_or(domain);
    let last_label = domain.rsplit_once('.').map_or(domain, |(_, label)| label);

    last_label.eq_ignore_ascii_case("localhost")
}

/// Determine whether the input domain has at least two labels.
#[inline]
pub fn is_at_least_two_labels_domain<S: AsRef<str>>(s: S) -> bool {
    let s = s.as_ref();

    s.strip_suffix('.').unwrap_or(s).contains('.')
}

/// Determine whether the input domain ends in a number, as the WHATWG URL Standard defines.
///
/// The last label is checked, and one ending dot is ignored.
/// The label is a number if it has only ASCII digits, or if it is `0x` or `0X` followed by zero or more ASCII hex digits.
/// A URL parser reads such a host as an IPv4 address (such as `127.1`), or rejects it (such as `example.123`).
/// A dotted-decimal IPv4 address such as `127.0.0.1` also ends in a number.
/// Call this function after IDNA normalization, because a name such as `１２７.１` becomes `127.1` only then.
#[inline]
pub fn ends_in_a_number<S: AsRef<str>>(s: S) -> bool {
    let domain = s.as_ref();
    let domain = domain.strip_suffix('.').unwrap_or(domain);
    let last_label = domain.rsplit_once('.').map_or(domain, |(_, label)| label);

    match last_label.as_bytes() {
        [] => false,
        [b'0', b'x' | b'X', hex_digits @ ..] => hex_digits.iter().all(u8::is_ascii_hexdigit),
        digits => digits.iter().all(u8::is_ascii_digit),
    }
}
