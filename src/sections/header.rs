use bincode::Options;
use serde::{ Deserialize, Serialize };
use rand::prelude::*;

use crate::helpers::big_endian;

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Default)]
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
        let mut rng = rand::rng();

        // assign random pid
        let mut header = Self::default();
        header.pid = rng.random::<u16>();

        header
    }

    // STREAM: remove bincode package - adds unnecessary build time

    pub fn from_bytes(bytes: &[u8]) -> Self {
        big_endian()
            .deserialize(&bytes[..12])
            .unwrap()
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        big_endian().serialize(&self).unwrap()
    }

    /// Set response flags using values from request header.
    pub fn from_request_header(&mut self, request_header: &Self) {
        self.pid = request_header.pid;

        self.clear_flags();
        self.set_qr(true);
        self.set_opcode(request_header.get_opcode());
        self.set_rd(request_header.get_rd());

        if request_header.get_opcode() != 0 {
            self.set_rcode(0x4);
        }
    }

    pub fn clear_flags(&mut self) {
        self.flags = 0;
    }

    pub fn set_qr(&mut self, set: bool) {
        // clear flag bit at MSB and set qr
        self.flags = (self.flags & 0x7fff) | ((set as u16) << 15);
    }

    // opcode is the lower half of u8 -> 4 bits
    pub fn set_opcode(&mut self, opcode: u16) {
        let op_bits = opcode & 0xf;
        self.flags = (self.flags & 0x87ff) | (op_bits << 11);
    }

    pub fn get_opcode(&self) -> u16 {
        (self.flags & 0x7800) >> 11
    }

    pub fn set_rd(&mut self, set: bool) {
        self.flags = (self.flags & 0xfeff) | ((set as u16) << 8);
    }

    // should I not make this bool?
    pub fn get_rd(&self) -> bool {
        ((self.flags & 0x0100) >> 8) == 0x1
    }

    pub fn set_rcode(&mut self, rcode: u16) {
        let rcode_bits = rcode & 0xf;
        self.flags = (self.flags & 0xfff0) | rcode_bits;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_qr() {
        let mut header = DNSHeader::new();
        header.set_qr(true);
        assert_eq!(header.to_bytes(), vec![0, 0, 128, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
        assert_eq!(header.flags, 0x8000);
    }
}
