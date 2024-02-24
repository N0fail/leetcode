use super::Solver;

#[derive(Debug)]
pub struct Solution {
    nums: Vec<i32>,
    l: Vec<i32>,
    r: Vec<i32>,
    solution: Vec<bool>,
}

// pub fn check_arithmetic_subarrays(nums: Vec<i32>, l: Vec<i32>, r: Vec<i32>) -> Vec<bool> {
//     // 12 ms, straightforward
//     l.into_iter()
//         .zip(r.into_iter())
//         .map(|(l, r)| {
//             let mut subarray = Vec::from_iter(nums[l as usize..=r as usize].iter().copied());
//             subarray.sort_unstable();
//             let expected = subarray[1] - subarray[0];
//             subarray.windows(2).all(|lr| lr[1] - lr[0] == expected)
//         })
//         .collect()
// }

// pub fn check_arithmetic_subarrays(nums: Vec<i32>, l: Vec<i32>, r: Vec<i32>) -> Vec<bool> {
//     // 4 ms, no sorting
//     l.into_iter()
//         .zip(r.into_iter())
//         .map(|(l, r)| {
//             let subarray = &nums[l as usize..=r as usize];
//             let min = subarray.iter().copied().min().unwrap();
//             let max = subarray.iter().copied().max().unwrap();
//             if (max - min) % (r-l) != 0 {
//                 return false
//             }
//             let dist = (max - min) / (r-l);
//             if dist == 0 {
//                 return true
//             }
//             let mut have = vec![false; (r-l+1) as usize];
//             for n in subarray.iter().copied() {
//                 let mut idx = n - min;
//                 if idx % dist != 0 {
//                     return false
//                 }
//                 idx /= dist;
//                 if have[idx as usize] {
//                     return false
//                 }
//                 have[idx as usize] = true
//             }
//             true
//         })
//         .collect()
// }

pub fn check_arithmetic_subarrays(nums: Vec<i32>, l: Vec<i32>, r: Vec<i32>) -> Vec<bool> {
    // 4 ms, no sorting, optimize allocations
    let mut have = vec![false; nums.len()];
    l.into_iter()
        .zip(r.into_iter())
        .map(|(l, r)| {
            let subarray = &nums[l as usize..=r as usize];
            let min = subarray.iter().copied().min().unwrap();
            let max = subarray.iter().copied().max().unwrap();
            if (max - min) % (r - l) != 0 {
                return false;
            }
            let dist = (max - min) / (r - l);
            if dist == 0 {
                return true;
            }
            have.iter_mut().take((r - l + 1) as usize).for_each(|x| *x = false);
            for n in subarray.iter().copied() {
                let mut idx = n - min;
                if idx % dist != 0 {
                    return false;
                }
                idx /= dist;
                if have[idx as usize] {
                    return false;
                }
                have[idx as usize] = true
            }
            true
        })
        .collect()
}

impl Solver for Solution {
    fn read_inputs() -> Self {
        return Solution {
            nums: vec![4, 6, 5, 9, 3, 7],
            l: vec![0, 0, 2],
            r: vec![2, 3, 5],
            solution: vec![],
        };
    }

    fn solve(mut self) {
        self.solution = check_arithmetic_subarrays(self.nums, self.l, self.r);
        dbg!(self.solution);
    }
}
