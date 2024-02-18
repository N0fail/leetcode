use super::Solver;

#[derive(Debug)]
pub struct Solution {
    nums: Vec<i32>,
    multipliers: Vec<i32>,
    solution: i32,
}

// fn rec(nums: &[i32], multipliers: &[i32], left_picked: usize, right_picked: usize, cache: &mut[Option<i32>;1000_000]) -> i32 {
//     // 32 ms
//     if multipliers.len() == 0 {
//         return 0
//     }
//     if let Some(x) = cache[left_picked*1000+right_picked] {
//         return x
//     }
//     let left = nums[0] * multipliers[0] + rec(&nums[1..], &multipliers[1..], left_picked+1, right_picked, cache);
//     let right = nums[nums.len()-1] * multipliers[0] + rec(&nums[..nums.len()-1], &multipliers[1..], left_picked, right_picked+1, cache);
//     let res = left.max(right);
//     cache[left_picked*1000+right_picked] = Some(res);
//     res
// }
//
// pub fn maximum_score(nums: Vec<i32>, multipliers: Vec<i32>) -> i32 {
//     rec(&nums, &multipliers, 0, 0, &mut [None;1000_000])
// }

// fn rec(nums: &[i32], multipliers: &[i32], left_picked: usize, right_picked: usize, cache: &mut[i32;1000_000]) -> i32 {
//     // 24ms
//     if multipliers.len() == 0 {
//         return 0
//     }
//     if cache[left_picked*1000+right_picked] == i32::MAX {
//         let left = nums[0] * multipliers[0] + rec(&nums[1..], &multipliers[1..], left_picked+1, right_picked, cache);
//         let right = nums[nums.len()-1] * multipliers[0] + rec(&nums[..nums.len()-1], &multipliers[1..], left_picked, right_picked+1, cache);
//         let res = left.max(right);
//         cache[left_picked*1000+right_picked] = res;
//     }
//     cache[left_picked*1000+right_picked]
// }
//
// pub fn maximum_score(nums: Vec<i32>, multipliers: Vec<i32>) -> i32 {
//     rec(&nums, &multipliers, 0, 0, &mut [i32::MAX;1000_000])
// }

// fn rec(nums: &[i32], multipliers: &[i32], left_picked: usize, right_picked: usize, cache: &mut[i32;1000_000]) -> i32 {
//     // 18ms
//     if multipliers.len() == left_picked+right_picked {
//         return 0
//     }
//     if cache[left_picked*1000+right_picked] == i32::MAX {
//         let left = nums[left_picked] * multipliers[left_picked+right_picked] + rec(nums, multipliers, left_picked+1, right_picked, cache);
//         let right = nums[nums.len()-right_picked-1] * multipliers[left_picked+right_picked] + rec(nums, multipliers, left_picked, right_picked+1, cache);
//         let res = left.max(right);
//         cache[left_picked*1000+right_picked] = res;
//     }
//     cache[left_picked*1000+right_picked]
// }
//
// impl Solution {
//     pub fn maximum_score(nums: Vec<i32>, multipliers: Vec<i32>) -> i32 {
//         rec(&nums, &multipliers, 0, 0, &mut [i32::MAX;1000_000])
//     }
// }

// fn rec(nums: &[i32], multipliers: &[i32], left_picked: usize, right_picked: usize, cache: &mut [[i32; 1000]; 1000]) -> i32 {
//     // 25 ms
//     if multipliers.len() == left_picked+right_picked {
//         return 0
//     }
//     if cache[left_picked][right_picked] == i32::MAX {
//         let left = nums[left_picked] * multipliers[left_picked+right_picked] + rec(nums, multipliers, left_picked+1, right_picked, cache);
//         let right = nums[nums.len()-right_picked-1] * multipliers[left_picked+right_picked] + rec(nums, multipliers, left_picked, right_picked+1, cache);
//         let res = left.max(right);
//         cache[left_picked][right_picked] = res;
//     }
//     cache[left_picked][right_picked]
// }
//
// pub fn maximum_score(nums: Vec<i32>, multipliers: Vec<i32>) -> i32 {
//     rec(&nums, &multipliers, 0, 0, &mut [[i32::MAX; 1000]; 1000])
// }

pub fn maximum_score(nums: Vec<i32>, multipliers: Vec<i32>) -> i32 {
    // unrolled recursion
    let m = multipliers.len();
    let n = nums.len();
    let mut cache = vec![vec![0; m + 1]; m + 1]; // 11 ms
                                                 // let mut cache = [[0; 1001]; 1001]; // 15 ms
    for total in (0..m).rev() {
        for left_picked in (0..=total).rev() {
            let right_picked = total - left_picked;
            let left = nums[left_picked] * multipliers[total] + cache[left_picked + 1][right_picked];
            let right = nums[n - right_picked - 1] * multipliers[total] + cache[left_picked][right_picked + 1];
            cache[left_picked][right_picked] = left.max(right);
        }
    }
    cache[0][0]
}

// pub fn maximum_score(nums: Vec<i32>, multipliers: Vec<i32>) -> i32 {
//     // unrolled recursion
//     let m = multipliers.len();
//     let n = nums.len();
//     // let mut cache = vec![0; (m+1)*(m+1)]; // 14 ms
//     let mut cache = [0; 1001 * 1001]; // 20ms
//     for total in (0..m).rev() {
//         for left_picked in (0..=total).rev() {
//             let right_picked = total - left_picked;
//             let left = nums[left_picked] * multipliers[total] + cache[(left_picked+1)*m+right_picked];
//             let right = nums[n-right_picked-1] * multipliers[total] + cache[left_picked*m+right_picked+1];
//             cache[left_picked*m+right_picked] = left.max(right);
//         }
//     }
//     cache[0]
// }

impl Solver for Solution {
    fn read_inputs() -> Self {
        return Solution {
            nums: vec![1, 2, 3],
            multipliers: vec![3, 2, 1],
            solution: -1,
        };
    }

    fn solve(mut self) {
        self.solution = maximum_score(self.nums, self.multipliers);
        dbg!(self.solution);
    }
}
