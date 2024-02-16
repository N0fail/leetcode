use super::Solver;

#[derive(Debug)]
pub struct Solution {
    nums: Vec<i32>,
    solution: i32,
}

fn sum_of_digits(mut num: i32) -> i32 {
    let mut res = 0;
    while num > 0 {
        res += num % 10;
        num /= 10;
    }
    res
}
pub fn difference_of_sum(nums: Vec<i32>) -> i32 {
    nums.into_iter().map(|x| x - sum_of_digits(x)).sum()
}

impl Solver for Solution {
    fn read_inputs() -> Self {
        return Solution {
            nums: vec![1, 15, 6, 3],
            solution: -1,
        };
    }

    fn solve(mut self) {
        self.solution = difference_of_sum(self.nums);
        dbg!(self.solution);
    }
}
