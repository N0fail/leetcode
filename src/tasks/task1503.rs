use super::Solver;

#[derive(Debug)]
pub struct Solution{
    n: i32,
    left: Vec<i32>,
    right: Vec<i32>,
    solution: i32,
}

pub fn get_last_moment(n: i32, left: Vec<i32>, right: Vec<i32>) -> i32 {
    left.into_iter()
        .max().unwrap_or(0)
        .max(right.into_iter()
            .map(|x| n - x).max().unwrap_or(0)
        )
}

impl Solver for Solution {
    fn read_inputs() -> Self {
        return Solution {
            n: 4,
            left: vec![4,3],
            right: vec![0,1],
            solution: -1,
        };
    }

    fn solve(mut self) {
        self.solution = get_last_moment(self.n, self.left, self.right);
        dbg!(self.solution);
    }
}
