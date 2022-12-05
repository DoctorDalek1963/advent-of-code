use day_5::*;

fn process(s: &str) -> String {
    let (mut stacks, instructions) = parse_whole_file(s);
    for inst in instructions {
        inst.perform(&mut stacks);
    }

    let mut rv = String::new();
    for stack in stacks {
        rv.push(*stack.last().unwrap());
    }

    rv
}

fn main() {
    println!("{}", process(&get_input()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn process_test() {
        const INPUT: &str = "    [G] [R]                 [P]    
    [H] [W]     [T] [P]     [H]    
    [F] [T] [P] [B] [D]     [N]    
[L] [T] [M] [Q] [L] [C]     [Z]    
[D] [G] [O] [A] [K] [J] [A] [H] [G]
 1   2   3   4   5   6   7   8   9 

move 3 from 8 to 2
move 2 from 4 to 5
move 3 from 3 to 9
";

        assert_eq!(process(INPUT), String::from("LNMAQPAZT"));
    }
}
