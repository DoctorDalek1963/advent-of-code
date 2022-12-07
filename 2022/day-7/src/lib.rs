pub mod bin;
pub mod parse;

use self::parse::build_directory_structure;
use std::{cell::RefCell, collections::HashMap, rc::Rc};

#[derive(Clone, Debug, PartialEq)]
pub struct Directory {
    name: String,
    files: Vec<u64>,
    subdirs: HashMap<String, Rc<RefCell<Directory>>>,
}

impl Directory {
    fn from_shell_output(shell_output: &str) -> Self {
        build_directory_structure(shell_output).unwrap().1
    }

    fn make_root() -> Self {
        Self {
            name: "/".to_string(),
            files: vec![],
            subdirs: HashMap::new(),
        }
    }

    fn with_name(name: String) -> Self {
        Self {
            name,
            files: vec![],
            subdirs: HashMap::new(),
        }
    }

    fn get_own_size(&self) -> u64 {
        self.files.iter().sum()
    }

    fn get_total_size(&self) -> u64 {
        self.subdirs
            .iter()
            .map(|(_, d)| d.borrow().get_total_size())
            .sum::<u64>()
            + self.get_own_size()
    }

    pub fn get_all_total_sizes(&self) -> Vec<u64> {
        self.subdirs
            .iter()
            .flat_map(|(_, d)| d.borrow().get_all_total_sizes())
            .chain([self.get_total_size()].iter().copied())
            .collect()
    }

    fn add_file(&mut self, file_size: u64) {
        self.files.push(file_size);
    }

    fn add_dir(&mut self, name: String) {
        self.subdirs
            .insert(name.clone(), Rc::new(RefCell::new(Self::with_name(name))));
    }

    fn cd<'s>(&self, name: &str) -> Rc<RefCell<Directory>> {
        Rc::clone(self.subdirs.get(name).unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k
";

    #[test]
    fn get_total_size_test() {
        let root = Directory::from_shell_output(INPUT);
        assert_eq!(root.get_total_size(), 48381165);
        assert_eq!(root.cd("a").borrow().get_total_size(), 94853);
    }
}
