use super::Solver;


#[derive(Debug)]
pub struct Solution{
    tokens: Vec<String>,
    solution: i32,
}

pub fn eval_rpn(tokens: Vec<String>) -> i32 {
    let mut stack: Vec<i32> = vec![];
    for token in &tokens{
        match token.parse::<i32>() {
            Ok(number) => stack.push(number),
            Err(_) => {
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                let c = token.chars().next().unwrap();
                match c {
                    '+' => stack.push(a+b),
                    '-' => stack.push(a-b),
                    '*' => stack.push(a*b),
                    '/' => stack.push(a/b),
                    _ => {}
                }
            }
        }
    }
    return stack.pop().unwrap()
}
impl Solver for Solution {
    fn read_inputs() -> Self {
        let input = vec!["10","6","9","3","+","-11","*","/","*","17","+","5","+"];
        let mut tokens: Vec<String> = vec![];
        tokens.reserve(input.len());
        for s in input {
            tokens.push(String::from(s));
        }
        return Solution {
            tokens,
            solution: 0,
        };
    }

    fn solve(mut self) {
        self.solution = eval_rpn(self.tokens);
        dbg!(self.solution);
    }
}