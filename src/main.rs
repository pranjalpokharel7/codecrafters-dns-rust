mod header;
mod question;
mod packet;
mod types;
mod helpers;

use header::DNSHeader;
use question::DNSQuestion;
use packet::DNSPacket;

#[allow(unused_imports)]
use std::net::UdpSocket;

fn main() {
    let udp_socket = UdpSocket::bind("127.0.0.1:2053").expect("Failed to bind to address");
    let mut buf = [0; 512];

    loop {
        match udp_socket.recv_from(&mut buf) {
            Ok((_size, source)) => {
                // client query
                // TODO: cast this as a DNS packet instead
                let query_header = DNSHeader::from_bytes(&buf);

                // server response
                let mut response = DNSPacket::new();
                response.header.pid = query_header.pid;
                response.header.clear_flags();
                response.header.set_qr(true);

                response.header.ancount = 0;
                response.header.arcount = 0;

                // set the question section value
                let question = DNSQuestion::new("\x0ccodecrafters\x02io\x00".as_bytes().to_vec());
                response.questions.push(question);
                response.header.qdcount = response.questions.len() as u16;

                udp_socket
                    .send_to(&response.to_bytes(), source)
                    .expect("Failed to send response");
            }
            Err(e) => {
                eprintln!("Error receiving data: {}", e);
                break;
            }
        }
    }
}
