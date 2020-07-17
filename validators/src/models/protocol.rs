#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Protocol {
    HTTP,
    HTTPS,
    FTP,
}
