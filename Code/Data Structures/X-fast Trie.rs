use std::collections::BTreeSet;

#[derive(Debug, Clone)]
struct XFastTrie {
    root: Node,
    max_bits: usize,
}


#[derive(Debug, Clone)]
enum Node {
    Leaf(BTreeSet<usize>),
    Internal(Box<[Option<Box<Node>>; 2]>),
}
