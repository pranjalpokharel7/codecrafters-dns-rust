use crate::{
    errors::DeserializationError,
    sections::{ answer::DNSAnswer, header::DNSHeader, question::DNSQuestion },
};

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

    pub fn add_question(&mut self, question: DNSQuestion) {
        self.questions.push(question);
        self.header.qdcount = self.questions.len() as u16;
    }

    pub fn add_multiple_questions(&mut self, questions: Vec<DNSQuestion>) {
        self.questions.extend(questions);
        self.header.qdcount = self.questions.len() as u16;
    }

    pub fn add_multiple_answers(&mut self, answers: Vec<DNSAnswer>) {
        self.answers.extend(answers);
        self.header.ancount = self.answers.len() as u16;
    }

    pub fn add_answer(&mut self, answer: DNSAnswer) {
        self.answers.push(answer);
        self.header.ancount = self.answers.len() as u16;
    }

    pub fn from_bytes(buf: &[u8]) -> Result<Self, DeserializationError> {
        let buf_size = buf.len();
        if buf_size < 12 {
            return Err(DeserializationError::UnexpectedEOF);
        }

        let header = DNSHeader::from_bytes(buf);
        let mut pos: usize = 12; // header size

        let mut questions = Vec::with_capacity(header.qdcount as usize);
        while questions.len() < (header.qdcount as usize) && pos < buf_size {
            let (question, bytes_parsed) = DNSQuestion::from_bytes(&buf, pos)?;
            pos += bytes_parsed;
            questions.push(question);
        }

        let mut answers = Vec::with_capacity(header.ancount as usize);
        while answers.len() < (header.ancount as usize) && pos < buf_size {
            let (answer, bytes_parsed) = DNSAnswer::from_bytes(&buf, pos)?;
            pos += bytes_parsed;
            answers.push(answer);
        }

        Ok(Self {
            header,
            questions,
            answers,
        })
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
