use super::Solver;
#[derive(Debug)]
pub struct Solution {
    s: String,
    solution: i32,
}

pub fn count_substrings(s: String) -> i32 {
    let s = s.as_bytes();
    let mut res = 0;
    for i in 0..s.len() {
        s.iter()
            .rev()
            .skip(s.len() - i)
            .zip(s.iter().skip(i))
            .take_while(|(l, r)| **l == **r)
            .for_each(|_| res += 1);

        s.iter()
            .rev()
            .skip(s.len() - i - 1)
            .zip(s.iter().skip(i))
            .take_while(|(l, r)| **l == **r)
            .for_each(|_| res += 1);
    }

    res
}

impl Solver for Solution {
    fn read_inputs() -> Self {
        return Solution {
            s: "aaaa".to_string(),
            solution: -1,
        };
    }

    fn solve(mut self) {
        self.solution = count_substrings(self.s);
        dbg!(self.solution);
    }
}
