pub mod bin;
mod dfs;
mod node;
mod parse;

use dfs::DfsVisitor;
use itertools::Itertools;
use node::Size;

use self::node::Node;
use std::rc::Rc;

/// A collection of nodes in a graph.
#[derive(Clone, Debug)]
pub struct Graph {
    /// The nodes of the graph.
    nodes: Vec<Rc<Node>>,
}

impl Graph {
    pub fn parse(input: &str) -> Self {
        let mut graph = Self { nodes: vec![] };

        let pairs = parse::parse_pairs(input)
            .expect("The input should parse correctly")
            .1;

        for (a, b) in pairs {
            let a = graph.add_node(a);
            let b = graph.add_node(b);
            Node::add_connection(a, b);
        }

        graph
    }

    /// Find the node with the given name and return an `Rc` to it, or create a new node if it
    /// doesn't already exist.
    fn add_node(&mut self, name: &str) -> Rc<Node> {
        match self.nodes.iter().find(|&node| node.name == name) {
            Some(x) => Rc::clone(x),
            None => {
                let node = Node::parse(name.to_string());
                self.nodes.push(Rc::clone(&node));
                node
            }
        }
    }

    pub fn all_dfs_paths_visiting_small_caves_once(&self) -> Vec<Vec<Rc<Node>>> {
        let start = Rc::clone(
            self.nodes
                .iter()
                .find(|&node| node.name == "start")
                .expect("`start` node must exist"),
        );

        let mut vis = DfsVisitor::new(start);
        vis.find_all_paths_visiting_small_caves_once();
        vis.all_paths
    }

    pub fn all_dfs_paths_visiting_small_caves_twice(&self) -> Vec<Vec<Rc<Node>>> {
        let start = Rc::clone(
            self.nodes
                .iter()
                .find(|&node| node.name == "start")
                .expect("`start` node must exist"),
        );

        let mut vis = DfsVisitor::new(start);
        vis.find_all_paths_visiting_small_caves_twice();
        vis.all_paths
            .into_iter()
            .filter(|path| {
                let smalls = path
                    .iter()
                    .filter(|&node| node.size == Size::Small)
                    .counts();
                smalls
                    .values()
                    .into_iter()
                    .filter(|&&count| count >= 2)
                    .count()
                    <= 1
            })
            .collect()
    }
}

#[cfg(test)]
pub const TEST_INPUT_1: &str = "start-A
start-b
A-c
A-b
b-d
A-end
b-end";

#[cfg(test)]
pub const TEST_INPUT_2: &str = "dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc";

#[cfg(test)]
pub const TEST_INPUT_3: &str = "fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW";
