use std::ptr::NonNull;

type Link<T> = Option<NonNull<Node<T>>>; // alias

struct Node<T> {
    value: T,
    next: Link<T>,
}

pub struct Queue<T> {
    lenght: usize,
    head: Link<T>,
    tail: Link<T>,
}

impl<T> Queue<T> {
    pub fn new() -> Queue<T> {
        Queue {
            lenght: 0,
            head: None,
            tail: None,
        }
    }

    pub fn enqueue(&mut self, value: T) {
        unsafe {
            let node = NonNull::new_unchecked(Box::into_raw(Box::new(Node { value, next: None })));
            if let Some(tail) = self.tail {
                (*tail.as_ptr()).next = Some(node);
            } else {
                self.head = Some(node);
            }
            self.tail = Some(node);
            self.lenght += 1;
        }
    }

    pub fn deque(&mut self) -> Option<T> {
        self.head.map(|node| unsafe {
            let boxed_node = Box::from_raw(node.as_ptr());
            let result = boxed_node.value;

            self.head = boxed_node.next;
            if self.head.is_none() {
                self.tail = None;
            }
            self.lenght -= 1;

            result
        })
    }

    pub fn peek(&self) -> Option<&T> {
        unsafe { self.head.map(|node| &(*node.as_ptr()).value) }
    }

    pub fn peek_mut(&self) -> Option<&mut T> {
        unsafe { self.head.map(|node| &mut (*node.as_ptr()).value) }
    }

    pub fn lenght(&self) -> usize {
        self.lenght
    }
}

impl<T> Drop for Queue<T> {
    fn drop(&mut self) {
        while let Some(_) = self.deque() {}
    }
}

#[cfg(test)]
mod test {
    use super::Queue;

    #[test]
    fn test_queue() {
        let mut queue = Queue::new();
        assert_eq!(queue.deque(), None);
        queue.enqueue(1);
        assert_eq!(queue.lenght(), 1);
        queue.enqueue(2);
        assert_eq!(queue.lenght(), 2);
        assert_eq!(queue.peek(), Some(&1));
        assert_eq!(queue.peek_mut(), Some(&mut 1));
        assert_eq!(queue.lenght(), 2);
        assert_eq!(queue.deque(), Some(1));
        assert_eq!(queue.lenght(), 1);
        assert_eq!(queue.deque(), Some(2));
        assert_eq!(queue.lenght(), 0);
        assert_eq!(queue.deque(), None);
        assert_eq!(queue.lenght(), 0);
    }
}
