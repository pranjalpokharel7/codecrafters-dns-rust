use crate::{
    helpers::with_bincode,
    types::{ClassType, RecordType},
};
use bincode::Options;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct DNSQuestion {
    name: Vec<u8>,
    record_type: RecordType,
    class: ClassType,
}

impl DNSQuestion {
    pub fn new(name: Vec<u8>) -> Self {
        Self {
            name,
            record_type: RecordType::A,
            class: ClassType::IN,
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        // TODO: do not unwrap, custom error types
        let mut bytes = vec![];
        bytes.extend(&self.name);
        bytes.extend(
            with_bincode()
                .serialize(&(self.record_type as u16))
                .unwrap(),
        );
        bytes.extend(with_bincode().serialize(&(self.class as u16)).unwrap());
        bytes
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_name() {
        let name = "\x06google\x03com\x00".as_bytes().to_vec();
        let question = DNSQuestion::new(name);
        assert_eq!(question.record_type as u16, 1);

        let b = question.to_bytes();
        assert_eq!(
            b,
            vec![
                0x06, 0x67, 0x6f, 0x6f, 0x67, 0x6c, 0x65, 0x03, 0x63, 0x6f, 0x6d, 0x00, 0x00, 0x01,
                0x00, 0x01
            ]
        )
    }
}
