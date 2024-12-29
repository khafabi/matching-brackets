pub fn brackets_are_balanced(string: &str) -> bool {
    fn matches(open: char, close: char) -> bool {
        match (open, close) {
            ('(', ')') | ('[', ']') | ('{', '}') => true,
            _ => false,
        }
    }

    fn check(chars: &[char], stack: Vec<char>) -> bool {
        match chars.split_first() {
            None => stack.is_empty(),
            Some((&current, rest)) => match current {
                '(' | '[' | '{' => check(rest, [stack, vec![current]].concat()),
                ')' | ']' | '}' => match stack.last() {
                    Some(&last) if matches(last, current) => {
                        check(rest, stack[..stack.len() - 1].to_vec())
                    }
                    _ => false,
                },
                _ => check(rest, stack),
            },
        }
    }

    check(&string.chars().collect::<Vec<_>>(), vec![])
}
