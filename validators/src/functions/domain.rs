/// Determine whether the input domain is localhost.
#[inline]
pub fn is_local_domain<S: AsRef<str>>(s: S) -> bool {
    let bytes = s.as_ref().as_bytes();

    debug_assert!(!bytes.is_empty());

    let length_dec = bytes.len() - 1;

    let bytes = if bytes[length_dec] == b'.' { &bytes[..length_dec] } else { bytes };

    bytes.eq_ignore_ascii_case(b"localhost")
}

/// Determine whether the input domain is has at least two labels.
#[inline]
pub fn is_at_least_two_labels_domain<S: AsRef<str>>(s: S) -> bool {
    let s = s.as_ref();

    debug_assert!(!s.is_empty());

    s
        .bytes()
        .take(s.len() - 1) // to avoid "."-ended domain
        .any(|e| e == b'.')
}
