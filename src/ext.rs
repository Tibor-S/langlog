use std::collections::BTreeMap;

#[derive(Debug)]
pub struct Tree<K, V> {
    val: Option<V>,
    connections: BTreeMap<K, Box<Tree<K, V>>>,
}
impl<K, V> Tree<K, V>
where
    K: Clone + Eq + Ord,
{
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

        if !self.connections.contains_key(k) {
            self.connections
                .insert(k.clone(), Box::new(Self::default()));
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
}
impl<K, V> Default for Tree<K, V> {
    fn default() -> Self {
        Self {
            val: Default::default(),
            connections: Default::default(),
        }
    }
}
