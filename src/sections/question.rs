use crate::{ errors::DeserializationError, types::{ ClassType, RecordType } };
use num_enum::TryFromPrimitive;
use serde::{ Deserialize, Serialize };

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DNSQuestion {
    pub name: Vec<u8>,
    pub record_type: RecordType,
    pub class_type: ClassType,
}

// ways a label can end
// 1. with a zero octet
// 2. with a pointer
// we'll only decompress it back to the original domain name
// \0def\c0\10
// 10 -> 0001 0000 -> 16
// codecrafters.io
fn _parse_domain_name(buf: &[u8], start: usize) -> Result<(Vec<u8>, usize), DeserializationError> {
    let mut pos = start;
    let mut name = vec![];

    while pos < buf.len() {
        let length_byte = buf[pos];

        // first condition of exit: encountered zero octet
        if length_byte == b'\x00' {
            name.push(b'\x00');
            pos += 1; // move past null byte
            break;
        }

        let is_compressed_pointer = ((length_byte & 0xc0) >> 6) == 0b11;
        if is_compressed_pointer {
            let ptr = (u16::from_be_bytes([buf[pos], buf[pos + 1]]) & 0x3fff) as usize;
            let (label, _) = _parse_domain_name(&buf, ptr)?;
            name.extend(label);
            pos += 2; // move past null byte
            break; // second condition of exit: encountered pointer
        } else {
            // length byte is also included in label so we add 1
            let end = pos + (length_byte as usize) + 1;
            name.extend(&buf[pos..end]);
            pos = end;
        }
    }

    Ok((name, pos))
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

    pub fn from_bytes(buf: &[u8], start: usize) -> Result<Self, DeserializationError> {
        let (name, pos) = _parse_domain_name(buf, start)?;

        if pos + 3 >= buf.len() {
            return Err(DeserializationError::UnexpectedEOF);
        }

        let record_type = RecordType::try_from_primitive(
            u16::from_be_bytes([buf[pos], buf[pos + 1]])
        ).map_err(|_| DeserializationError::MalformedField("record_type"))?;

        let class_type = ClassType::try_from_primitive(
            u16::from_be_bytes([buf[pos + 2], buf[pos + 3]])
        ).map_err(|_| DeserializationError::MalformedField("class_type"))?;

        Ok(Self {
            name,
            record_type,
            class_type,
        })
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
