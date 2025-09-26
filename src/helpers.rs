use bincode::{self, Options};

pub fn big_endian() -> impl Options {
    bincode::DefaultOptions::new()
        .with_fixint_encoding()
        .with_big_endian()
}
