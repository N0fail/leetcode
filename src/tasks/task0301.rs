use super::Solver;
use std::collections::HashSet;
#[derive(Debug)]
pub struct Solution {
    s: String,
    solution: Vec<String>,
}

fn build_stack(s: &[u8]) -> Vec<u8> {
    let mut stack: Vec<u8> = vec![];
    for b in s {
        match b {
            b'(' => stack.push(*b),
            b')' => {
                if stack.last().is_some_and(|top| *top == b'(') {
                    stack.pop();
                } else {
                    stack.push(*b);
                }
            }
            _ => (),
        }
    }
    stack
}

fn dfs(bytes: &mut [u8; 25], l_to_delete: usize, r_to_delete: usize, cur_idx: usize, n: usize, res: &mut HashSet<String>) {
    if l_to_delete == 0 && r_to_delete == 0 {
        if build_stack(&bytes[..n]).is_empty() {
            res.insert(String::from_utf8(bytes.iter().filter(|b| **b != 0).map(|x| *x).collect()).unwrap());
        }
        return;
    }

    for i in cur_idx..n - l_to_delete - r_to_delete + 1 {
        if bytes[i] == b'(' && l_to_delete > 0 {
            let save = bytes[i];
            bytes[i] = 0;
            dfs(bytes, l_to_delete - 1, r_to_delete, i + 1, n, res);
            bytes[i] = save;
        }

        if bytes[i] == b')' && r_to_delete > 0 {
            let save = bytes[i];
            bytes[i] = 0;
            dfs(bytes, l_to_delete, r_to_delete - 1, i + 1, n, res);
            bytes[i] = save;
        }
    }
}

pub fn remove_invalid_parentheses(s: String) -> Vec<String> {
    let mut bytes = [0u8; 25];
    let n = s.len();
    for (idx, b) in s.as_bytes().iter().enumerate() {
        bytes[idx] = *b;
    }
    let stack = build_stack(&bytes[..n]);
    let l_to_delete = stack.iter().filter(|b| **b == b'(').count();
    let r_to_delete = stack.len() - l_to_delete;
    let mut res: HashSet<String> = HashSet::new();
    dfs(&mut bytes, l_to_delete, r_to_delete, 0, n, &mut res);
    res.into_iter().collect::<Vec<_>>()
}

impl Solver for Solution {
    fn read_inputs() -> Self {
        return Solution {
            s: "x(".to_string(),
            solution: vec![],
        };
    }

    fn solve(mut self) {
        self.solution = remove_invalid_parentheses(self.s);
        dbg!(self.solution);
    }
}
