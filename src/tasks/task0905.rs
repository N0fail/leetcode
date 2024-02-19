use super::Solver;

#[derive(Debug)]
pub struct Solution {
    nums: Vec<i32>,
    solution: Vec<i32>,
}

pub fn sort_array_by_parity(mut nums: Vec<i32>) -> Vec<i32> {
    let mut l = 0;
    let mut r = nums.len() - 1;
    loop {
        while l < r && nums[l] % 2 == 0 {
            l += 1;
        }
        while r > l && nums[r] % 2 == 1 {
            r -= 1;
        }
        if l == r {
            return nums;
        }
        (nums[l], nums[r]) = (nums[r], nums[l]); // 0ms
                                                 //nums.swap(l,r) // 3ms
    }
}

impl Solver for Solution {
    fn read_inputs() -> Self {
        return Solution {
            nums: vec![1, 0],
            solution: vec![],
        };
    }

    fn solve(mut self) {
        self.solution = sort_array_by_parity(self.nums);
        dbg!(self.solution);
    }
}
