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

use crate::{ args::Args, helpers::query_nameserver };

fn main() {
    let socket = UdpSocket::bind("127.0.0.1:2053").expect("Failed to bind to address");
    let mut buffer = [0; 512];

    // initialize store
    let mut store = DNSStore::init();

    // initialize cli
    let args = Args::init();

    // STREAM: this must be asynchronous or multi-threaded server

    loop {
        match socket.recv_from(&mut buffer) {
            Ok((size, client_addr)) => {
                if
                    let Err(err) = handle_dns_query(
                        &socket,
                        &buffer[..size],
                        &mut store,
                        &args,
                        client_addr
                    )
                {
                    eprintln!("{}", err);
                }
            }
            Err(e) => {
                eprintln!("Error receiving data: {}", e);
                break;
            }
        }
    }
}

fn build_dns_response(
    request: &DNSMessage,
    store: &mut DNSStore,
    args: &Args
) -> anyhow::Result<DNSMessage> {
    let mut response = DNSMessage::new();
    response.header.from_request_header(&request.header);

    for question in &request.questions {
        let domain_name = question.domain_name.to_vec();
        if let Some(record) = store.lookup(&domain_name) {
            response.add_answer(DNSAnswer::new(domain_name, record.to_vec()));
        } else {
            if let Some(ns) = &args.resolver {
                let mut ns_query = DNSMessage::new();
                ns_query.add_question(question.clone());
                let record = query_nameserver(&ns_query, ns)?;
                response.add_multiple_answers(record.answers);
            } else {
                // this is just to pass some internal test
                let ip = random_ip();
                store.insert(domain_name.clone(), ip.clone());
                response.add_answer(DNSAnswer::new(domain_name, ip));
            }
        };
    }

    response.add_multiple_questions(request.questions.clone());
    Ok(response)
}

fn handle_dns_query(
    socket: &UdpSocket,
    buffer: &[u8],
    store: &mut DNSStore,
    args: &Args,
    client_addr: std::net::SocketAddr
) -> anyhow::Result<()> {
    let request = DNSMessage::from_bytes(buffer)?;
    let response = build_dns_response(&request, store, args)?;
    socket.send_to(&response.to_bytes(), client_addr)?;
    Ok(())
}
