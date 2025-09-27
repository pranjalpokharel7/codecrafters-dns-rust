mod sections;
mod helpers;
mod store;
mod message;
mod types;

use sections::answer::DNSAnswer;
use store::DNSStore;
use message::DNSMessage;
use std::net::UdpSocket;
use helpers::random_ip;

fn main() {
    let udp_socket = UdpSocket::bind("127.0.0.1:2053").expect("Failed to bind to address");
    let mut buf = [0; 512];

    // initialize store
    let store = DNSStore::init();

    loop {
        match udp_socket.recv_from(&mut buf) {
            Ok((_size, source)) => {
                // client request
                let request = DNSMessage::from_request_buffer(&buf);

                // server response
                let mut response = DNSMessage::new();

                // set response flags
                response.header.from_request_header(&request.header);

                // set the question/answers section value
                for question in &request.questions {
                    let domain_name = question.name.to_vec();
                    response.questions.push(question.clone());

                    if let Some(record) = store.lookup(&domain_name) {
                        response.answers.push(DNSAnswer::new(domain_name, record.to_vec()));
                    } else {
                        response.answers.push(DNSAnswer::new(domain_name, random_ip()));
                    }
                }

                // set questions and answers count
                response.header.qdcount = response.questions.len() as u16;
                response.header.ancount = response.answers.len() as u16;

                udp_socket.send_to(&response.to_bytes(), source).expect("Failed to send response");
            }
            Err(e) => {
                eprintln!("Error receiving data: {}", e);
                break;
            }
        }
    }
}
