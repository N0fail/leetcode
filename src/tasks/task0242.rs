use super::Solver;
#[derive(Debug)]
pub struct Solution{
    s: String,
    t: String,
    solution: bool,
}

pub fn is_anagram(s: String, t: String) -> bool {
    let mut res = s.as_bytes().iter().fold([0;26],|mut res, b| {res[(*b - b'a') as usize] += 1; res});
    res = t.as_bytes().iter().fold(res,|mut res, b| {res[(*b - b'a') as usize] -= 1; res});
    res.iter().all(|x| *x == 0)
}

impl Solver for Solution {
    fn read_inputs() -> Self {
        return Solution {
            s: "rat".to_string(),
            t: "car".to_string(),
            solution: true,
        };
    }

    fn solve(mut self) {
        self.solution = is_anagram(self.s, self.t);
        dbg!(self.solution);
    }
}
