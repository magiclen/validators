/// Used for the `http_ftp_url` validator to differentiate the protocol.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Protocol {
    HTTP,
    HTTPS,
    FTP,
}
