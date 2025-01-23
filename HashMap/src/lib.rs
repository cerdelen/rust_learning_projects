#![allow(non_snake_case)]
use std::{borrow::Borrow, hash::{Hash, Hasher}, mem, usize};
use std::collections::hash_map::DefaultHasher;

pub struct HashMap<K, V> {
    buckets: Vec<Vec<(K, V)>>,
    size: usize,
}

pub struct OccupiedEntry<'a, K: 'a, V: 'a> {
    entry: &'a mut(K, V),
}

pub struct VacantEntry<'a, K: 'a, V: 'a> {
    key: K,
    map: &'a mut HashMap<K, V>,
    bucket_ind: usize
}

pub enum Entry<'a, K: 'a, V: 'a> {
    Occupied(OccupiedEntry<'a, K, V>),
    Vacant(VacantEntry<'a, K, V>),
}

impl<'a, K: 'a, V: 'a> VacantEntry<'a, K, V> {
    pub fn insert(self, val: V) -> &'a mut V
    where
        K: Hash + Eq,
    {
        self.map.buckets[self.bucket_ind].push((self.key, val));
        self.map.size += 1;
        &mut self.map.buckets[self.bucket_ind].last_mut().unwrap().1
    }
}

impl<'a, K, V> Entry<'a, K, V>
where
    K: Hash + Eq,
{
    pub fn or_insert(self, val: V) -> &'a mut V {
        match self {
            Entry::Occupied(e) => &mut e.entry.1,
            Entry::Vacant(e) => e.insert(val),
        }
    }

    pub fn or_insert_with<F>(self, maker: F) -> &'a mut V
    where
        F: FnOnce() -> V,
    {
        match self {
            Entry::Occupied(e) => &mut e.entry.1,
            Entry::Vacant(e) => e.insert(maker()),
        }
    }

    pub fn or_default(self) -> &'a mut V
    where
        V: Default,
    {
        self.or_insert_with(Default::default)
    }
}


impl <K, V> HashMap<K, V> {
    pub fn new() -> Self {
        Self {
            buckets: Vec::new(),
            size: 0
        }
    }
}

impl <K, V> HashMap<K, V>
where K: Hash + Eq,
{
    fn find_bucket<Q>(&self, key: &Q) -> Option<usize>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized
    {
        if self.buckets.is_empty() {
            return None;
        }
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        Some((hasher.finish() % self.buckets.len() as u64) as usize)
    }

    pub fn entry<'a>(&'a mut self, key: K) -> Entry<'a, K, V> {
        if self.buckets.is_empty() || self.size > 3 * self.buckets.len() / 4 {
            self.resize();
        }

        let bucket_ind = self.find_bucket(&key).expect("Empty Bucket should be handeled before");
        match self.buckets[bucket_ind].iter().position(|&(ref ekey, _)| ekey == &key) {
            Some(index) => Entry::Occupied(OccupiedEntry{entry: &mut self.buckets[bucket_ind][index]}),
            None => Entry::Vacant(VacantEntry {map: self, key, bucket_ind}),
        }
    }

    fn resize(&mut self) {
        let new_size = match self.buckets.len() {
            0 => 1,
            n => 2 * n,
        };

        let mut new_buckets = Vec::with_capacity(new_size);
        new_buckets.extend((0..new_size).map(|_| Vec::new()));

        for (key, value) in self.buckets.iter_mut().flat_map(|bucket| bucket.drain(..)) {
            let mut hasher = DefaultHasher::new();
            key.hash(&mut hasher);
            let bucket_ind = (hasher.finish() % new_buckets.len() as u64) as usize;
            new_buckets[bucket_ind].push((key, value));
        }

        let _ = mem::replace(&mut self.buckets, new_buckets);
    }

    pub fn insert(&mut self, key: K, val: V) -> Option<V> {
        if self.buckets.is_empty() || self.size > 3 * self.buckets.len() / 4 {
            self.resize();
        }

        let bucket_ind = self.find_bucket(&key).expect("Empty Bucket should be handeled before");
        let bucket = &mut self.buckets[bucket_ind];

        for &mut (ref ekey, ref mut evalue) in bucket.iter_mut() {
            if ekey == &key {
                return Some(mem::replace(evalue, val))
            }
        }

        self.size += 1;
        bucket.push((key, val));
        None
    }

    pub fn get<Q>(&self, key: &Q) -> Option<&V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        let bucket_ind = self.find_bucket(&key)?;

        self.buckets[bucket_ind]
            .iter()
            .find(|&(ref ekey, _)| ekey.borrow() == key)
            .map(|&(_, ref v)| v)
    }

    pub fn contains_key<Q>(&self, key: &Q) -> bool
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        self.get(key).is_some()
        // let bucket_ind = self.find_bucket(&key);
        // if let Some(bucket_ind) = bucket_ind {
        //     if let Some(_) = self.buckets[bucket_ind]
        //         .iter()
        //         .find(|&(ref ekey, _)| ekey.borrow() == key) {
        //         return true;
        //     }
        // }
        // false
    }

    pub fn remove<Q>(&mut self, key: &Q) -> Option<V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        let bucket_ind = self.find_bucket(&key).expect("Empty Bucket should be handeled before");
        let bucket = &mut self.buckets[bucket_ind];

        let i = bucket
            .iter()
            .position(|&(ref ekey, _)| ekey.borrow() == key)?;


        self.size -= 1;
        Some(bucket.swap_remove(i).1)
    }

    pub fn len(&self) -> usize {
        self.size
    }

    pub fn empty(&self) -> bool {
        self.size == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn insert() {
        let mut map = HashMap::new();
        assert_eq!(map.len(), 0);
        map.insert(1, 2);

        let l = map.get(&1);
        println!("{:?}", l);
    }
}
