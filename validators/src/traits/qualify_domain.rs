/// The `domain` validator will implement this for its types.
pub trait QualifyDomain {
    /// Determine whether the domain is fully qualified.
    fn is_fully_qualified(&self) -> bool;

    /// Get the non fully qualified part of this domain.
    fn get_domain_non_fully_qualified(&self) -> &str;
}
