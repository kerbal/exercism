use std::collections::LinkedList;

fn is_brackets_matched (left: char, right: char) -> bool {
    match (left, right) {
        ('(', ')') => return true,
        ('[', ']') => return true,
        ('{', '}') => return true,
        _ => return false
    }
}

pub fn brackets_are_balanced(string: &str) -> bool {
    let mut stack: LinkedList<char> = LinkedList::new();

    for c in string.chars() {
        match c {
            '(' | '[' | '{' => stack.push_back(c),
            ')' | ']' | '}' => {
                if let Some(&stack_top) = stack.back() {
                    if is_brackets_matched(stack_top, c) {
                        stack.pop_back();
                    } else {
                        return false;
                    }
                } else {
                    return false;
                }
            },
            _ => {}
        }
    }

    return stack.is_empty();
}
