use super::Solver;

#[derive(Debug)]
pub struct Solution{
    nums: Vec<i32>,
    solution: i32 ,
}

pub fn majority_element(nums: Vec<i32>) -> i32 {
    nums.iter().fold((0,0), |res, num| {
        if res.0 == 0 {
            (1, *num)
        } else if res.1 == *num {
            (res.0 + 1, res.1)
        } else {
            (res.0 - 1, res.1)
        }
    }).1
}

impl Solver for Solution {
    fn read_inputs() -> Self {
        return Solution {
            nums: vec![3,3,4],
            solution: -1,
        };
    }

    fn solve(mut self) {
        self.solution = majority_element(self.nums);
        dbg!(self.solution);
    }
}
