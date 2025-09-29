use crate::{
    errors::DeserializationError,
    sections::parser::{ parse_domain_name, parse_u16_from_be_bytes, parse_u32_from_be_bytes },
    types::{ ClassType, RecordType },
};
use num_enum::TryFromPrimitive;

#[derive(Debug)]
pub struct DNSAnswer {
    pub domain_name: Vec<u8>, // pointer to the question section (or in general bytes) where the name is
    pub record_type: RecordType,
    pub class_type: ClassType,
    pub ttl: u32,
    pub rdlength: u16,
    pub rdata: Vec<u8>, // this is 4 bytes because ipv4 requires 4 bytes (x.x.x.x) - could we use u32?
}

impl DNSAnswer {
    pub fn new(name: Vec<u8>, rdata: Vec<u8>) -> Self {
        Self {
            domain_name: name.to_vec(),
            record_type: RecordType::A,
            class_type: ClassType::IN,
            ttl: 60,
            rdlength: rdata.len() as u16,
            rdata: rdata.to_vec(),
        }
    }

    pub fn from_bytes(buf: &[u8], start: usize) -> Result<(Self, usize), DeserializationError> {
        let (domain_name, pos) = parse_domain_name(buf, start)?;

        let record_type = RecordType::try_from_primitive(
            parse_u16_from_be_bytes(&buf[pos..])?
        ).map_err(|_| DeserializationError::MalformedField("record_type"))?;

        let class_type = ClassType::try_from_primitive(
            parse_u16_from_be_bytes(&buf[pos + 2..])?
        ).map_err(|_| DeserializationError::MalformedField("class_type"))?;

        let ttl = parse_u32_from_be_bytes(&buf[pos + 4..])?;
        let rdlength = parse_u16_from_be_bytes(&buf[pos + 8..])?;

        let rdata_start = pos + 10;
        let rdata = buf[rdata_start..rdata_start + (rdlength as usize)].to_vec();

        // number of bytes parsed from buffer
        let bytes_parsed = pos + 10 + (rdlength as usize) - start;

        Ok((
            Self {
                domain_name,
                record_type,
                class_type,
                ttl,
                rdlength,
                rdata,
            },
            bytes_parsed,
        ))
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = vec![];

        bytes.extend(&self.domain_name);
        bytes.extend((self.record_type as u16).to_be_bytes());
        bytes.extend((self.class_type as u16).to_be_bytes());
        bytes.extend(self.ttl.to_be_bytes());
        bytes.extend(self.rdlength.to_be_bytes());
        bytes.extend(&self.rdata);

        bytes
    }
}
