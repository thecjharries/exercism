#[derive(Debug)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    if 0 == inputs.len() {
        return None;
    }
    let mut stack = Vec::new();
    for input in inputs {
        match input {
            CalculatorInput::Add => {
                if 2 > stack.len() {
                    return None;
                }
                let a = stack.pop().unwrap();
                let b = stack.pop().unwrap();
                stack.push(a + b);
            }
            CalculatorInput::Subtract => {
                if 2 > stack.len() {
                    return None;
                }
                let a = stack.pop().unwrap();
                let b = stack.pop().unwrap();
                stack.push(b - a);
            }
            CalculatorInput::Multiply => {
                if 2 > stack.len() {
                    return None;
                }
                let a = stack.pop().unwrap();
                let b = stack.pop().unwrap();
                stack.push(a * b);
            }
            CalculatorInput::Divide => {
                if 2 > stack.len() {
                    return None;
                }
                let a = stack.pop().unwrap();
                let b = stack.pop().unwrap();
                stack.push(b / a);
            }
            CalculatorInput::Value(value) => {
                stack.push(*value);
            }
        }
    }
    if 1 != stack.len() {
        return None;
    }
    Some(stack.pop().unwrap())
}
