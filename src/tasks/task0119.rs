use super::Solver;

#[derive(Debug)]
pub struct Solution {
    row_index: i32,
    solution: Vec<i32>,
}

pub fn get_row(row_index: i32) -> Vec<i32> {
    let row_index = row_index as usize;
    let mut prev_row = vec![1];
    for row_num in 1..row_index + 1 {
        let mut new_row = Vec::with_capacity(row_num);
        new_row.push(1);
        prev_row
            .windows(2)
            .for_each(|lr| new_row.push(lr[0] + lr[1]));
        new_row.push(1);
        prev_row = new_row;
    }

    prev_row
}

impl Solver for Solution {
    fn read_inputs() -> Self {
        return Solution {
            row_index: 3,
            solution: vec![],
        };
    }

    fn solve(mut self) {
        self.solution = get_row(self.row_index);
        dbg!(self.solution);
    }
}
