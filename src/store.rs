use std::collections::HashMap;

// we need to do a different stream for data structure implementations
pub struct DNSStore {
    _tree: HashMap<Vec<u8>, Vec<u8>>,
}

impl DNSStore {
    pub fn init() -> Self {
        Self {
            _tree: HashMap::new(),
        }
    }

    pub fn insert(&mut self, name: Vec<u8>, addr4: Vec<u8>) {
        self._tree.insert(name, addr4);
    }

    pub fn lookup(&self, name: &[u8]) -> Option<&Vec<u8>> {
        self._tree.get(name)
    }
}
