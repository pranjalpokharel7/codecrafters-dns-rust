use std::collections::HashMap;

// STREAM: replace with trie data structure and pre-load records from a file
pub struct DNSStore {
    _store: HashMap<Vec<u8>, Vec<u8>>,
}

impl DNSStore {
    pub fn init() -> Self {
        Self {
            _store: HashMap::new(),
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
