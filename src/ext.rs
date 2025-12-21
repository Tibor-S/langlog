use std::collections::BTreeMap;

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
