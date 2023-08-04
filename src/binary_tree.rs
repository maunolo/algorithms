use crate::queue::Queue;

type Link<T> = Option<Box<BinaryNode<T>>>; // alias

#[derive(Clone, Debug)]
pub struct BinaryTree<T> {
    head: Link<T>,
}

#[derive(Clone, Debug)]
pub struct BinaryNode<T> {
    value: T,
    left: Link<T>,
    right: Link<T>,
}

impl<T> BinaryTree<T> {
    pub fn new() -> Self {
        Self { head: None }
    }

    pub fn set_head(&mut self, node: BinaryNode<T>) {
        self.head = Some(Box::new(node));
    }
}

impl<T> PartialEq for BinaryTree<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        compare(&self.head, &other.head)
    }
}

pub fn compare<T: PartialEq>(
    a: &Option<Box<BinaryNode<T>>>,
    b: &Option<Box<BinaryNode<T>>>,
) -> bool {
    if a.is_none() && b.is_none() {
        return true;
    }

    if a.is_none() || b.is_none() {
        return false;
    }

    let a = a.as_ref().unwrap();
    let b = b.as_ref().unwrap();

    if a.value != b.value {
        return false;
    }

    compare(&a.left, &b.left) && compare(&a.right, &b.right)
}

impl<T> BinaryNode<T> {
    pub fn new(
        value: T,
        left: Option<Box<BinaryNode<T>>>,
        right: Option<Box<BinaryNode<T>>>,
    ) -> Self {
        Self { value, left, right }
    }

    pub fn boxed(
        value: T,
        left: Option<Box<BinaryNode<T>>>,
        right: Option<Box<BinaryNode<T>>>,
    ) -> Box<Self> {
        Box::new(Self { value, left, right })
    }
}

fn pre_order_walk(curr: Option<Box<BinaryNode<isize>>>, path: &mut Vec<isize>) {
    if let Some(node) = curr {
        path.push(node.value);
        pre_order_walk(node.left, path);
        pre_order_walk(node.right, path);
    }
}

pub fn pre_order_search(tree: BinaryTree<isize>) -> Vec<isize> {
    let mut path = Vec::new();
    pre_order_walk(tree.head, &mut path);
    path
}

fn in_order_walk(curr: Option<Box<BinaryNode<isize>>>, path: &mut Vec<isize>) {
    if let Some(node) = curr {
        in_order_walk(node.left, path);
        path.push(node.value);
        in_order_walk(node.right, path);
    }
}

pub fn in_order_search(tree: BinaryTree<isize>) -> Vec<isize> {
    let mut path = Vec::new();
    in_order_walk(tree.head, &mut path);
    path
}

fn post_order_walk(curr: Option<Box<BinaryNode<isize>>>, path: &mut Vec<isize>) {
    if let Some(node) = curr {
        post_order_walk(node.left, path);
        post_order_walk(node.right, path);
        path.push(node.value);
    }
}

pub fn post_order_search(tree: BinaryTree<isize>) -> Vec<isize> {
    let mut path = Vec::new();
    post_order_walk(tree.head, &mut path);
    path
}

pub fn breadth_first_search(tree: BinaryTree<isize>, needle: isize) -> bool {
    if let Some(head) = tree.head {
        let mut queue = Queue::new();
        queue.enqueue(head);

        while queue.lenght() > 0 {
            let curr = queue.deque().unwrap();

            if curr.value == needle {
                return true;
            }

            if let Some(left) = curr.left {
                queue.enqueue(left);
            }
            if let Some(right) = curr.right {
                queue.enqueue(right);
            }
        }
    }

    false
}

pub fn dfs_recursion(curr: Option<Box<BinaryNode<isize>>>, needle: isize) -> bool {
    println!("curr: {:?}", curr);

    if let Some(node) = curr {
        if node.value == needle {
            return true;
        }

        if node.value < needle {
            return dfs_recursion(node.right, needle);
        }

        return dfs_recursion(node.left, needle);
    }

    false
}

pub fn depth_first_search(tree: BinaryTree<isize>, needle: isize) -> bool {
    dfs_recursion(tree.head, needle)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_binary_tree() {
        let tree = crate::tests::tree::tree();
        assert_eq!(
            pre_order_search(tree.clone()),
            vec![20, 50, 100, 30, 45, 29, 10, 15, 5, 7]
        );
        assert_eq!(
            in_order_search(tree.clone()),
            vec![100, 50, 45, 30, 29, 20, 15, 10, 7, 5]
        );
        assert_eq!(
            post_order_search(tree),
            vec![100, 45, 29, 30, 50, 15, 7, 5, 10, 20]
        );
    }

    #[test]
    fn test_bfs() {
        let tree = crate::tests::tree::tree();
        assert_eq!(breadth_first_search(tree.clone(), 45), true);
        assert_eq!(breadth_first_search(tree.clone(), 7), true);
        assert_eq!(breadth_first_search(tree.clone(), 69), false);
    }

    #[test]
    fn test_compare() {
        let tree = crate::tests::tree::tree();
        let tree2 = crate::tests::tree::tree();
        assert_eq!(tree, tree2);

        let tree3 = crate::tests::tree::tree2();
        assert_ne!(tree, tree3);
    }

    #[test]
    fn test_dfs() {
        let tree = crate::tests::tree::sorted_tree();
        assert_eq!(depth_first_search(tree.clone(), 15), true);
        assert_eq!(depth_first_search(tree.clone(), 7), false);
        assert_eq!(depth_first_search(tree.clone(), 69), false);
    }
}
