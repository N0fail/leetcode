use super::Solver;
#[derive(Debug)]
pub struct Solution{
    nums: Vec<i32>,
    solution: Vec<i32>,
}

pub fn smaller_numbers_than_current(nums: Vec<i32>) -> Vec<i32> {
    let mut tmp: Vec<_> = nums.clone().into_iter().enumerate().collect();
    tmp.sort_by(|a, b| a.1.cmp(&b.1));
    let mut count = 0;
    tmp.windows(2).enumerate()
        .fold(vec![0; nums.len()], |mut res, (idx, lr)| {
            if lr[0].1 != lr[1].1 {
                count = idx as i32 + 1;
            }
            res[lr[1].0] = count;
            res
        })
}

impl Solver for Solution {
    fn read_inputs() -> Self {
        return Solution {
            nums: vec![8,1,2,2,3],
            solution: vec![],
        };
    }

    fn solve(mut self) {
        self.solution = smaller_numbers_than_current(self.nums);
        dbg!(self.solution);
    }
}
