struct Node<T> {
    value: T,
    next: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>; // alias

pub struct Stack<T> {
    head: Link<T>,
    lenght: usize,
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Self {
            head: None,
            lenght: 0,
        }
    }

    pub fn push(&mut self, value: T) {
        let new_node = Some(Box::new(Node {
            value,
            next: self.head.take(),
        }));
        self.head = new_node;
        self.lenght += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            self.lenght -= 1;
            node.value
        })
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.value)
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| &mut node.value)
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_stack() {
        let mut stack = super::Stack::new();
        assert_eq!(stack.pop(), None);
        assert_eq!(stack.peek(), None);
        assert_eq!(stack.peek_mut(), None);
        stack.push(1);
        assert_eq!(stack.peek(), Some(&1));
        assert_eq!(stack.peek_mut(), Some(&mut 1));
        stack.push(2);
        assert_eq!(stack.peek(), Some(&2));
        assert_eq!(stack.peek_mut(), Some(&mut 2));
        stack.push(3);
        assert_eq!(stack.peek(), Some(&3));
        assert_eq!(stack.peek_mut(), Some(&mut 3));
        stack.push(4);
        assert_eq!(stack.peek(), Some(&4));
        assert_eq!(stack.peek_mut(), Some(&mut 4));
        stack.push(5);
        assert_eq!(stack.peek(), Some(&5));
        assert_eq!(stack.peek_mut(), Some(&mut 5));
        stack.peek_mut().map(|value| *value = 42);
        assert_eq!(stack.pop(), Some(42));
        assert_eq!(stack.pop(), Some(4));
        assert_eq!(stack.pop(), Some(3));
        assert_eq!(stack.pop(), Some(2));
        assert_eq!(stack.pop(), Some(1));
        assert_eq!(stack.pop(), None);
        assert_eq!(stack.peek(), None);
        assert_eq!(stack.peek_mut(), None);
    }
}
