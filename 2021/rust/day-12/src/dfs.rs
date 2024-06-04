use itertools::Itertools;

use crate::node::{Node, Size};
use std::{collections::HashSet, rc::Rc};

type Path = Vec<Rc<Node>>;

#[allow(unstable_name_collisions)]
fn path_to_string(path: &Path) -> String {
    path.iter()
        .map(|node| node.name.clone())
        .intersperse(String::from(","))
        .collect()
}

pub(super) struct DfsVisitor {
    current_path: Path,
    pub all_paths: Vec<Path>,
    all_paths_strings: HashSet<String>,
}

impl DfsVisitor {
    pub fn new(start: Rc<Node>) -> Self {
        Self {
            current_path: vec![start],
            all_paths: vec![],
            all_paths_strings: HashSet::new(),
        }
    }

    fn all_paths_contains(&self, path: &Path) -> bool {
        let s = path_to_string(path);
        self.all_paths_strings.contains(&s)
    }

    fn all_paths_push(&mut self, path: Path) {
        self.all_paths_strings.insert(path_to_string(&path));
        self.all_paths.push(path);
    }

    pub fn find_all_paths_visiting_small_caves_once(&mut self) {
        let current = self.current_path.last().unwrap();

        if current.name == "end" {
            assert!(!self.all_paths_contains(&self.current_path));
            self.all_paths_push(self.current_path.clone());
        } else {
            let connections: Vec<Rc<Node>> = current
                .rc_connections()
                .into_iter()
                .filter(|node| {
                    node.name != "start"
                        && !(node.size == Size::Small && self.current_path.contains(&node))
                })
                .collect();

            for node in connections {
                self.current_path.push(node);
                if self.all_paths_contains(&self.current_path) {
                    continue;
                }
                self.find_all_paths_visiting_small_caves_once();
                self.current_path.pop();
            }
        }
    }

    pub fn find_all_paths_visiting_small_caves_twice(&mut self) {
        let current = self.current_path.last().unwrap();

        if current.name == "end" {
            assert!(!self.all_paths_contains(&self.current_path));
            self.all_paths_push(self.current_path.clone());
        } else {
            // Have we already visited a small cave twice? If so, which one?
            let visited_small_cave_twice: Option<Rc<Node>> = self
                .current_path
                .iter()
                .filter(|&node| node.size == Size::Small)
                .counts()
                .into_iter()
                .filter_map(|(node, count)| (count >= 2).then_some(node))
                .collect_vec()
                .first()
                .map(|&node| Rc::clone(node));

            let connections: Vec<Rc<Node>> = current
                .rc_connections()
                .into_iter()
                .filter(|node| {
                    node.name != "start"
                        && !(node.size == Size::Small
                            && self
                                .current_path
                                .iter()
                                .filter(|&path_node| path_node == node)
                                .count()
                                >= 2)
                })
                .filter(|node| {
                    if let Some(visited_node) = &visited_small_cave_twice {
                        node != visited_node
                    } else {
                        true
                    }
                })
                .collect();
            //dbg!(&self.current_path, &connections);

            for node in connections {
                self.current_path.push(node);
                if self.all_paths_contains(&self.current_path) {
                    continue;
                }
                self.find_all_paths_visiting_small_caves_twice();
                self.current_path.pop();
            }
        }
    }
}
