use std::{collections::HashMap, hash::Hash};

#[derive(Debug, Clone)]
pub struct HashTree<K, T> {
    pub data: T,
    childs: HashMap<K, HashTree<K, T>>,
}

impl<K, T> HashTree<K, T>
where
    K: Eq + Hash + Clone,
{
    pub fn new(data: T) -> Self {
        HashTree {
            data,
            childs: HashMap::new(),
        }
    }

    pub fn get_child(&self, k: &K) -> Option<&HashTree<K, T>> {
        self.childs.get(k)
    }

    pub fn add_child(&mut self, k: K, child: T) -> &mut HashTree<K, T> {
        if !self.childs.contains_key(&k) {
            self.childs.insert(
                k.clone(),
                HashTree {
                    data: child,
                    childs: HashMap::new(),
                },
            );
        }

        self.childs.get_mut(&k).unwrap()
    }
}
