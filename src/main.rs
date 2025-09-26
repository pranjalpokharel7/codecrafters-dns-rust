mod sections;
mod helpers;
mod store;
mod message;
mod types;

use sections::answer::DNSAnswer;
use store::DNSStore;
use message::DNSMessage;
use std::net::UdpSocket;

fn main() {
    let udp_socket = UdpSocket::bind("127.0.0.1:2053").expect("Failed to bind to address");
    let mut buf = [0; 512];

    loop {
        match udp_socket.recv_from(&mut buf) {
            Ok((_size, source)) => {
                // client query
                let client_query = DNSMessage::from_request_buffer(&buf);

                // server response
                let mut response = DNSMessage::new();
                response.header.pid = client_query.header.pid;
                response.header.clear_flags();
                response.header.set_qr(true);

                // initialize from file
                let mut store = DNSStore::init();
                store.insert(
                    "\x0ccodecrafters\x02io\x00".as_bytes().to_vec(),
                    vec![192, 168, 1, 10]
                );

                // set the question section value
                for client_question in &client_query.questions {
                    let domain_name = &client_question.name;
                    response.questions.push(client_question.clone());

                    if let Some(record) = store.lookup(&domain_name) {
                        response.answers.push(DNSAnswer::new(domain_name, record));
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
