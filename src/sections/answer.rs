use crate::types::{ClassType, RecordType};

pub struct DNSAnswer {
    pub name: Vec<u8>, // pointer to the question section (or in general bytes) where the name is
    pub record_type: RecordType,
    pub class_type: ClassType,
    pub ttl: i32,
    pub rdlength: u16,
    pub rdata: Vec<u8> // this is 4 bytes because ipv4 requires 4 bytes (x.x.x.x) - could we use u32?
}

impl DNSAnswer {
    pub fn new(name: &[u8], addr4: &[u8]) -> Self {
        Self {
            name: name.to_vec(),
            record_type: RecordType::A,
            class_type: ClassType::IN,
            rdlength: addr4.len() as u16,
            ttl: 60,
            rdata: addr4.to_vec(),
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = vec![];
        
        bytes.extend(&self.name);
        bytes.extend((self.record_type as u16).to_be_bytes());
        bytes.extend((self.class_type as u16).to_be_bytes());
        bytes.extend(self.ttl.to_be_bytes());
        bytes.extend(self.rdlength.to_be_bytes());
        bytes.extend(&self.rdata);

        bytes
    }
}

