use crate::types::{ ClassType, RecordType };
use num_enum::TryFromPrimitive;
use serde::{ Deserialize, Serialize };

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DNSQuestion {
    pub name: Vec<u8>,
    pub record_type: RecordType,
    pub class_type: ClassType,
}

impl DNSQuestion {
    #[allow(unused)]
    pub fn new(name: Vec<u8>) -> Self {
        Self {
            name,
            record_type: RecordType::A,
            class_type: ClassType::IN,
        }
    }

    pub fn from_bytes(buf: &[u8]) -> Self {
        let pos = buf
            .iter()
            .position(|b| *b == b'\x00')
            .unwrap();

        let name = buf[0..pos + 1].to_vec();
        let record_type = RecordType::try_from_primitive(
            u16::from_be_bytes([buf[pos + 1], buf[pos + 2]])
        ).unwrap();
        let class = ClassType::try_from_primitive(
            u16::from_be_bytes([buf[pos + 3], buf[pos + 4]])
        ).unwrap();

        Self {
            name,
            record_type,
            class_type: class,
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = vec![];
        bytes.extend(&self.name);
        bytes.extend((self.record_type as u16).to_be_bytes());
        bytes.extend((self.class_type as u16).to_be_bytes());
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
                0x06,
                0x67,
                0x6f,
                0x6f,
                0x67,
                0x6c,
                0x65,
                0x03,
                0x63,
                0x6f,
                0x6d,
                0x00,
                0x00,
                0x01,
                0x00,
                0x01
            ]
        )
    }
}
