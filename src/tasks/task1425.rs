use super::Solver;
use std::collections::BinaryHeap;

#[derive(Debug)]
pub struct Solution {
    nums: Vec<i32>,
    k: i32,
    solution: i32,
}

pub fn constrained_subset_sum(nums: Vec<i32>, k: i32) -> i32 {
    // 37 ms
    let k = k as usize;
    let mut dp = vec![i32::MIN; nums.len()];
    dp[0] = nums[0];
    let mut heap = BinaryHeap::new();
    heap.push((dp[0], 0));
    for (idx, num) in nums.iter().enumerate().skip(1) {
        while heap.peek().unwrap().1 + k < idx {
            heap.pop();
        }
        dp[idx] = *num + 0.max(heap.peek().unwrap().0);
        heap.push((dp[idx], idx));
    }

    *dp.iter().max().unwrap()
}

// pub fn constrained_subset_sum(nums: Vec<i32>, k: i32) -> i32 {
//     // 837 ms
//     let k = k as usize;
//     let mut dp = vec![i32::MIN; nums.len()];
//     dp[0] = nums[0];
//     let mut cur_max = (dp[0], 0);
//     for (idx, num) in nums.iter().enumerate().skip(1) {
//         if cur_max.1 + k < idx {
//             cur_max = dp[(cur_max.1+1).max(idx-k)..idx]
//                 .iter()
//                 .enumerate()
//                 .rev()
//                 .map(|(idx, el)| (*el, idx))
//                 .max()
//                 .unwrap();
//         }
//         dp[idx] = *num + 0.max(cur_max.0);
//         cur_max = cur_max.max((dp[idx], idx));
//     }
//
//     *dp.iter().max().unwrap()
// }

// pub fn constrained_subset_sum(nums: Vec<i32>, k: i32) -> i32 {
//     // 27 ms
//     let k = k as usize;
//     let mut dp = vec![i32::MIN; nums.len()];
//     dp[0] = nums[0];
//     let mut res = dp[0];
//     let mut deque = VecDeque::new();
//     deque.push_back(0);
//     for (idx, num) in nums.iter().enumerate().skip(1) {
//         while deque[0] + k < idx {
//             deque.pop_front();
//         }
//         dp[idx] = *num + 0.max(dp[deque[0]]);
//         res = res.max(dp[idx]);
//         while !deque.is_empty() && dp[idx] >= dp[*deque.back().unwrap()] {
//             deque.pop_back();
//         }
//         deque.push_back(idx)
//     }
//
//     res
// }

impl Solver for Solution {
    fn read_inputs() -> Self {
        return Solution {
            nums: vec![10, -2, -10, -5, 20],
            k: 2,
            solution: -1,
        };
    }

    fn solve(mut self) {
        self.solution = constrained_subset_sum(self.nums, self.k);
        dbg!(self.solution);
    }
}
