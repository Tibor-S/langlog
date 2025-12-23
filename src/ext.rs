#![allow(dead_code)]

use std::{collections::BTreeMap, ops::Deref};

#[derive(Debug)]
pub struct Tree<K, V> {
    path: Vec<K>,
    val: Option<V>,
    connections: BTreeMap<K, Box<Tree<K, V>>>,
}
impl<K, V> Tree<K, V>
where
    K: Clone + Eq + Ord,
{
    fn with_path(path: Vec<K>) -> Self {
        Self {
            path,
            ..Default::default()
        }
    }

    pub fn insert<'a>(
        &'a mut self,
        key: &'a mut impl Iterator<Item = &'a K>,
        value: V,
    ) {
        let k = match key.next() {
            Some(k) => k,
            None => {
                self.val = Some(value);
                return;
            }
        };
        let mut prefix = self.path.clone();
        prefix.push(k.clone());
        if !self.connections.contains_key(k) {
            self.connections
                .insert(k.clone(), Box::new(Self::with_path(prefix)));
        }

        self.connections
            .get_mut(k)
            .map(|node| node.insert(key, value));
    }

    pub fn get<'a>(
        &'a self,
        key: &'a mut impl Iterator<Item = &'a K>,
    ) -> Option<&'a V> {
        let k = match key.next() {
            Some(k) => k,
            None => return self.val.as_ref(),
        };

        match self.connections.get(k) {
            Some(node) => node.get(key),
            None => None,
        }
    }

    pub fn get_tree<'a>(
        &'a self,
        key: &'a mut impl Iterator<Item = &'a K>,
    ) -> Option<&'a Self> {
        let k = match key.next() {
            Some(k) => k,
            None => return Some(self),
        };

        match self.connections.get(k) {
            Some(node) => node.get_tree(key),
            None => None,
        }
    }

    pub fn all(&self) -> Vec<&V> {
        let mut ret = vec![];
        if let Some(v) = self.val.as_ref() {
            ret.push(v);
        }
        for (_, tree) in self.connections.iter() {
            ret.extend(tree.all());
        }
        ret
    }

    pub fn all_path(&self) -> Vec<(Vec<K>, &V)> {
        let mut ret = vec![];
        if let Some(v) = self.val.as_ref() {
            ret.push((self.path.clone(), v));
        }
        for (_, tree) in self.connections.iter() {
            ret.extend(tree.all_path());
        }
        ret
    }
}
impl<V: Clone> Tree<char, V> {
    pub fn with_prefix(&self, token: &str) -> Vec<(String, V)> {
        let temp = token.chars().collect::<Vec<_>>();
        let key = &mut temp.iter();
        let sub = match self.get_tree(key) {
            Some(t) => t,
            None => return vec![],
        };

        sub.all_path()
            .iter()
            .map(|(cs, j)| (cs.iter().collect(), (*j).clone()))
            .collect()
    }

    pub fn get_str(&self, token: &str) -> Option<V> {
        let temp = token.chars().collect::<Vec<_>>();
        let key = &mut temp.iter();
        self.get(key).cloned()
    }

    pub fn insert_str(&mut self, token: &str, value: V) {
        let temp = token.chars().collect::<Vec<_>>();
        let key = &mut temp.iter();
        self.insert(key, value);
    }
}
impl<K, V> Default for Tree<K, V> {
    fn default() -> Self {
        Self {
            path: vec![],
            val: Default::default(),
            connections: Default::default(),
        }
    }
}

// Allows k-indexing (and usize-indexing via deref)
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct OrderedMap<K, V>(Vec<(K, V)>);
impl<K: Ord, V> OrderedMap<K, V> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<(K, V)>
    where
        K: Clone,
        V: Clone,
    {
        let index = match self.get_entry_mut_or_index(&key) {
            Ok((k, v)) => {
                let ret = (k.clone(), v.clone());
                *v = value;
                return Some(ret);
            }
            Err(i) => i,
        };

        self.0.insert(index, (key, value));
        None
    }

    pub fn remove(&mut self, key: &K) -> Option<(K, V)> {
        match self.key_index(key) {
            Some(i) => Some(self.0.remove(i)),
            None => None,
        }
    }

    pub fn key_index(&self, key: &K) -> Option<usize> {
        self.binary_search(key).ok()
    }

    pub fn get_entry(&self, key: &K) -> Option<&(K, V)> {
        self.get_entry_or_index(key).ok()
    }

    pub fn value(&mut self, key: &K) -> Option<&V> {
        self.get_entry(key).map(|(_, v)| v)
    }

    pub fn value_mut(&mut self, key: &K) -> Option<&mut V> {
        self.get_entry_mut_or_index(key).ok().map(|(_, v)| v)
    }

    fn binary_search(&self, key: &K) -> Result<usize, usize> {
        self.binary_search_by(|(k, _)| k.cmp(key))
    }

    fn get_entry_or_index(&self, key: &K) -> Result<&(K, V), usize> {
        match self.binary_search(key) {
            Ok(index) => self.0.get(index).ok_or(index),
            Err(index) => Err(index),
        }
    }

    fn get_entry_mut_or_index(
        &mut self,
        key: &K,
    ) -> Result<&mut (K, V), usize> {
        match self.binary_search(key) {
            Ok(index) => self.0.get_mut(index).ok_or(index),
            Err(index) => Err(index),
        }
    }
}
impl<K, V> Deref for OrderedMap<K, V> {
    type Target = Vec<(K, V)>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<K, V> Default for OrderedMap<K, V> {
    fn default() -> Self {
        Self(Default::default())
    }
}
