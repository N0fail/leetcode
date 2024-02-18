use super::Solver;

#[derive(Debug)]
pub struct Solution {
    low: i32,
    high: i32,
    solution: Vec<i32>,
}

pub fn sequential_digits(low: i32, high: i32) -> Vec<i32> {
    let numbers = [
        12, 23, 34, 45, 56, 67, 78, 89, 123, 234, 345, 456, 567, 678, 789, 1234, 2345, 3456, 4567, 5678, 6789, 12345, 23456,
        34567, 45678, 56789, 123456, 234567, 345678, 456789, 1234567, 2345678, 3456789, 12345678, 23456789, 123456789,
    ];

    let mut result: Vec<i32> = Vec::new();
    for num in numbers {
        if num >= low && num <= high {
            result.push(num);
        }
    }

    return result;
}

impl Solver for Solution {
    fn read_inputs() -> Self {
        return Solution {
            low: 100,
            high: 300,
            solution: vec![],
        };
    }

    fn solve(mut self) {
        self.solution = sequential_digits(self.low, self.high);
        dbg!(self.solution);
    }
}
