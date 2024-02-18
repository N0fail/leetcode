use super::Solver;

#[derive(Debug)]
pub struct Solution {
    piles: Vec<i32>,
    solution: i32,
}

pub fn max_coins(mut piles: Vec<i32>) -> i32 {
    piles.sort_unstable();
    piles[piles.len() / 3..].iter().step_by(2).sum()
}

impl Solver for Solution {
    fn read_inputs() -> Self {
        return Solution {
            piles: vec![2, 4, 1, 2, 7, 8],
            solution: -1,
        };
    }

    fn solve(mut self) {
        self.solution = max_coins(self.piles);
        dbg!(self.solution);
    }
}
