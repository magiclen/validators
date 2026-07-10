/// Determine whether the input domain is localhost.
#[inline]
pub fn is_local_domain<S: AsRef<str>>(s: S) -> bool {
    let s = s.as_ref();

    s.strip_suffix('.').unwrap_or(s).eq_ignore_ascii_case("localhost")
}

/// Determine whether the input domain is has at least two labels.
#[inline]
pub fn is_at_least_two_labels_domain<S: AsRef<str>>(s: S) -> bool {
    let s = s.as_ref();

    s.strip_suffix('.').unwrap_or(s).contains('.')
}
