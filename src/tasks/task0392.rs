use super::Solver;

#[derive(Debug)]
pub struct Solution {
    s: String,
    t: String,
    solution: bool,
}

pub fn is_subsequence(s: String, t: String) -> bool {
    let mut t_it = t.bytes();
    for b in s.bytes() {
        loop {
            match t_it.next() {
                Some(tb) => match tb == b {
                    true => break,
                    _ => (),
                },
                None => return false,
            }
        }
    }

    true
}

impl Solver for Solution {
    fn read_inputs() -> Self {
        return Solution {
            s: "axc".to_string(),
            t: "ahbgdc".to_string(),
            solution: false,
        };
    }

    fn solve(mut self) {
        self.solution = is_subsequence(self.s, self.t);
        dbg!(self.solution);
    }
}
