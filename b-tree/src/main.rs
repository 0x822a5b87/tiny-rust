use crate::bst::rs_bst;

mod bst;

fn main() {
    add();

    add_self();

    add_rs();
}

pub fn add() {
    use bst::Node;

    let mut root = Node::new(-1);
    root = Node::add(root, 0);
    root = Node::add(root, 1);
    root = Node::add(root, 9);
    root = Node::add(root, 7);
    root = Node::add(root, 10);
    root = Node::add(root, 2);
    root = Node::add(root, 6);
    root = Node::add(root, 4);
    root = Node::add(root, 3);

    println!("{}", root)
}

pub fn add_self() {
    use bst::Node;
    let mut root = Node::new(-1);
    root = root.add_self(0);
    root = root.add_self(1);
    root = root.add_self(9);
    root = root.add_self(7);
    root = root.add_self(10);
    root = root.add_self(2);
    root = root.add_self(6);
    root = root.add_self(4);
    root = root.add_self(3);

    println!("{}", root)
}

pub fn add_rs() {
    use rs_bst::RsNode;
    let mut root = RsNode::new();
    root.add(-1);
    root.add(0);
    root.add(1);
    root.add(9);
    root.add(7);
    root.add(10);
    root.add(2);
    root.add(6);
    root.add(4);
    root.add(3);
    println!("{}", root);

    let x = root.find(4);
    println!("find {}", x)
}
