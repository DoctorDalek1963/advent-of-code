pub mod bin;

const CLOSE_DELIMS: [char; 4] = [')', ']', '}', '>'];

/// Return the appropriate pair for the given delimiter.
fn pair(delim: char) -> char {
    match delim {
        '(' => ')',
        '[' => ']',
        '{' => '}',
        '<' => '>',
        ')' => '(',
        ']' => '[',
        '}' => '{',
        '>' => '<',
        _ => unreachable!("Delimiters must be one of ()[]{{}}<>"),
    }
}

/// Find the corruption score of the given line.
pub fn find_corruption_score(line: &str) -> u32 {
    let mut stack: Vec<char> = vec![];

    for c in line.chars() {
        // Push an open delim on the stack
        if ['(', '[', '{', '<'].contains(&c) {
            stack.push(c);

        // If we encounter a close delim, then pop the stack if it's correct. Else, print the error
        // and return the score for that character
        } else if CLOSE_DELIMS.contains(&c) {
            if *stack.last().unwrap() == pair(c) {
                stack.pop();
            } else {
                eprintln!(
                    "Expected {exp:?} but got {got:?}",
                    exp = pair(*stack.last().unwrap()),
                    got = c,
                );
                return match c {
                    ')' => 3,
                    ']' => 57,
                    '}' => 1197,
                    '>' => 25137,
                    _ => unreachable!("c must be a closing delimiter"),
                };
            }
        }
    }

    0
}

/// Find the autocomplete score for the given line.
pub fn find_autocomplete_score(line: &str) -> u64 {
    let mut stack: Vec<char> = vec![];

    for c in line.chars() {
        // Push an open delim on the stack
        if ['(', '[', '{', '<'].contains(&c) {
            stack.push(c);

        // If we encounter a close delim, then pop the stack if it's correct. Else, panic
        } else if CLOSE_DELIMS.contains(&c) {
            if *stack.last().unwrap() == pair(c) {
                stack.pop();
            } else {
                panic!("This line is corrupt, and cannot be autocompleted: {line:?}");
            }
        }
    }

    let mut score = 0;
    for c in stack.into_iter().rev().map(pair) {
        score *= 5;
        score += match c {
            ')' => 1,
            ']' => 2,
            '}' => 3,
            '>' => 4,
            _ => unreachable!("Applying pair() should result in only closing delims here"),
        };
    }

    score
}

#[cfg(test)]
pub const TEST_INPUT: &str = r#"[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]"#;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_corruption_score_test() {
        assert_eq!(find_corruption_score("{([(<{}[<>[]}>{[]{[(<()>"), 1197);
        assert_eq!(find_corruption_score("[[<[([]))<([[{}[[()]]]"), 3);
        assert_eq!(find_corruption_score("[{[{({}]{}}([{[{{{}}([]"), 57);
        assert_eq!(find_corruption_score("[<(<(<(<{}))><([]([]()"), 3);
        assert_eq!(find_corruption_score("<{([([[(<>()){}]>(<<{{"), 25137);
    }

    #[test]
    fn find_autocomplete_score_test() {
        assert_eq!(find_autocomplete_score("[({(<(())[]>[[{[]{<()<>>"), 288957);
        assert_eq!(find_autocomplete_score("[(()[<>])]({[<{<<[]>>("), 5566);
        assert_eq!(find_autocomplete_score("(((({<>}<{<{<>}{[]{[]{}"), 1480781);
        assert_eq!(find_autocomplete_score("{<[[]]>}<{[{[{[]{()[[[]"), 995444);
        assert_eq!(find_autocomplete_score("<{([{{}}[<[[[<>{}]]]>[]]"), 294);
    }
}
