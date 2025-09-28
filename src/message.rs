use crate::sections::{ answer::DNSAnswer, header::DNSHeader, question::DNSQuestion };

pub struct DNSMessage {
    pub header: DNSHeader,
    pub questions: Vec<DNSQuestion>,
    pub answers: Vec<DNSAnswer>,
}

impl DNSMessage {
    pub fn new() -> Self {
        Self {
            header: DNSHeader::new(),
            questions: vec![],
            answers: vec![],
        }
    }

    pub fn from_request_buffer(buf: &[u8]) -> Self {
        let header = DNSHeader::from_bytes(buf);
        let mut questions = vec![];

        let mut pos: usize = 12; // header size
        let buf_size = buf.len(); // read bytes?
        while pos < buf_size {
            // pass position as a parameter, and 
            match DNSQuestion::from_bytes(&buf, pos) {
                Ok(question) => {
                    pos = pos + question.name.len() + 4;
                    questions.push(question);
                }
                Err(err) => {
                    eprintln!("{:?}", err);
                    break;
                }
            }
        }

        Self {
            header,
            questions,
            answers: vec![],
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = vec![];
        bytes.extend(self.header.to_bytes());

        for question in self.questions.iter() {
            bytes.extend(question.to_bytes());
        }

        for answer in self.answers.iter() {
            bytes.extend(answer.to_bytes());
        }

        bytes
    }
}
