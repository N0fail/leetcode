use std::cmp::min;
use super::Solver;


#[derive(Debug)]
pub struct Solution{
    matrix: Vec<Vec<i32>>,
    solution: i32,
}

pub fn min_falling_path_sum(matrix: Vec<Vec<i32>>) -> i32 {
    let n = matrix.len();
    let mut prev: Vec<i32> = matrix[0].clone();
    let mut cur: Vec<i32> = matrix[0].clone();
    for row in matrix[1..].iter(){
        for j in 0..n {
            let mut minv = prev[j];
            if j > 0 {
                minv = min(minv, prev[j-1])
            }
            if j + 1 < n {
                minv = min(minv, prev[j+1])
            }
            cur[j] = row[j] + minv;
        }
        (cur, prev) = (prev, cur)
    }
    return *prev.iter().min().unwrap()
}

impl Solver for Solution {
    fn read_inputs() -> Self {
        return Solution {
            matrix: vec![
                vec![100,-42,-46,-41],
                vec![31,97,10,-10],
                vec![-58,-51,82,89],
                vec![51,81,69,-51],
            ],
            solution: 0,
        };
    }

    fn solve(mut self) {
        self.solution = min_falling_path_sum(self.matrix);
        dbg!(self.solution);
    }
}