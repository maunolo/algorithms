#[derive(Default, Debug)]
pub struct MinHeap<T> {
    data: Vec<T>,
    pub length: usize,
}

impl<T> MinHeap<T>
where
    T: Default + Copy + PartialOrd,
{
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert(&mut self, value: T) {
        self.data.push(value);
        self.heapify_up(self.length);
        self.length += 1;
    }

    pub fn delete(&mut self) -> Option<T> {
        if self.length == 0 {
            return None;
        }

        let out = self.data[0];
        self.length -= 1;

        if self.length == 0 {
            self.data = vec![];
            return Some(out);
        }

        let last = self.data.pop().unwrap();
        self.data[0] = last;
        self.heapify_down(0);

        Some(out)
    }

    fn heapify_down(&mut self, idx: usize) {
        let left_idx = Self::left(idx);
        let right_idx = Self::right(idx);

        if idx >= self.length || left_idx >= self.length {
            return;
        }

        let left_value = self.data[left_idx];
        let right_value = self.data.get(right_idx);
        let value = self.data[idx];

        if let Some(right_value) = right_value {
            if left_value > *right_value && value > *right_value {
                self.data.swap(idx, right_idx);
                self.heapify_down(right_idx);
            } else if *right_value > left_value && value > left_value {
                self.data.swap(idx, left_idx);
                self.heapify_down(left_idx);
            }
        } else {
            if left_value < value {
                self.data.swap(idx, left_idx);
                self.heapify_down(left_idx);
            }
        }
    }

    fn heapify_up(&mut self, idx: usize) {
        if idx == 0 {
            return;
        }

        let parent = Self::parent(idx);

        if self.data[parent] > self.data[idx] {
            self.data.swap(parent, idx);
            self.heapify_up(parent);
        }
    }

    fn parent(idx: usize) -> usize {
        (idx - 1) / 2
    }

    fn left(idx: usize) -> usize {
        2 * idx + 1
    }

    fn right(idx: usize) -> usize {
        2 * idx + 2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_heap() {
        let mut heap = MinHeap::new();

        assert_eq!(heap.delete(), None);
        assert_eq!(heap.length, 0);

        heap.insert(5);
        heap.insert(3);
        heap.insert(69);
        heap.insert(420);
        heap.insert(4);
        heap.insert(1);
        heap.insert(8);
        heap.insert(7);

        assert_eq!(heap.length, 8);
        assert_eq!(heap.delete(), Some(1));
        assert_eq!(heap.delete(), Some(3));
        assert_eq!(heap.delete(), Some(4));
        assert_eq!(heap.delete(), Some(5));
        assert_eq!(heap.length, 4);
        assert_eq!(heap.delete(), Some(7));
        assert_eq!(heap.delete(), Some(8));
        assert_eq!(heap.delete(), Some(69));
        assert_eq!(heap.delete(), Some(420));
        assert_eq!(heap.delete(), None);
        assert_eq!(heap.length, 0);
    }
}
