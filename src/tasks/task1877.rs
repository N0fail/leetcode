use super::Solver;

#[derive(Debug)]
pub struct Solution {
    nums: Vec<i32>,
    solution: i32,
}

pub fn min_pair_sum(mut nums: Vec<i32>) -> i32 {
    // 31-35 ms
    nums.sort_unstable();
    let n = nums.len() / 2;
    nums[..n]
        .iter()
        .zip(nums[n..].iter().rev())
        .map(|(l, r)| l + r)
        .max()
        .unwrap()
}

impl Solver for Solution {
    fn read_inputs() -> Self {
        return Solution {
            nums: vec![3, 5, 2, 3],
            solution: 1,
        };
    }

    fn solve(mut self) {
        self.solution = min_pair_sum(self.nums);
        dbg!(self.solution);
    }
}
