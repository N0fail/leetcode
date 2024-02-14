use std::collections::HashSet;
use super::Solver;

#[derive(Debug)]
pub struct Solution{
    nums: Vec<i32>,
    solution: i32,
}

fn reverse_num(mut num: i32) -> i32 {
    let mut res = 0;
    while num > 0 {
        res *= 10;
        res += num % 10;
        num /= 10;
    }
    res
}

pub fn count_distinct_integers(nums: Vec<i32>) -> i32 {
    let mut res = HashSet::with_capacity(nums.len());
    nums.into_iter().for_each(|num| {res.insert(num);res.insert(reverse_num(num));});
    res.len() as i32
}

impl Solver for Solution {
    fn read_inputs() -> Self {
        return Solution {
            nums: vec![1,13,10,12,31],
            solution: -1,
        };
    }

    fn solve(mut self) {
        self.solution = count_distinct_integers(self.nums);
        dbg!(self.solution);
    }
}
