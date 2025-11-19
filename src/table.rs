use crate::value::Value;

#[derive(Clone)]
enum Entry {
    Empty,
    Occupied { key: String, value: Value },
    Tombstone,
}

pub struct Table {
    entries: Vec<Entry>,
    count: usize,
}

impl Table {
    pub fn new() -> Self {
        Table {
            entries: Vec::new(),
            count: 0,
        }
    }

    pub fn set(&mut self, key: String, value: Value) -> bool {
        if self.count + 1 > self.entries.len() * 3 / 4 {
            let capacity = if self.entries.len() < 8 {
                8
            } else {
                self.entries.len() * 2
            };
            self.adjust_capacity(capacity);
        }

        let index = self.find_entry(&key);
        let is_new_key = matches!(self.entries[index], Entry::Empty);

        if is_new_key {
            self.count += 1;
        }

        self.entries[index] = Entry::Occupied { key, value };
        is_new_key
    }

    pub fn get(&self, key: &str) -> Option<&Value> {
        if self.entries.is_empty() {
            return None;
        }

        let index = self.find_entry(key);
        match &self.entries[index] {
            Entry::Occupied { value, .. } => Some(value),
            _ => None,
        }
    }

    pub fn delete(&mut self, key: &str) -> bool {
        if self.entries.is_empty() {
            return false;
        }

        let index = self.find_entry(key);
        match self.entries[index] {
            Entry::Occupied { .. } => {
                self.entries[index] = Entry::Tombstone;
                true
            }
            _ => false,
        }
    }

    pub fn find_string(&self, string: &str, hash: u32) -> Option<&str> {
        if self.entries.is_empty() {
            return None;
        }

        let mut index = (hash as usize) % self.entries.len();

        loop {
            match &self.entries[index] {
                Entry::Empty => {
                    return None;
                }
                Entry::Occupied { key, .. } => {
                    if key == string {
                        return Some(key.as_str());
                    }
                }
                Entry::Tombstone => {}
            }
            index = (index + 1) % self.entries.len();
        }
    }

    fn find_entry(&self, key: &str) -> usize {
        let mut index = (hash_string(key) as usize) % self.entries.len();
        let mut tombstone: Option<usize> = None;

        loop {
            match &self.entries[index] {
                Entry::Empty => {
                    return tombstone.unwrap_or(index);
                }
                Entry::Occupied { key: entry_key, .. } => {
                    if entry_key == key {
                        return index;
                    }
                }
                Entry::Tombstone => {
                    if tombstone.is_none() {
                        tombstone = Some(index);
                    }
                }
            }
            index = (index + 1) % self.entries.len();
        }
    }

    fn adjust_capacity(&mut self, capacity: usize) {
        let mut new_entries = vec![Entry::Empty; capacity];

        self.count = 0;
        for entry in self.entries.iter() {
            if let Entry::Occupied { key, value } = entry {
                let index = Self::find_entry_in(&new_entries, key);
                new_entries[index] = Entry::Occupied {
                    key: key.clone(),
                    value: value.clone(),
                };
                self.count += 1;
            }
        }

        self.entries = new_entries;
    }

    fn find_entry_in(entries: &[Entry], key: &str) -> usize {
        let mut index = (hash_string(key) as usize) % entries.len();

        loop {
            match &entries[index] {
                Entry::Empty | Entry::Tombstone => {
                    return index;
                }
                Entry::Occupied { key: entry_key, .. } => {
                    if entry_key == key {
                        return index;
                    }
                }
            }
            index = (index + 1) % entries.len();
        }
    }
}

pub fn hash_string(key: &str) -> u32 {
    let mut hash: u32 = 2166136261;
    for byte in key.bytes() {
        hash ^= byte as u32;
        hash = hash.wrapping_mul(16777619);
    }
    hash
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_table_set_and_get() {
        let mut table = Table::new();

        table.set("name".to_string(), Value::string("Alice".to_string()));
        table.set("age".to_string(), Value::number(30.0));

        assert_eq!(table.get("name").unwrap().as_string(), "Alice");
        assert_eq!(table.get("age").unwrap().as_number(), 30.0);
        assert!(table.get("unknown").is_none());
    }

    #[test]
    fn test_table_update() {
        let mut table = Table::new();

        let is_new = table.set("key".to_string(), Value::number(1.0));
        assert!(is_new);

        let is_new = table.set("key".to_string(), Value::number(2.0));
        assert!(!is_new);

        assert_eq!(table.get("key").unwrap().as_number(), 2.0);
    }

    #[test]
    fn test_table_delete() {
        let mut table = Table::new();

        table.set("key".to_string(), Value::number(42.0));
        assert!(table.get("key").is_some());

        assert!(table.delete("key"));
        assert!(table.get("key").is_none());

        assert!(!table.delete("key"));
    }

    #[test]
    fn test_table_many_entries() {
        let mut table = Table::new();

        for i in 0..100 {
            let key = format!("key{}", i);
            table.set(key.clone(), Value::number(i as f64));
        }

        for i in 0..100 {
            let key = format!("key{}", i);
            assert_eq!(table.get(&key).unwrap().as_number(), i as f64);
        }
    }
}
