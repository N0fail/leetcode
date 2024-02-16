use super::Solver;
use std::collections::BinaryHeap;

#[derive(Debug)]
pub struct Solution {
    nums: Vec<i32>,
    solution: i64,
}

// pub fn largest_perimeter(mut nums: Vec<i32>) -> i64 {
//     // 24 ms
//     nums.sort_unstable();
//     let mut res: i64 = -1;
//     let mut current_sum: i64 = 0;
//     for (idx, num) in nums.into_iter().map(|x|x as i64).enumerate() {
//         if current_sum > num && idx > 1 {
//             res = current_sum + num;
//         }
//         current_sum += num;
//     }
//
//     res
// }

// pub fn largest_perimeter(mut nums: Vec<i32>) -> i64 {
//     // 20 ms
//     nums.sort_unstable();
//     let mut res = nums.iter().map(|x| *x as i64).sum();
//     for num in nums.into_iter().rev().map(|x| x as i64) {
//         if num * 2 < res {
//             return res;
//         }
//         res -= num;
//     }
//
//     -1
// }

pub fn largest_perimeter(nums: Vec<i32>) -> i64 {
    // 11 ms
    let mut res = nums.iter().map(|x| *x as i64).sum();
    let mut heap = BinaryHeap::from(nums);
    while !heap.is_empty() {
        let num = heap.pop().unwrap() as i64;
        if 2 * num < res {
            return res;
        }
        res -= num;
    }

    -1
}

impl Solver for Solution {
    fn read_inputs() -> Self {
        return Solution {
            nums: vec![1, 3, 2],
            solution: -1,
        };
    }

    fn solve(mut self) {
        self.solution = largest_perimeter(self.nums);
        dbg!(self.solution);
    }
}
