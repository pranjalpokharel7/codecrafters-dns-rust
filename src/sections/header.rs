use serde::{ Deserialize, Serialize };
use rand::prelude::*;

use crate::errors::DeserializationError;

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

    pub fn from_bytes(buf: &[u8]) -> Result<Self, DeserializationError> {
        if buf.len() < 12 {
            return Err(DeserializationError::UnexpectedEOF);
        }

        Ok(Self {
            pid: u16::from_be_bytes([buf[0], buf[1]]),
            flags: u16::from_be_bytes([buf[2], buf[3]]),
            qdcount: u16::from_be_bytes([buf[4], buf[5]]),
            ancount: u16::from_be_bytes([buf[6], buf[7]]),
            nscount: u16::from_be_bytes([buf[8], buf[9]]),
            arcount: u16::from_be_bytes([buf[10], buf[11]]),
        })
    }

    pub fn to_bytes(&self) -> [u8; 12] {
        let mut bytes = [0u8; 12];

        bytes[0..2].copy_from_slice(&self.pid.to_be_bytes());
        bytes[2..4].copy_from_slice(&self.flags.to_be_bytes());
        bytes[4..6].copy_from_slice(&self.qdcount.to_be_bytes());
        bytes[6..8].copy_from_slice(&self.ancount.to_be_bytes());
        bytes[8..10].copy_from_slice(&self.nscount.to_be_bytes());
        bytes[10..12].copy_from_slice(&self.arcount.to_be_bytes());

        bytes
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
        assert_eq!(header.to_bytes(), [0, 0, 128, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
        assert_eq!(header.flags, 0x8000);
    }
}
