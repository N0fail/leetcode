use super::Solver;
#[derive(Debug)]
pub struct Solution {
    s: String,
    t: String,
    solution: i32,
}

pub fn min_steps(s: String, t: String) -> i32 {
    let mut count_s = [0i32; 26];
    for b in s.as_bytes() {
        count_s[(b - b'a') as usize] += 1;
    }
    for b in t.as_bytes() {
        count_s[(b - b'a') as usize] -= 1;
    }
    count_s.into_iter().map(|count| count.abs()).sum::<i32>() >> 1
}

impl Solver for Solution {
    fn read_inputs() -> Self {
        return Solution {
            s: "qwertyu".to_string(),
            t: "asddfgh".to_string(),
            solution: -1,
        };
    }

    fn solve(mut self) {
        self.solution = min_steps(self.s, self.t);
        dbg!(self.solution);
    }
}
