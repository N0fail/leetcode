use super::Solver;

#[derive(Debug)]
pub struct Solution {
    nums: Vec<i32>,
    solution: i32,
}

// pub fn missing_number(mut nums: Vec<i32>) -> i32 {
//     // sort solution 4ms
//     nums.sort_unstable();
//     let n = nums.len() as i32;
//     nums.into_iter()
//         .zip(0..n)
//         .filter(|(num, idx)| num != idx)
//         .next()
//         .unwrap_or((0, n))
//         .1
// }

// pub fn missing_number(nums: Vec<i32>) -> i32 {
//     // bool vec solution 2ms
//     let n = nums.len();
//     nums.into_iter()
//         .fold(vec![false; n + 1], |mut have, num| {
//             have[num as usize] = true;
//             have
//         })
//         .into_iter()
//         .enumerate()
//         .filter(|(_, have)| *have == false)
//         .next()
//         .unwrap()
//         .0 as _
// }

pub fn missing_number(nums: Vec<i32>) -> i32 {
    // sum solution 0ms
    let n = nums.len() as i32;
    (n * (n + 1)) / 2 - nums.iter().sum::<i32>()
}

impl Solver for Solution {
    fn read_inputs() -> Self {
        return Solution {
            nums: vec![3, 0, 1],
            solution: -1,
        };
    }

    fn solve(mut self) {
        self.solution = missing_number(self.nums);
        dbg!(self.solution);
    }
}
