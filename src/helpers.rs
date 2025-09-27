use bincode::{ self, Options };

use rand::prelude::*;

pub fn big_endian() -> impl Options {
    bincode::DefaultOptions::new().with_fixint_encoding().with_big_endian()
}

/// utility function to get us a random IP address 
/// note: for now, we are not worried about getting conflicting addresses for multiple runs
pub fn random_ip() -> Vec<u8> {
    let mut rng = rand::rng();
    let ip_u32 = rng.random::<u32>();

    vec![
        (ip_u32 & 0xff000000) as u8,
        (ip_u32 & 0xff0000) as u8,
        (ip_u32 & 0xff00) as u8,
        (ip_u32 & 0xff) as u8
    ]
}
