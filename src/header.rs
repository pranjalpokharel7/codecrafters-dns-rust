// first 48 bits -> 6 bytes comprise of DNS Header
// can use copy trait because all fields implement the copy trait
#[derive(Debug, Clone, Copy)]
pub struct DNSHeader {
    pid: u16,
    flags: u16, // will be broken down later on
    qdcount: u16,
    ancount: u16,
    nscount: u16,
    arcount: u16,
}

macro_rules! u16_to_byte_buffer {
    ($u16:expr, $buf:expr) => {
        for byte in $u16.to_be_bytes() {
            $buf.push(byte);
        }
    };
}

impl DNSHeader {
    pub fn from_bytes(header: &[u8]) -> Self {
        debug_assert!(header.len() >= 12);
        Self {
            pid: u16::from_be_bytes([header[0], header[1]]),
            flags: u16::from_be_bytes([header[2], header[3]]),
            qdcount: u16::from_be_bytes([header[4], header[5]]),
            ancount: u16::from_be_bytes([header[6], header[7]]),
            nscount: u16::from_be_bytes([header[8], header[9]]),
            arcount: u16::from_be_bytes([header[10], header[11]]),
        }
    }

    pub fn to_bytes_vec(&self) -> Vec<u8> {
        let mut buf = Vec::new();
        u16_to_byte_buffer!(self.pid, buf);
        u16_to_byte_buffer!(self.flags, buf);
        u16_to_byte_buffer!(self.qdcount, buf);
        u16_to_byte_buffer!(self.ancount, buf);
        u16_to_byte_buffer!(self.nscount, buf);
        u16_to_byte_buffer!(self.arcount, buf);
        buf
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

    pub fn set_rcode(&mut self, rcode: u16) {
        let op_bits = rcode & 0xf;
        self.flags = (self.flags & 0x87ff) | op_bits;
    }

    pub fn set_aa(&mut self, set: bool) {
        self.flags = (self.flags & 0xfbff) | ((set as u16) << 10);
    }

    pub fn set_tc(&mut self, set: bool) {
        self.flags = (self.flags & 0xfdff) | ((set as u16) << 9);
    }

    pub fn set_rd(&mut self, set: bool) {
        self.flags = (self.flags & 0xfeff) | ((set as u16) << 8);
    }

    pub fn set_ra(&mut self, set: bool) {
        self.flags = (self.flags & 0xff7f) | ((set as u16) << 7);
    }

    pub fn set_nscount(&mut self, count: u16) {
        self.nscount = count;
    }

    pub fn set_qdcount(&mut self, count: u16) {
        self.qdcount = count;
    }

    pub fn set_ancount(&mut self, count: u16) {
        self.ancount = count;
    }

    pub fn set_arcount(&mut self, count: u16) {
        self.arcount = count;
    }
}
