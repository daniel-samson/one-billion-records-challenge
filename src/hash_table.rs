use std::hash::Hash;
use crate::xxhash::XxHash64;

const INITIAL_CAPACITY: usize = 16;
const LOAD_FACTOR_THRESHOLD: f64 = 0.75;

pub struct HashTable<K, V> {
    buckets: Vec<Vec<(K, V)>>,
    size: usize,
    capacity: usize,
}

impl<K, V> HashTable<K, V>
where
    K: Hash + Eq + Clone,
    V: Clone,
{
    pub fn new() -> Self {
        Self::with_capacity(INITIAL_CAPACITY)
    }

    pub fn with_capacity(capacity: usize) -> Self {
        let mut buckets = Vec::with_capacity(capacity);
        for _ in 0..capacity {
            buckets.push(Vec::new());
        }
        
        Self {
            buckets,
            size: 0,
            capacity,
        }
    }

    fn hash(&self, key: &K) -> usize {
        let mut hasher = XxHash64::new(0);
        key.hash(&mut hasher);
        (hasher.finish() as usize) % self.capacity
    }

    fn load_factor(&self) -> f64 {
        self.size as f64 / self.capacity as f64
    }

    fn resize(&mut self) {
        let old_buckets = std::mem::take(&mut self.buckets);
        self.capacity *= 2;
        self.size = 0;
        
        self.buckets = Vec::with_capacity(self.capacity);
        for _ in 0..self.capacity {
            self.buckets.push(Vec::new());
        }

        for bucket in old_buckets {
            for (key, value) in bucket {
                self.insert(key, value);
            }
        }
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        if self.load_factor() > LOAD_FACTOR_THRESHOLD {
            self.resize();
        }

        let index = self.hash(&key);
        let bucket = &mut self.buckets[index];

        for (existing_key, existing_value) in bucket.iter_mut() {
            if existing_key == &key {
                return Some(std::mem::replace(existing_value, value));
            }
        }

        bucket.push((key, value));
        self.size += 1;
        None
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        let index = self.hash(key);
        let bucket = &self.buckets[index];

        for (existing_key, existing_value) in bucket {
            if existing_key == key {
                return Some(existing_value);
            }
        }

        None
    }

    pub fn remove(&mut self, key: &K) -> Option<V> {
        let index = self.hash(key);
        let bucket = &mut self.buckets[index];

        for (i, (existing_key, _)) in bucket.iter().enumerate() {
            if existing_key == key {
                let (_, value) = bucket.remove(i);
                self.size -= 1;
                return Some(value);
            }
        }

        None
    }

    pub fn contains_key(&self, key: &K) -> bool {
        self.get(key).is_some()
    }

    pub fn len(&self) -> usize {
        self.size
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    pub fn keys(&self) -> impl Iterator<Item = &K> {
        self.buckets.iter().flat_map(|bucket| bucket.iter().map(|(k, _)| k))
    }

    pub fn values(&self) -> impl Iterator<Item = &V> {
        self.buckets.iter().flat_map(|bucket| bucket.iter().map(|(_, v)| v))
    }

    pub fn iter(&self) -> impl Iterator<Item = (&K, &V)> {
        self.buckets.iter().flat_map(|bucket| bucket.iter().map(|(k, v)| (k, v)))
    }
}

impl<K, V> Default for HashTable<K, V>
where
    K: Hash + Eq + Clone,
    V: Clone,
{
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_hash_table() {
        let table: HashTable<String, i32> = HashTable::new();
        assert_eq!(table.len(), 0);
        assert!(table.is_empty());
    }

    #[test]
    fn test_insert_and_get() {
        let mut table = HashTable::new();
        table.insert("key1".to_string(), 42);
        table.insert("key2".to_string(), 84);

        assert_eq!(table.get(&"key1".to_string()), Some(&42));
        assert_eq!(table.get(&"key2".to_string()), Some(&84));
        assert_eq!(table.get(&"key3".to_string()), None);
        assert_eq!(table.len(), 2);
    }

    #[test]
    fn test_insert_duplicate_key() {
        let mut table = HashTable::new();
        assert_eq!(table.insert("key".to_string(), 42), None);
        assert_eq!(table.insert("key".to_string(), 84), Some(42));
        assert_eq!(table.get(&"key".to_string()), Some(&84));
        assert_eq!(table.len(), 1);
    }

    #[test]
    fn test_remove() {
        let mut table = HashTable::new();
        table.insert("key1".to_string(), 42);
        table.insert("key2".to_string(), 84);

        assert_eq!(table.remove(&"key1".to_string()), Some(42));
        assert_eq!(table.get(&"key1".to_string()), None);
        assert_eq!(table.remove(&"key1".to_string()), None);
        assert_eq!(table.len(), 1);
    }

    #[test]
    fn test_contains_key() {
        let mut table = HashTable::new();
        table.insert("key".to_string(), 42);

        assert!(table.contains_key(&"key".to_string()));
        assert!(!table.contains_key(&"nonexistent".to_string()));
    }

    #[test]
    fn test_resize() {
        let mut table = HashTable::with_capacity(2);
        
        for i in 0..10 {
            table.insert(i, i * 10);
        }

        for i in 0..10 {
            assert_eq!(table.get(&i), Some(&(i * 10)));
        }
        
        assert_eq!(table.len(), 10);
    }

    #[test]
    fn test_iterators() {
        let mut table = HashTable::new();
        table.insert("a".to_string(), 1);
        table.insert("b".to_string(), 2);
        table.insert("c".to_string(), 3);

        let keys: Vec<_> = table.keys().collect();
        let values: Vec<_> = table.values().collect();
        
        assert_eq!(keys.len(), 3);
        assert_eq!(values.len(), 3);
        
        let pairs: Vec<_> = table.iter().collect();
        assert_eq!(pairs.len(), 3);
    }
}