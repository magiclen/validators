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
