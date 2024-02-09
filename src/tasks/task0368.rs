use super::Solver;
#[derive(Debug)]
pub struct Solution{
    nums: Vec<i32>,
    solution: Vec<i32>,
}

pub fn largest_divisible_subset(mut nums: Vec<i32>) -> Vec<i32> {
    nums.sort();
    let mut result: Vec<(i32, Vec<i32>)> = Vec::with_capacity(nums.len());
    for num in nums {
        let mut longest_other = &Vec::new();
        for (other_num, other_dividers) in &result {
            if num % other_num == 0 && other_dividers.len() > longest_other.len() {
                longest_other = other_dividers;
            }
        }
        let mut tmp = longest_other.clone();
        tmp.push(num);
        result.push((num, tmp));
    }
    result.into_iter()
        .map(|(_, dividers)| dividers)
        .max_by(|a, b| a.len().cmp(&b.len()))
        .unwrap()
}

impl Solver for Solution {
    fn read_inputs() -> Self {
        return Solution {
            nums: vec![1,2,4,8,9,72],
            solution: vec![],
        };
    }

    fn solve(mut self) {
        self.solution = largest_divisible_subset(self.nums);
        dbg!(self.solution);
    }
}
