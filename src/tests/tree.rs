use crate::binary_tree::{BinaryNode, BinaryTree};

pub fn tree() -> BinaryTree<isize> {
    let mut tree = BinaryTree::new();
    tree.set_head(BinaryNode::new(
        20,
        Some(BinaryNode::boxed(
            50,
            Some(BinaryNode::boxed(100, None, None)),
            Some(BinaryNode::boxed(
                30,
                Some(BinaryNode::boxed(45, None, None)),
                Some(BinaryNode::boxed(29, None, None)),
            )),
        )),
        Some(BinaryNode::boxed(
            10,
            Some(BinaryNode::boxed(15, None, None)),
            Some(BinaryNode::boxed(
                5,
                Some(BinaryNode::boxed(7, None, None)),
                None,
            )),
        )),
    ));
    tree
}

pub fn tree2() -> BinaryTree<isize> {
    let mut tree = BinaryTree::new();
    tree.set_head(BinaryNode::new(
        20,
        Some(BinaryNode::boxed(
            50,
            Some(BinaryNode::boxed(
                30,
                Some(BinaryNode::boxed(
                    45,
                    Some(BinaryNode::boxed(49, None, None)),
                    None,
                )),
                Some(BinaryNode::boxed(
                    29,
                    None,
                    Some(BinaryNode::boxed(21, None, None)),
                )),
            )),
            None,
        )),
        Some(BinaryNode::boxed(
            10,
            Some(BinaryNode::boxed(15, None, None)),
            Some(BinaryNode::boxed(
                5,
                Some(BinaryNode::boxed(7, None, None)),
                None,
            )),
        )),
    ));
    tree
}

pub fn sorted_tree() -> BinaryTree<isize> {
    let mut tree = BinaryTree::new();
    tree.set_head(BinaryNode::new(
        20,
        Some(BinaryNode::boxed(
            10,
            Some(BinaryNode::boxed(5, None, None)),
            Some(BinaryNode::boxed(15, None, None)),
        )),
        Some(BinaryNode::boxed(
            50,
            Some(BinaryNode::boxed(30, None, None)),
            Some(BinaryNode::boxed(100, None, None)),
        )),
    ));
    tree
}
