use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

static FREE_SLOT: i64 = -1;

#[derive(Debug)]
struct Dictionary {
    capacity: u64,
    size: u64,
    hashTable: Vec<i64>,
    keys: Vec<i32>, // FIXME make it generic
}

impl Dictionary {
    fn new(initialCapacity: u64) -> Dictionary {
        Dictionary {
            capacity: initialCapacity,
            size: 0,
            hashTable: vec![FREE_SLOT; initialCapacity as usize],
            keys: vec![-1; initialCapacity as usize],
        }
    }

    fn resize(&mut self, new_capacity: u64) {
        let mut tmp = Dictionary::new(new_capacity);

        // Remap
        for i in 0..self.size {
            let pos = i as usize;
            let x = self.keys[pos];
            tmp.map(x);
        }

        self.keys = tmp.keys;
        self.hashTable = tmp.hashTable;
        self.size = tmp.size;
        self.capacity = tmp.capacity;
    }

    fn hash(&self, key: i32) -> u64 {
        let mut s = DefaultHasher::default();
        key.hash(&mut s);
        s.finish() & (self.capacity - 1)
    }

    fn map(&mut self, key: i32) -> u64 {
        println!("=====================");
        // Double the table size if 50% full
        if self.size >= (self.capacity >> 1) {
            println!("resize !!!!!");
            self.resize(self.capacity << 1); // TODO make sure it does not overflow
            println!("resize done: {:?}", self);
        }

        let mut index = self.hash(key);
        loop {
            println!("computed hash: {}, size: {}, capacity: {}", index, self.size, self.capacity);
            let address = &self.hashTable[index as usize];
            println!("address: {}", address);
            if *address != FREE_SLOT {
                let k = match self.keys.get(*address as usize) {
                    None => { break; }
                    Some(v) => { v }
                };

                if *k == key { // FIXME equals when generic
                    return *address as u64;
                }
                index = (index + 1) & (self.capacity - 1); // linear probing, find a new available slot
                continue;
            }
            break;
        };

        let newAddress = self.size;
        println!("setting key: {}, at address: {}", key, newAddress);
        self.keys[newAddress as usize] = key;
        println!("size of keys: {}", self.keys.len());
        self.hashTable[index as usize] = newAddress as i64;
        self.size += 1;
        newAddress
    }

    fn read(&self, position: u64) -> i32 {
        match self.keys.get(position as usize) {
            None => { FREE_SLOT as i32 }
            Some(v) => { *v }
        }
    }

    fn get_position(&self, key: i32) {
        // TODO
    }
}

#[cfg(test)]
mod tests {
    use crate::dictionary::Dictionary;

    #[test]
    fn test() {
        let mut dictionary = Dictionary::new(4);
        assert_eq!(dictionary.map(5), 0);
        assert_eq!(dictionary.map(0), 1);
        assert_eq!(dictionary.map(2), 2); // resize should happen
        assert_eq!(dictionary.map(1234), 3);
        assert_eq!(dictionary.map(5), 0); // key already exists
        assert_eq!(dictionary.map(3), 4);
        assert_eq!(dictionary.map(1), 5);
    }
}
