//!
//! an implementation for BST
//!
//!
//!

use std::fmt;

pub struct Node<T>
where
    T: Ord,
{
    val: T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}

impl<T: fmt::Display + Ord> fmt::Display for Node<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write_node(self, f)
    }
}

fn write_node<T: fmt::Display + Ord>(node: &Node<T>, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    if let Some(left) = &node.left {
        let _ = f.write_str(format!("{} --> {}[{}L]\n", node.val, left.val, left.val).as_str());
        let _ = write_node(left, f);
    }

    if let Some(right) = &node.right {
        let _ = f.write_str(format!("{} --> {}[{}R]\n", node.val, right.val, right.val).as_str());
        let _ = write_node(right, f);
    }

    Ok(())
}

/// a more rust-way implementation for BST
pub mod rs_bst {
    use std::cmp::Ordering;
    use std::fmt;

    pub enum RsNode<T: Ord>
    {
        Node {
            val: T,
            left: Box<RsNode<T>>,
            right: Box<RsNode<T>>,
        },
        Empty,
    }

    impl<T: Ord> RsNode<T> {
        pub fn new() -> RsNode<T> {
            RsNode::Empty
        }

        pub fn create(val: T) -> RsNode<T> {
            RsNode::Node {
                val,
                left: Box::new(RsNode::Empty),
                right: Box::new(RsNode::Empty),
            }
        }

        pub fn add(&mut self, new_value: T) {
            match self {
                RsNode::Node {
                    ref val,
                    ref mut left,
                    ref mut right,
                } => match new_value.cmp(val) {
                    Ordering::Less => left.add(new_value),
                    Ordering::Greater => right.add(new_value),
                    Ordering::Equal => return,
                },
                RsNode::Empty => {
                    *self = RsNode::create(new_value);
                }
            }
        }

        pub fn find(&self, find_value: T) -> bool {
            match self {
                RsNode::Node {
                    ref val,
                    ref left,
                    ref right,
                } => match find_value.cmp(val) {
                    Ordering::Less => left.find(find_value),
                    Ordering::Greater => right.find(find_value),
                    Ordering::Equal => true,
                },
                RsNode::Empty => false,
            }
        }
    }

    impl<T: fmt::Display + Ord> fmt::Display for RsNode<T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write_node(self, f)
        }
    }

    fn write_node<T: fmt::Display + Ord>(node: &RsNode<T>, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match node {
            RsNode::Node {
                ref val,
                ref left,
                ref right
            } => {
                // 访问 left 和 right 中的 val
                if let RsNode::Node { val: left_val, .. } = &**left {
                    let _ = f.write_str(format!("{} --> {}[{}L]\n", val, left_val, left_val).as_str());
                }
                let _ = write_node(left, f);

                if let RsNode::Node { val: right_val, .. } = &**right {
                    let _ = f.write_str(format!("{} --> {}[{}R]\n", val, right_val, right_val).as_str());
                }
                let _ = write_node(right, f);
            }
            RsNode::Empty => {}
        }

        Ok(())
    }
}

/// a traditional implementation for BST
pub mod tradition {
    use crate::bst::Node;

    impl<T> Node<T>
    where
        T: Ord,
    {
        pub fn new(root: T) -> Node<T> {
            Node {
                val: root,
                left: None,
                right: None,
            }
        }

        pub fn add(mut root: Node<T>, val: T) -> Node<T> {
            assert!(root.val != val);
            if root.val < val {
                root.right = Self::add_child(root.right, val)
            } else {
                root.left = Self::add_child(root.left, val);
            }
            root
        }

        fn add_child(child: Option<Box<Node<T>>>, val: T) -> Option<Box<Node<T>>> {
            match child {
                Some(node) => Some(Box::new(Self::add(*node, val))),
                None => Some(Box::new(Node::new(val))),
            }
        }
    }
}

/// another traditional implementation with add(*self) for BST
pub mod se1f {
    use crate::bst::Node;

    impl<T> Node<T>
    where
        T: Ord,
    {
        // `self` must be `mut` rather than `&mut` or `&` due to :
        //  1.1 the struct must be mutable because we are going to modify it;
        //  1.2 when we modify a node, we take the ownership because it maybe changes.
        pub fn add_self(mut self, val: T) -> Node<T> {
            assert!(self.val != val);
            if self.val < val {
                self.right = Self::add_self_child(self.right, val);
            } else {
                self.left = Self::add_self_child(self.left, val);
            }
            self
        }

        fn add_self_child(child: Option<Box<Node<T>>>, val: T) -> Option<Box<Node<T>>> {
            match child {
                Some(node) => Some(Box::new(node.add_self(val))),
                None => Some(Box::new(Node::new(val))),
            }
        }
    }
}
