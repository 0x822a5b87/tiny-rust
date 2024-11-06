//!
//! an implementation for Red-Black Tree
//!
//! https://en.wikipedia.org/wiki/Red%E2%80%93black_tree
//!
//! In addition to the requirements imposed on a binary search tree the following must be satisfied by a red–black tree:
//! 1. Every node is either red or black.
//! 2. All NIL nodes (figure 1) are considered black.
//! 3. A red node does not have a red child.
//! 4. Every path from a given node to any of its descendant NIL nodes goes through the same number of **black nodes**.
//! 5. (Conclusion) If a node N has exactly one child, the child must be red(and the node N itself must be black, because <3>), because if it were black, its NIL descendants
//! would sit at a different black depth than N's NIL child, violating requirement 4.
//!
// enum Rbt<T: Ord> {
//     Node {
//         is_red: bool,
//         left: Box<Rbt<T>>,
//         right: Box<Rbt<T>>,
//     },
//     Nil,
// }
//
// impl<T: Ord> Rbt<T> {
//     pub fn is_red(&self) -> bool {
//         match self {
//             Rbt::Node {
//                 is_red,
//                 ..
//             } => *is_red,
//             Rbt::Nil => false
//         }
//     }
// }
