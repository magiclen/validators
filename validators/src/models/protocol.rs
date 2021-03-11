#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[allow(clippy::upper_case_acronyms)]
pub enum Protocol {
    HTTP,
    HTTPS,
    FTP,
}
