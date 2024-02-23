// минимум среди максимальных расстояний до такого же элемента
use super::Solver;

#[derive(Debug)]
pub struct Solution {
    nums: Vec<i32>,
    solution: i32,
}

// pub fn minimum_seconds(nums: Vec<i32>) -> i32 {
//     // hashmap 54-62 ms, in theory O(n)
//     nums.iter()
//         .copied()
//         .chain(nums.iter().copied())
//         .enumerate()
//         .fold(HashMap::new(), |mut hm: HashMap<i32, (usize, usize)>, (idx, num)| {
//             hm.entry(num)
//                 .and_modify(|(prev_idx, max_dist)| {
//                     *max_dist = (*max_dist).max(idx - *prev_idx);
//                     *prev_idx = idx;
//                 })
//                 .or_insert((idx, 0));
//             hm
//         })
//         .iter()
//         .min_by_key(|(_, (_, max_dist))| *max_dist)
//         .map_or(0, |(_, (_, max_dist))| *max_dist/2) as i32
// }

// pub fn minimum_seconds(nums: Vec<i32>) -> i32 {
//     // sort vector of indexes 44-48ms, in theory O(n*log(n))
//     let mut idx_vec = nums.iter().copied().chain(nums.iter().copied()).enumerate().map(|(idx, num)| (num, idx)).collect::<Vec<_>>();
//     idx_vec.sort_unstable();
//     let res = idx_vec.windows(2).fold((0,nums.len()), |(cur_max, res), lr| {
//         let (prev_num, prev_idx) = lr[0];
//         let (cur_num, cur_idx) = lr[1];
//         if prev_num == cur_num {
//             (cur_max.max(cur_idx - prev_idx), res)
//         } else {
//             (0, res.min(cur_max))
//         }
//     });
//     res.1.min(res.0) as i32 / 2
// }

pub fn minimum_seconds(nums: Vec<i32>) -> i32 {
    // sort vector of indexes without duplication 22-24ms, in theory O(n*log(n))
    let mut idx_vec = nums
        .iter()
        .copied()
        .enumerate()
        .map(|(idx, num)| (num, idx))
        .collect::<Vec<_>>();
    idx_vec.sort_unstable();
    let n = nums.len();
    idx_vec.push((-1, n)); // pad
    idx_vec
        .windows(2)
        .fold((idx_vec[0].1, 0, n), |(fist_idx, cur_max, res), lr| {
            let (prev_num, prev_idx) = lr[0];
            let (cur_num, cur_idx) = lr[1];
            if prev_num == cur_num {
                (fist_idx, cur_max.max(cur_idx - prev_idx), res)
            } else {
                (cur_idx, 0, res.min(cur_max.max(fist_idx + n - prev_idx)))
            }
        })
        .2 as i32
        / 2
}

impl Solver for Solution {
    fn read_inputs() -> Self {
        return Solution {
            // nums: vec![1,4,2,3],
            nums: vec![1, 2, 3, 4],
            solution: 0,
        };
    }

    fn solve(mut self) {
        self.solution = minimum_seconds(self.nums);
        dbg!(self.solution);
    }
}
