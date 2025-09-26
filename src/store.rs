use std::collections::HashMap;

// we need to do a different stream for data structure implementations
pub struct DNSStore {
    _store: HashMap<Vec<u8>, Vec<u8>>,
}

impl DNSStore {
    pub fn init() -> Self {
        let mut _store = HashMap::new();

        // initialize with some pre-filled DNS entries
        // TODO: move this to a file
        _store.insert("\x0ccodecrafters\x02io\x00".as_bytes().to_vec(), vec![192, 168, 1, 10]);

        Self {
            _store,
        }
    }

    #[allow(unused)]
    pub fn insert(&mut self, name: Vec<u8>, addr4: Vec<u8>) {
        self._store.insert(name, addr4);
    }

    pub fn lookup(&self, name: &[u8]) -> Option<&Vec<u8>> {
        self._store.get(name)
    }
}
