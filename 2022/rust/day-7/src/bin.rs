use crate::*;

pub fn process_part1(s: &str) -> u64 {
    Directory::from_shell_output(s)
        .get_all_total_sizes()
        .iter()
        .filter(|&n| *n <= 100_000)
        .sum()
}

pub fn process_part2(s: &str) -> u64 {
    let root = Directory::from_shell_output(s);
    let needed_space = 30_000_000 - (70_000_000 - root.get_total_size());

    root.get_all_total_sizes()
        .iter()
        .filter(|&n| *n >= needed_space)
        .min()
        .copied()
        .unwrap()
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
    fn process_part1_test() {
        assert_eq!(process_part1(INPUT), 95437);
    }

    #[test]
    fn process_part2_test() {
        assert_eq!(process_part2(INPUT), 24933642);
    }
}
