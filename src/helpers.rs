use bincode::{self, Options};

pub fn with_bincode() -> impl Options {
    bincode::DefaultOptions::new()
        .with_fixint_encoding()
        .with_big_endian()
}
