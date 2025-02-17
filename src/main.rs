mod header;

use header::DNSHeader;

#[allow(unused_imports)]
use std::net::UdpSocket;

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    // Uncomment this block to pass the first stage
    let udp_socket = UdpSocket::bind("127.0.0.1:2053").expect("Failed to bind to address");
    let mut buf = [0; 512];
    
    loop {
        match udp_socket.recv_from(&mut buf) {
            Ok((_size, source)) => {
                // println!("Received {} bytes from {}", size, source);

                let header = DNSHeader::from_bytes(&buf);
                let mut resp_header = header.clone();
                resp_header.clear_flags();
                resp_header.set_qr(true);

                // answer size
                resp_header.set_ancount(0);
                resp_header.set_arcount(0);

                let response = resp_header.to_bytes_vec();
                udp_socket
                    .send_to(&response, source)
                    .expect("Failed to send response");
            }
            Err(e) => {
                eprintln!("Error receiving data: {}", e);
                break;
            }
        }
    }
}
