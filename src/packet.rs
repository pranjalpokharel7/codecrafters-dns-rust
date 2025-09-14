use crate::{header::DNSHeader, question::DNSQuestion};

pub struct DNSPacket {
    pub header: DNSHeader,
    pub questions: Vec<DNSQuestion>,
}

impl DNSPacket {
    pub fn new() -> Self {
        Self {
            header: DNSHeader::new(),
            questions: vec![],
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = vec![];
        bytes.extend(self.header.to_bytes());
        for question in self.questions.iter() {
            bytes.extend(question.to_bytes());
        }
        bytes
    }
}
