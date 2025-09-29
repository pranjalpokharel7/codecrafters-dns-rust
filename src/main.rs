mod sections;
mod helpers;
mod store;
mod message;
mod types;
mod errors;
mod args;

use sections::answer::DNSAnswer;
use store::DNSStore;
use message::DNSMessage;
use std::net::UdpSocket;
use helpers::random_ip;

use crate::{args::Args, helpers::query_nameserver};

fn main() {
    let udp_socket = UdpSocket::bind("127.0.0.1:2053").expect("Failed to bind to address");
    let mut buf = [0; 512];

    // initialize store
    let mut store = DNSStore::init();

    // initialize cli
    let args = Args::init();

    loop {
        match udp_socket.recv_from(&mut buf) {
            Ok((size, source)) => {
                // client request
                let request = DNSMessage::from_bytes(&buf[..size]);

                // server response
                let mut response = DNSMessage::new();

                // set response flags
                response.header.from_request_header(&request.header);

                // set the question/answers section value
                for question in &request.questions {
                    let domain_name = question.domain_name.to_vec();
                    if let Some(record) = store.lookup(&domain_name) {
                        response.answers.push(DNSAnswer::new(domain_name, record.to_vec()));
                    } else {
                        if let Some(ns) = &args.resolver {
                            let mut ns_query = DNSMessage::new();
                            ns_query.questions.push(question.clone());
                            ns_query.header.qdcount = 1;

                            if let Ok(record) = query_nameserver(&ns_query, ns) {
                                for answer in record.answers {
                                    response.answers.push(answer);
                                }
                            };
                        } else {
                            let ip = random_ip();
                            store.insert(domain_name.clone(), ip.clone());
                            response.answers.push(DNSAnswer::new(domain_name, ip));
                        }
                    };
                }

                response.questions.extend(request.questions);

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
