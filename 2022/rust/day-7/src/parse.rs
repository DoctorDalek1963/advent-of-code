use crate::Directory;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, alpha1, alphanumeric1, digit1, newline},
    multi::{many1, separated_list1},
    IResult,
};
use std::cell::RefCell;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum ShellCommand<'s> {
    Cd(&'s str),
    Ls,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum DirNameOrFileSize<'s> {
    DirName(&'s str),
    FileSize(u64),
}

fn parse_shell_prompt(input: &str) -> IResult<&str, ShellCommand> {
    fn parse_cd(input: &str) -> IResult<&str, ShellCommand> {
        let (input, _) = tag("cd ")(input)?;
        let (input, dir) = alt((alphanumeric1, tag("..")))(input)?;
        Ok((input, ShellCommand::Cd(dir)))
    }

    fn parse_ls(input: &str) -> IResult<&str, ShellCommand> {
        let (input, _) = tag("ls")(input)?;
        Ok((input, ShellCommand::Ls))
    }

    let (input, _) = tag("$ ")(input)?;
    let (input, command) = alt((parse_cd, parse_ls))(input)?;
    let (input, _) = newline(input)?;
    Ok((input, command))
}

fn parse_ls_output(input: &str) -> IResult<&str, Vec<DirNameOrFileSize>> {
    fn parse_file_size(input: &str) -> IResult<&str, DirNameOrFileSize> {
        let (input, size) = complete::u64(input)?;
        let (input, _) = tag(" ")(input)?;
        let (input, _) = many1(alt((tag("."), alpha1, digit1)))(input)?;

        Ok((input, DirNameOrFileSize::FileSize(size)))
    }

    fn parse_dir_name(input: &str) -> IResult<&str, DirNameOrFileSize> {
        let (input, _) = tag("dir ")(input)?;
        let (input, name) = alphanumeric1(input)?;

        Ok((input, DirNameOrFileSize::DirName(name)))
    }

    separated_list1(newline, alt((parse_file_size, parse_dir_name)))(input)
}

pub fn build_directory_structure(input: &str) -> IResult<&str, Directory> {
    use DirNameOrFileSize::*;

    let mut root = Directory::make_root();
    let mut cd_stack: Vec<&str> = vec![];

    let (input, _) = tag("$ cd /\n")(input)?;
    let input_container: RefCell<&str> = RefCell::new(input);

    loop {
        let input = *input_container.borrow();
        let (input, command) = parse_shell_prompt(input)?;
        let input = match command {
            ShellCommand::Ls => {
                let (input, files_and_dirs) = parse_ls_output(input)?;

                if cd_stack.len() == 0 {
                    for item in files_and_dirs {
                        match item {
                            DirName(name) => root.add_dir(name.to_string()),
                            FileSize(size) => root.add_file(size),
                        };
                    }
                } else {
                    let container = RefCell::new(root.cd(cd_stack.get(0).unwrap()));
                    for dir in &cd_stack[1..] {
                        container.replace_with(|x| x.borrow().cd(dir));
                    }
                    for item in files_and_dirs {
                        match item {
                            DirName(name) => (**container.borrow_mut())
                                .borrow_mut()
                                .add_dir(name.to_string()),
                            FileSize(size) => {
                                (**container.borrow_mut()).borrow_mut().add_file(size)
                            }
                        };
                    }
                }

                let (input, _) = newline(input)?;
                input
            }
            ShellCommand::Cd(name) => {
                if name == ".." {
                    cd_stack.pop();
                } else {
                    cd_stack.push(name);
                }
                input
            }
        };

        if input.is_empty() {
            return Ok((input, root));
        }

        input_container.replace(input);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_shell_prompt_test() {
        assert_eq!(
            parse_shell_prompt("$ cd somedir\n"),
            Ok(("", ShellCommand::Cd("somedir")))
        );

        assert_eq!(
            parse_shell_prompt("$ cd ..\n"),
            Ok(("", ShellCommand::Cd("..")))
        );

        assert_eq!(parse_shell_prompt("$ ls\n"), Ok(("", ShellCommand::Ls)));
    }

    #[test]
    fn parse_ls_output_test() {
        use DirNameOrFileSize::*;

        let input = "dir e
29116 f
2557 g
62596 h.lst
57382 x
dir other";

        assert_eq!(
            parse_ls_output(input),
            Ok((
                "",
                vec![
                    DirName("e"),
                    FileSize(29116),
                    FileSize(2557),
                    FileSize(62596),
                    FileSize(57382),
                    DirName("other"),
                ]
            ))
        );
    }

    #[test]
    fn build_directory_structure_test() {
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

        let mut root = Directory::make_root();
        root.add_dir("a".to_string());
        root.add_file(14848514);
        root.add_file(8504156);
        root.add_dir("d".to_string());

        (*root.cd("a")).borrow_mut().add_dir("e".to_string());
        (*root.cd("a")).borrow_mut().add_file(29116);
        (*root.cd("a")).borrow_mut().add_file(2557);
        (*root.cd("a")).borrow_mut().add_file(62596);
        (*(*root.cd("a")).borrow_mut().cd("e"))
            .borrow_mut()
            .add_file(584);
        (*root.cd("d")).borrow_mut().add_file(4060174);
        (*root.cd("d")).borrow_mut().add_file(8033020);
        (*root.cd("d")).borrow_mut().add_file(5626152);
        (*root.cd("d")).borrow_mut().add_file(7214296);

        assert_eq!(build_directory_structure(INPUT), Ok(("", root)));
    }
}
