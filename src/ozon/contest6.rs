use super::Solver;
use std::fs::read_to_string;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

#[derive(Debug)]
pub struct Solution {
    inputs: Vec<Vec<Vec<u8>>>,
    results: Vec<(usize, usize)>,
}

const MAX: u8 = 5;
fn mask_col(mut matrix: Vec<Vec<u8>>, col_num: usize) -> Vec<Vec<u8>> {
    for row in matrix.iter_mut() {
        row[col_num] += MAX;
    }
    matrix
}

fn unmask_col(mut matrix: Vec<Vec<u8>>, col_num: usize) -> Vec<Vec<u8>> {
    for row in matrix.iter_mut() {
        row[col_num] -= MAX;
    }
    matrix
}

fn mask_row(mut matrix: Vec<Vec<u8>>, row_num: usize) -> Vec<Vec<u8>> {
    for col in 0..matrix[row_num].len() {
        matrix[row_num][col] += MAX;
    }
    matrix
}

fn unmask_row(mut matrix: Vec<Vec<u8>>, row_num: usize) -> Vec<Vec<u8>> {
    for col in 0..matrix[row_num].len() {
        matrix[row_num][col] -= MAX;
    }
    matrix
}

fn find_min(matrix: &Vec<Vec<u8>>) -> (usize, usize) {
    let mut min = (0, 0);
    for (i, row) in matrix.iter().enumerate() {
        for (j, el) in row.iter().enumerate() {
            if *el < matrix[min.0][min.1] {
                min = (i, j)
            }
        }
    }
    min
}

pub fn get_row_col(mut matrix: Vec<Vec<u8>>) -> (usize, usize) {
    let mut best_result = (1, 0, 0);
    let min_coord = find_min(&matrix);
    matrix = mask_row(matrix, min_coord.0);
    let mut min_coord_2 = find_min(&matrix);
    matrix = mask_col(matrix, min_coord_2.1);
    let mut prob_res = find_min(&matrix);
    if matrix[prob_res.0][prob_res.1] > best_result.0 {
        best_result = (matrix[prob_res.0][prob_res.1], min_coord.0, min_coord_2.1)
    }
    matrix = unmask_col(matrix, min_coord_2.1);
    matrix = unmask_row(matrix, min_coord.0);

    matrix = mask_col(matrix, min_coord.1);
    min_coord_2 = find_min(&matrix);
    matrix = mask_row(matrix, min_coord_2.0);
    prob_res = find_min(&matrix);
    if matrix[prob_res.0][prob_res.1] > best_result.0 {
        best_result = (matrix[prob_res.0][prob_res.1], min_coord_2.0, min_coord.1)
    }

    return (best_result.1 + 1, best_result.2 + 1);
}

impl Solver for Solution {
    fn read_inputs() -> Self {
        let mut inputs: Vec<Vec<Vec<u8>>> = Vec::new();
        let mut lines_count: usize = 0;
        let mut matrix: Vec<Vec<u8>> = Vec::new();
        for line in read_to_string(Self::get_input_filename()).unwrap().lines().skip(1) {
            if lines_count == 0 {
                let x = line.split(" ").map(|x| x.parse().unwrap()).collect::<Vec<usize>>();
                lines_count = x[0];
                matrix = Vec::new();
            } else {
                lines_count -= 1;
                matrix.push(line.bytes().into_iter().map(|x| x - b'0').collect());
                if lines_count == 0 {
                    inputs.push(matrix.clone());
                }
            }
        }

        return Solution { inputs, results: vec![] };
    }

    fn solve(&mut self) {
        for input in &self.inputs {
            self.results.push(get_row_col(input.clone()));
        }
    }

    fn write_output(&self) {
        let path = Path::new(Self::get_output_filename());
        let display = path.display();

        // Open a file in write-only mode, returns `io::Result<File>`
        let mut output_file = match File::create(&path) {
            Err(why) => panic!("couldn't create {}: {}", display, why),
            Ok(file) => file,
        };

        for result in &self.results {
            output_file.write(format!("{} {}\n", result.0, result.1).as_bytes()).unwrap();
        }
    }

    fn dump_example(&self, idx: usize) -> String {
        return idx.to_string();
    }
}
