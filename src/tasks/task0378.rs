use std::cmp::max;
use super::Solver;

#[derive(Debug)]
pub struct Solution{
    input: Vec<Vec<i32>>,
    k: i32,
    solution: i32,
}

pub fn kth_smallest(matrix: Vec<Vec<i32>>, k: i32) -> i32 {
    let mut runners = vec![0; matrix.len()];
    fn step(matrix: &Vec<Vec<i32>>, runners: & mut Vec<usize>, len: usize){
        let mut min = matrix[len-1][len-1];
        let mut runner_to_move: usize = 0;
        for (row, cur_pos) in runners.iter().enumerate() {
            if *cur_pos >= len {
                continue;
            }

            if matrix[row][*cur_pos] <= min {
                runner_to_move = row;
                min = matrix[row][*cur_pos];
            }
        }
        runners[runner_to_move] += 1;

        for (row, cur_pos) in runners.iter_mut().enumerate() {
            if *cur_pos == len && matrix[row][*cur_pos-1] < min {
                *cur_pos += 1;
            }
        }
    }

    let len = matrix.len();
    let mut result: i32 = matrix[0][0];
    for _ in 0..k {
        step(&matrix, &mut runners, len);
    }
    for (row, col) in runners.iter().enumerate() {
        if *col > 0 && *col <= len {
            result = max(result, matrix[row][*col-1]);
        }
    }

    return result;
}

impl Solver for Solution {
    fn read_inputs() -> Self {
        return Solution {
            input: vec![
                // vec![1,2],
                // vec![1,3],
                // vec![1,5,9],
                // vec![10,11,13],
                // vec![12,13,15],
                vec![1,4,7,11,15],
                vec![2,5,8,12,19],
                vec![3,6,9,16,22],
                vec![10,13,14,17,24],
                vec![18,21,23,26,30],
            ],
            k: 5,
            solution: 0,
        };
    }

    fn solve(mut self) {
        self.solution = kth_smallest(self.input, self.k);
        dbg!(self.solution);
    }
}