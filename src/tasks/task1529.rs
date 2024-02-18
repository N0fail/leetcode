use super::Solver;

#[derive(Debug)]
pub struct Solution {
    target: String,
    solution: i32,
}

pub fn min_flips(target: String) -> i32 {
    let vec_chars: Vec<char> = target.chars().collect();
    let res = match vec_chars[0] {
        '1' => 1,
        _ => 0,
    };
    vec_chars.windows(2).fold(res, |res, lr| match lr[0] == lr[1] {
        true => res,
        false => res + 1,
    })
}

impl Solver for Solution {
    fn read_inputs() -> Self {
        return Solution {
            target: "0000".to_string(),
            solution: -1,
        };
    }

    fn solve(mut self) {
        self.solution = min_flips(self.target);
        dbg!(self.solution);
    }
}
