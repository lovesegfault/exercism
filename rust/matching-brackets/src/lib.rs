const BRACKETS_OPEN: &[char] = &['[', '{', '('];
const BRACKETS_CLOSE: &[char] = &[']', '}', ')'];

pub fn brackets_are_balanced(string: &str) -> bool {
    // filter only the brackets
    let brackets: Vec<char> = string
        .chars()
        .filter(|c| BRACKETS_OPEN.contains(c) || BRACKETS_CLOSE.contains(c))
        .collect();

    let mut stack: Vec<char> = Vec::new();

    for b in brackets {
        if BRACKETS_OPEN.contains(&b) {
            stack.push(b);
            continue;
        }
        // we got a closing bracket
        let wants = match b {
            ']' => '[',
            '}' => '{',
            ')' => '(',
            _ => unreachable!(),
        };
        if !(stack.pop() == Some(wants)) {
            return false;
        }
    }

    stack.is_empty()
}
