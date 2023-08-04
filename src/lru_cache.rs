use std::{collections::HashMap, ptr::NonNull};

type Link<T> = NonNull<Node<T>>;

struct Node<T> {
    value: T,
    next: Option<Link<T>>,
    prev: Option<Link<T>>,
}

pub struct LRUCache<K, V>
where
    K: std::hash::Hash + Eq + Clone + std::fmt::Debug,
    V: std::hash::Hash + Eq + Clone + std::fmt::Debug,
{
    length: usize,
    head: Option<Link<V>>,
    tail: Option<Link<V>>,
    lookup: HashMap<K, Link<V>>,
    reverse_lookup: HashMap<Link<V>, K>,
    capacity: usize,
}

fn create_node<V>(value: V) -> NonNull<Node<V>> {
    unsafe {
        NonNull::new_unchecked(Box::into_raw(Box::new(Node {
            value,
            next: None,
            prev: None,
        })))
    }
}

impl<K, V> LRUCache<K, V>
where
    K: std::hash::Hash + Eq + Clone + std::fmt::Debug,
    V: std::hash::Hash + Eq + Clone + std::fmt::Debug,
{
    pub fn new(capacity: Option<usize>) -> Self {
        Self {
            length: 0,
            head: None,
            tail: None,
            lookup: HashMap::new(),
            reverse_lookup: HashMap::new(),
            capacity: capacity.unwrap_or(10),
        }
    }

    pub fn update(&mut self, key: &K, value: V) {
        let node = self.lookup.remove(key);
        // does it exist?
        match node {
            // if it does, we need to update to the front of the list
            Some(node) => {
                self.detach(node);
                unsafe { (*node.as_ptr()).value = value.clone() };
                self.prepend(node);
                self.lookup.insert(key.clone(), node);
            }
            // if it doesn't we need to insert
            None => {
                let node = create_node(value);
                self.prepend(node);
                // check capacity and evict if over
                self.trim();
                self.lookup.insert(key.clone(), node);
                self.reverse_lookup.insert(node, key.clone());
            }
        };
    }

    pub fn get(&mut self, key: &K) -> Option<V> {
        // check the cache for existence
        let node = self.lookup.remove(key);
        match node {
            // update the value we found and move it to the front
            Some(node) => {
                self.detach(node);
                self.prepend(node);
                self.lookup.insert(key.clone(), node);

                Some(unsafe { (*node.as_ptr()).value.clone() })
            }
            // return out the value found or none if it doesn't exist
            None => None,
        }
    }

    pub fn clear(&mut self) {
        while let Some(node) = self.head {
            self.detach(node);
            drop(unsafe { Box::from_raw(node.as_ptr()) });
        }

        self.lookup.clear();
        self.reverse_lookup.clear();
    }

    fn detach(&mut self, mut node: Link<V>) {
        let node = unsafe { node.as_mut() };

        if let Some(prev) = node.prev {
            unsafe {
                (*prev.as_ptr()).next = node.next;
            }
        } else {
            self.head = node.next;
        }

        if let Some(next) = node.next {
            unsafe {
                (*next.as_ptr()).prev = node.prev;
            }
        } else {
            self.tail = node.prev;
        }

        self.length -= 1;
        node.prev = None;
        node.next = None;
    }

    fn prepend(&mut self, node: Link<V>) {
        if self.head.is_none() {
            self.head = Some(node);
            self.tail = Some(node);
        } else {
            let head = self.head.unwrap();
            unsafe {
                (*head.as_ptr()).prev = Some(node);
                (*node.as_ptr()).next = Some(head);
            }
            self.head = Some(node);
        }

        self.length += 1;
    }

    fn trim(&mut self) {
        if self.length > self.capacity {
            let tail = self.tail.unwrap();
            let key = self.reverse_lookup.remove(&tail).unwrap();
            self.lookup.remove(&key);
            self.detach(tail);
            drop(unsafe { Box::from_raw(tail.as_ptr()) });
        }
    }
}

impl<K, V> Drop for LRUCache<K, V>
where
    K: std::hash::Hash + Eq + Clone + std::fmt::Debug,
    V: std::hash::Hash + Eq + Clone + std::fmt::Debug,
{
    fn drop(&mut self) {
        self.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lru_cache() {
        let mut lru = LRUCache::<String, usize>::new(Some(3));
        assert_eq!(lru.get(&"foo".to_string()), None);
        lru.update(&"foo".to_string(), 69);
        assert_eq!(lru.get(&"foo".to_string()), Some(69));

        lru.update(&"foo".to_string(), 70);
        assert_eq!(lru.get(&"foo".to_string()), Some(70));

        lru.update(&"bar".to_string(), 420);
        assert_eq!(lru.get(&"bar".to_string()), Some(420));

        lru.update(&"baz".to_string(), 1337);
        assert_eq!(lru.get(&"baz".to_string()), Some(1337));

        lru.update(&"ball".to_string(), 69420);
        assert_eq!(lru.get(&"ball".to_string()), Some(69420));
        assert_eq!(lru.get(&"foo".to_string()), None);
        assert_eq!(lru.get(&"bar".to_string()), Some(420));
        lru.update(&"foo".to_string(), 69);

        // shouldn't of been deleted, but since bar was get'd, bar was added to the
        // front of the list, so baz became the end
        assert_eq!(lru.get(&"baz".to_string()), None);
    }
}
