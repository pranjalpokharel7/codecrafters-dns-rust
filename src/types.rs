use serde::{Serialize, Deserialize};

// record type is a 2-byte int
#[repr(u16)]
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecordType {
    A = 1,      // a host address
    NS = 2,     // authoritative name server
    MD = 3,     // mail destination (Obsolete - use MX)
    MF = 4,     // mail forwarder (Obsolete - use MX)
    CNAME = 5,  // canonical name for an alias
    SOA = 6,    // start of a zone of authority
    MB = 7,     // mailbox domain name (EXPERIMENTAL)
    MG = 8,     // mail group member (EXPERIMENTAL)
    MR = 9,     // mail rename domain name (EXPERIMENTAL)
    NULL = 10,  // null RR (EXPERIMENTAL)
    WKS = 11,   // well known service description
    PTR = 12,   // domain name pointer
    HINFO = 13, // host information
    MINFO = 14, // mailbox or mail list information
    MX = 15,    // mail exchange
    TXT = 16,   // text strings
}

#[repr(u16)]
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClassType {
    IN = 1, // the internet
    CS = 2, // CSNET class (Obsolete)
    CH = 3, // CHAOS class
    HS = 4, // Hesiod
}
