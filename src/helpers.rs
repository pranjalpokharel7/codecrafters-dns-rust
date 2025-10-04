use std::net::UdpSocket;

use rand::prelude::*;

use crate::message::DNSMessage;

/// utility function to get us a random IP address
/// note: for now, we are not worried about getting conflicting addresses for multiple runs
pub fn random_ip() -> Vec<u8> {
    let mut rng = rand::rng();
    rng.random::<u32>().to_be_bytes().to_vec()
}

pub fn query_nameserver(query: &DNSMessage, nameserver: &str) -> anyhow::Result<DNSMessage> {
    let socket = UdpSocket::bind("0.0.0.0:0")?;
    socket.send_to(&query.to_bytes(), nameserver)?;

    let mut response = vec![0u8; 512];
    socket.recv_from(&mut response)?;

    Ok(DNSMessage::from_bytes(&response)?)
}
