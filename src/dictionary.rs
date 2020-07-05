#![crate_name = "doc"]

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

static FREE_SLOT: i64 = -1;

#[derive(Debug)]
pub struct Dictionary {
    capacity: u64,
    size: u64,
    hash_table: Vec<i64>,
    keys: Vec<i32>, // FIXME make it generic
}

impl Dictionary {
    /// Constructs a new empty dictionary of size 0 with the given initial capacity.
    fn new(initial_capacity: u64) -> Dictionary {
        Dictionary {
            capacity: initial_capacity,
            size: 0,
            hash_table: vec![FREE_SLOT; initial_capacity as usize],
            keys: vec![0; initial_capacity as usize],
        }
    }

    /// Resizes the hash table to the given capacity by re-hashing all of the keys.
    fn resize(&mut self, new_capacity: u64) {
        let mut tmp = Dictionary::new(new_capacity);

        // Remap
        for i in 0..self.size {
            let pos = i as usize;
            let x = self.keys[pos];
            tmp.map(x);
        }

        self.keys = tmp.keys;
        self.hash_table = tmp.hash_table;
        self.size = tmp.size;
        self.capacity = tmp.capacity;
    }

    /// Computes the hash of the given key.
    fn hash(&self, key: i32) -> u64 {
        let mut s = DefaultHasher::default();
        key.hash(&mut s);
        s.finish() & (self.capacity - 1)
    }

    /// Inserts the key into the dictionary and returns its position. If the key is already in the
    /// dictionary, it simply returns its position.
    ///
    /// # Example:
    /// FIXME this code does not compile because the import but I don't know why
    /// ```ignore
    /// use dictionary::Dictionary;
    ///
    /// let mut dictionary = Dictionary::new(4);
    /// assert_eq!(dictionary.map(5), 0);
    /// assert_eq!(dictionary.map(5), 0);
    /// ```
    pub fn map(&mut self, key: i32) -> u64 {
        // Double the table size if 50% full
        if self.size >= (self.capacity >> 1) {
            self.resize(self.capacity << 1); // TODO make sure it does not overflow
        }

        let mut index = self.hash(key);
        while let Some(address) = self.hash_table.get(index as usize) {
            match self.keys.get(*address as usize) {
                None => { break; }
                Some(k) => {
                    if *k == key { // FIXME equals when generic
                        return *address as u64;
                    }
                    index = (index + 1) & (self.capacity - 1); // linear probing, find a new available slot
                }
            };
        }
        if let Some(pos) = self.get(key) {
            pos as u64
        } else {
            let new_address = self.size;
            println!("setting key: {}, at address: {}", key, new_address);
            self.keys[new_address as usize] = key;
            println!("size of keys: {}", self.keys.len());
            self.hash_table[index as usize] = new_address as i64;
            self.size += 1;
            new_address
        }
    }

    /// Reads the key stored at a given dictionary position or return -1 if there is no key stored
    /// at the given position.
    pub fn read(&self, position: u64) -> i32 {
        match self.keys.get(position as usize) {
            None => { FREE_SLOT as i32 }
            Some(v) => { *v }
        }
    }

    /// Gets the position of the key in the dictionary or return -1 if there is no key at this
    /// position.
    pub fn get_position(&self, key: i32) -> i64 {
        match self.get(key) {
            None => { FREE_SLOT },
            Some(pos) => { pos as i64 },
        }
    }

    fn get(&self, key: i32) -> Option<i64> {
        let mut index = self.hash(key);
        while let Some(address) = self.hash_table.get(index as usize) {
            match self.keys.get(*address as usize) {
                None => { break; }
                Some(k) => {
                    if *k == key { // FIXME equals when generic
                        return Some(*address);
                    }
                    index = (index + 1) & (self.capacity - 1); // linear probing, find a new available slot
                }
            };
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use crate::dictionary::{Dictionary, FREE_SLOT};

    #[test]
    fn testWithInteger() {
        let mut dictionary = Dictionary::new(4);
        assert_eq!(dictionary.map(5), 0);
        assert_eq!(dictionary.map(0), 1);
        assert_eq!(dictionary.map(2), 2); // resize should happen
        assert_eq!(dictionary.map(1234), 3);
        assert_eq!(dictionary.map(5), 0); // key already exists
        assert_eq!(dictionary.map(3), 4);
        assert_eq!(dictionary.map(1), 5);

        assert_eq!(dictionary.get_position(5), 0);
        assert_eq!(dictionary.get_position(0), 1);
        assert_eq!(dictionary.get_position(2), 2);
        assert_eq!(dictionary.get_position(1234), 3);
        assert_eq!(dictionary.get_position(3), 4);
        assert_eq!(dictionary.get_position(1), 5);
        assert_eq!(dictionary.get_position(11111), FREE_SLOT); // does not exist
    }

    // TODO test with struct
}
