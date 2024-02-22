use super::Solver;

#[derive(Debug)]
pub struct Solution {
    nums: Vec<i32>,
    solution: Vec<i32>,
}

// pub fn get_sum_absolute_differences(nums: Vec<i32>) -> Vec<i32> {
//     // 28ms 3.53 Mb
//     let sum_abs_left = nums
//         .windows(2)
//         .enumerate()
//         .fold(vec![0; nums.len()], |mut sum_abs_left, (idx, lr)| {
//             sum_abs_left[idx + 1] = (lr[1] - lr[0]) * (idx+1) as i32 + sum_abs_left[idx];
//             sum_abs_left
//         });
//     let sum_abs_right = nums
//         .windows(2)
//         .enumerate()
//         .rev()
//         .fold(vec![0; nums.len()], |mut sum_abs_right, (idx, lr)| {
//             sum_abs_right[idx] = (lr[1] - lr[0]) * (nums.len() - idx - 1) as i32 + sum_abs_right[idx + 1];
//             sum_abs_right
//         });
//     sum_abs_left
//         .into_iter()
//         .zip(sum_abs_right.into_iter())
//         .map(|(l, r)| l + r)
//         .collect()
// }

pub fn get_sum_absolute_differences(nums: Vec<i32>) -> Vec<i32> {
    // 28ms 3.36Mb
    nums.windows(2).enumerate().fold(
        vec![nums.iter().sum::<i32>() - nums[0] * nums.len() as i32; nums.len()],
        |mut res, (idx, lr)| {
            let diff = lr[1] - lr[0];
            // we increased sum for elements to the left and reduced for elements to the right
            // res[idx + 1] = res[idx] + diff * (idx+1) as i32 - diff * (nums.len() - idx - 1) as i32;
            // same thing, after math simplification
            res[idx + 1] = res[idx] + diff * (2 * (idx + 1) as i32 - nums.len() as i32);
            res
        },
    )
}

impl Solver for Solution {
    fn read_inputs() -> Self {
        return Solution {
            nums: vec![1, 2, 3],
            solution: vec![],
        };
    }

    fn solve(mut self) {
        self.solution = get_sum_absolute_differences(self.nums);
        dbg!(self.solution);
    }
}
