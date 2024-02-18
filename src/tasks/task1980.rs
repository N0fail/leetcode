use super::Solver;
use std::collections::HashSet;
#[derive(Debug)]
pub struct Solution {
    nums: Vec<String>,
    solution: String,
}

pub fn find_different_binary_string(nums: Vec<String>) -> String {
    // easy solution
    let values: HashSet<_> = nums.iter().map(|s| u16::from_str_radix(s, 2).unwrap()).collect();
    let ans = (0..u16::MAX).find(|num| !values.contains(num)).unwrap();
    let n = nums.len();
    format!("{ans:0n$b}")
}

// pub fn find_different_binary_string(nums: Vec<String>) -> String {
//     // some 5head solution with Cantor diagonal
//     let res = nums.iter().enumerate().fold(Vec::with_capacity(nums.len()), |mut res, (idx, s)|{
//         res.push(b'1' - s.bytes().skip(idx).next().unwrap() + b'0');
//         res
//     });
//     String::from_utf8(res).unwrap()
// }

impl Solver for Solution {
    fn read_inputs() -> Self {
        let input = ["111", "011", "000"];
        return Solution {
            nums: input.map(|s| String::from(s)).into_iter().collect(),
            solution: "".to_string(),
        };
    }

    fn solve(mut self) {
        self.solution = find_different_binary_string(self.nums);
        dbg!(self.solution);
    }
}
