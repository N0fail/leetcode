use super::Solver;

#[derive(Debug)]
pub struct Solution {
    nums: Vec<i32>,
    solution: Vec<i32>,
}

// pub fn rearrange_array(nums: Vec<i32>) -> Vec<i32> {
//     // 71 ms
//     let neg_iter = nums.iter().filter(|x| **x < 0).copied();
//     let pos_iter = nums.iter().filter(|x| **x > 0).copied();
//     pos_iter.zip(neg_iter).map(|(pos, neg)| [pos, neg]).flatten().collect()
// }

// pub fn rearrange_array(nums: Vec<i32>) -> Vec<i32> {
//     // 66 ms
//     let neg_iter = nums.iter().filter(|x| **x < 0).copied();
//     let pos_iter = nums.iter().filter(|x| **x > 0).copied();
//     pos_iter.zip(neg_iter)
//         .fold(Vec::with_capacity(nums.len()), |mut res, (pos, neg)|{
//             res.push(pos);
//             res.push(neg);
//             res
//         })
// }

// pub fn rearrange_array(nums: Vec<i32>) -> Vec<i32> {
//     // 63 ms
//     let mut pos_i = 0;
//     let mut neg_i = 0;
//     let mut res = Vec::with_capacity(nums.len());
//     while res.len() < nums.len() {
//         while nums[pos_i] < 0 {
//             pos_i += 1;
//         }
//         res.push(nums[pos_i]);
//         pos_i += 1;
//         while nums[neg_i] > 0 {
//             neg_i += 1;
//         }
//         res.push(nums[neg_i]);
//         neg_i += 1;
//     }
//     res
// }

pub fn rearrange_array(nums: Vec<i32>) -> Vec<i32> {
    // 54 ms, one pass
    let mut pos_i = 0;
    let mut neg_i = 1;
    let mut res = vec![0; nums.len()];
    for el in nums {
        if el > 0 {
            res[pos_i] = el;
            pos_i += 2;
        } else {
            res[neg_i] = el;
            neg_i += 2;
        }
    }
    res
}

impl Solver for Solution {
    fn read_inputs() -> Self {
        return Solution {
            nums: vec![3, 1, -2, -5, 2, -4],
            solution: vec![],
        };
    }

    fn solve(mut self) {
        self.solution = rearrange_array(self.nums);
        dbg!(self.solution);
    }
}
