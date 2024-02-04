use super::Solver;

#[derive(Debug)]
pub struct Solution{
    nums: Vec<i32>,
    k: i32,
    solution: Vec<Vec<i32>>,
}

pub fn divide_array(mut nums: Vec<i32>, k: i32) -> Vec<Vec<i32>> {
    nums.sort();
    let mut result: Vec<Vec<i32>> = Vec::new();
    result.reserve(nums.len()/3);
    for chunk in nums.chunks(3) {
        if chunk[2] - chunk[0] > k {
            return vec![]
        }
        result.push(Vec::from(chunk));
    }

    return result
}

impl Solver for Solution {
    fn read_inputs() -> Self {
        return Solution {
            nums: vec![1,3,3,2,7,3],
            k: 3,
            solution: vec![],
        };
    }

    fn solve(mut self) {
        self.solution = divide_array(self.nums, self.k);
        dbg!(self.solution);
    }
}