use std::vec;

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

    pub fn from_bytes(buf: &[u8]) -> Self {
        let buf_size = buf.len();
        let header = DNSHeader::from_bytes(buf);
        let mut pos: usize = 12; // header size

        let mut questions = vec![];
        while questions.len() < (header.qdcount as usize) && pos < buf_size {
            match DNSQuestion::from_bytes(&buf, pos) {
                Ok((question, bytes_parsed)) => {
                    pos += bytes_parsed;
                    questions.push(question);
                }
                Err(err) => {
                    eprintln!("{:?}", err);
                    break;
                }
            }
        }

        let mut answers = vec![];
        while answers.len() < (header.ancount as usize) && pos < buf_size {
            match DNSAnswer::from_bytes(&buf, pos) {
                Ok((answer, bytes_parsed)) => {
                    pos += bytes_parsed;
                    answers.push(answer);
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
            answers,
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
