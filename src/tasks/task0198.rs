use super::Solver;

#[derive(Debug)]
pub struct Solution {
    nums: Vec<i32>,
    solution: i32,
}

// pub fn rob(nums: Vec<i32>) -> i32 {
//     // my simple O(n) mem
//     let mut res = nums.clone();
//     if res.len() > 2 {
//         res[2] += res[0];
//     }
//     for i in 3..nums.len() {
//         res[i] += res[i-2].max(res[i-3])
//     }
//     res.into_iter().rev().take(2).max().unwrap()
// }

// pub fn rob(nums: Vec<i32>) -> i32 {
//     // some 5head O(1) mem
//     nums.into_iter().fold((0,0), |(pp, p), x| (p, p.max(pp+x))).1
// }

pub fn rob(nums: Vec<i32>) -> i32 {
    // my O(1) mem
    // prev[0] - element i-3
    // prev[1] - element i-2
    // prev[2] - element i-1
    nums.into_iter()
        .fold([0, 0, 0], |prev, x| [prev[1], prev[2], x + prev[1].max(prev[0])])
        .into_iter()
        .max()
        .unwrap()
}

impl Solver for Solution {
    fn read_inputs() -> Self {
        return Solution {
            nums: vec![2, 7, 9, 3, 1],
            solution: -1,
        };
    }

    fn solve(mut self) {
        self.solution = rob(self.nums);
        dbg!(self.solution);
    }
}
