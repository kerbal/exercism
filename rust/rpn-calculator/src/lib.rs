#[derive(Debug, Copy, Clone)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

pub fn get_operator_priority (operator: CalculatorInput) -> u8 {
    use CalculatorInput::*;
    match operator {
        Add => 1,
        Subtract => 1,
        Multiply => 2,
        Divide => 2,
        _ => 0,
    }
}

pub fn infix_to_postfix (input: &[CalculatorInput]) -> Vec<CalculatorInput> {
    use CalculatorInput::*;

    let (mut stack, mut postfix): (Vec<CalculatorInput>, Vec<CalculatorInput>) = input.iter().fold((vec![], vec![]), |(mut stack, mut postfix), e| {
        match e {
            Value(_) => postfix.push(*e),
            _ => {
                match stack.last() {
                    None => stack.push(*e),
                    Some(top) => {
                        if get_operator_priority(*top) > get_operator_priority(*e) {
                            postfix.push(*top);
                            stack.pop();
                            stack.push(*e);
                        } else {
                            stack.push(*e);
                        }
                    }
                }
            }
        }
        return (stack, postfix);
    });
    stack.reverse();
    postfix.extend(&stack);
    return postfix;
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    use CalculatorInput::*;
    let stack: Option<Vec<i32>> = inputs.iter().fold(Some(vec![]), |st, &e| {
        match st {
            None => None,
            Some(mut stack) => {
                println!("{:?}", stack);
                match e {
                    Value(v) => stack.push(v),
                    operator if stack.len() >= 2 => {
                        match (stack.last(), stack.get(stack.len() - 2)) {
                            (None, _) => return None,
                            (_, None) => return None,
                            (Some(a), Some(b)) => {
                                let x: i32;
                                match operator {
                                    Add => x = a + b,
                                    Subtract => x = b - a,
                                    Multiply => x = a * b,
                                    Divide => x = b / a,
                                    _ => return None,
                                }
                                stack.pop();
                                stack.pop();
                                stack.push(x);
                            },
                        }
                    },
                    _ => return None,
                }
                return Some(stack);
            }
        }
    });
    match stack {
        None => None,
        Some(s) => {
            if s.len() > 1 { return None }
            match s.get(0) {
                None => None,
                Some(&value) => Some(value)
            }
        }
    }
}
