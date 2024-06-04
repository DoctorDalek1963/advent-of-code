use std::{
    cell::RefCell,
    hash::Hash,
    rc::{Rc, Weak},
};

/// Small or large.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Size {
    Small,
    Large,
}

/// A node which holds some internal data and
#[derive(Clone, Debug)]
pub struct Node {
    /// The name of the node.
    pub(super) name: String,

    /// The size of the node.
    pub(super) size: Size,

    /// The nodes which are connected to this one.
    ///
    /// We're using [`Weak`] references here to avoid reference cycles. All nodes should be owned
    /// by a [`Graph`](../struct.Graph.html), so that all the nodes will be dropped when the graph
    /// is dropped.
    pub(super) connected: RefCell<Vec<Weak<Node>>>,
}

impl Hash for Node {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.name.hash(state);
        self.size.hash(state);
        //self.connected.hash(state);
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.size == other.size
    }
}

impl Eq for Node {}

impl Node {
    /// Create a new node with no neighbours.
    pub fn new(name: String, size: Size) -> Rc<Self> {
        let node = Self {
            name,
            size,
            connected: RefCell::new(Vec::new()),
        };
        Rc::new(node)
    }

    pub fn parse(name: String) -> Rc<Self> {
        let size = if name == name.to_lowercase() {
            Size::Small
        } else if name == name.to_uppercase() {
            Size::Large
        } else {
            unreachable!("Names must be all lowercase or all uppercase")
        };

        Self::new(name, size)
    }

    /// Connect two nodes together.
    pub fn add_connection(node_a: Rc<Self>, node_b: Rc<Self>) {
        node_a.connected.borrow_mut().push(Rc::downgrade(&node_b));
        node_b.connected.borrow_mut().push(Rc::downgrade(&node_a));
    }

    pub fn rc_connections(&self) -> Vec<Rc<Self>> {
        self.connected
            .borrow()
            .iter()
            .filter_map(|node| node.upgrade())
            .collect()
    }
}
