use bincode::Options;
use serde::{Deserialize, Serialize};

use crate::helpers::big_endian;

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct DNSHeader {
    pub pid: u16,
    pub flags: u16,
    pub qdcount: u16,
    pub ancount: u16, 
    pub nscount: u16,
    pub arcount: u16,
}

impl DNSHeader {
    pub fn new() -> Self {
        Self {
            pid: 0,
            flags: 0,
            qdcount: 0,
            ancount: 0,
            nscount: 0,
            arcount: 0,
        }
    }

    pub fn from_bytes(bytes: &[u8]) -> Self {
        big_endian().deserialize(&bytes[..12]).unwrap()
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        big_endian().serialize(&self).unwrap()
    }

    pub fn clear_flags(&mut self) {
        self.flags = 0;
    }

    pub fn set_qr(&mut self, set: bool) {
        // clear flag bit at MSB and set qr
        self.flags = (self.flags & 0x7fff) | ((set as u16) << 15);
    }

    // opcode is the lower half of u8 -> 4 bits
    #[allow(unused)]
    pub fn set_opcode(&mut self, opcode: u16) {
        let op_bits = opcode & 0xf;
        self.flags = (self.flags & 0x87ff) | (op_bits << 11);
    }

    #[allow(unused)]
    pub fn set_rcode(&mut self, rcode: u16) {
        let op_bits = rcode & 0xf;
        self.flags = (self.flags & 0x87ff) | op_bits;
    }

    #[allow(unused)]
    pub fn set_aa(&mut self, set: bool) {
        self.flags = (self.flags & 0xfbff) | ((set as u16) << 10);
    }

    #[allow(unused)]
    pub fn set_tc(&mut self, set: bool) {
        self.flags = (self.flags & 0xfdff) | ((set as u16) << 9);
    }

    #[allow(unused)]
    pub fn set_rd(&mut self, set: bool) {
        self.flags = (self.flags & 0xfeff) | ((set as u16) << 8);
    }

    #[allow(unused)]
    pub fn set_ra(&mut self, set: bool) {
        self.flags = (self.flags & 0xff7f) | ((set as u16) << 7);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_qr() {
        let mut header = DNSHeader::new();
        header.set_qr(true);
        assert_eq!(
            header.to_bytes(),
            vec![0, 0, 128, 0, 0, 0, 0, 0, 0, 0, 0, 0]
        );
        assert_eq!(header.flags, 0x8000);
    }
}
