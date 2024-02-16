use super::Solver;
use std::cmp::min;

#[derive(Debug)]
pub struct Solution {
    input: Vec<Vec<i32>>,
    solution: Vec<Vec<i32>>,
}

fn update_matrix(mat: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut result = mat.clone();

    const VISITED: i32 = 1 << 29;
    let mut queue_head = 0;
    let mut queue_tail = 0;
    let mut queue = vec![(0, 0); result.len() * result[0].len() * 4];
    for (i, row) in mat.iter().enumerate() {
        for (j, &el) in row.iter().enumerate() {
            if el == 0 {
                queue[queue_tail] = (i, j);
                queue_tail += 1;
            }
        }
    }
    let neighbors: [(isize, isize); 4] = [(-1, 0), (0, -1), (0, 1), (1, 0)];

    while queue_tail > queue_head {
        let current = queue[queue_head];
        queue_head += 1;

        if (result[current.0][current.1] & VISITED) != 0 {
            continue;
        }

        let mut min_distance = 1 << 30;
        for (di, dj) in neighbors.iter() {
            let new_i = current.0 as isize + *di;
            if new_i < 0 {
                continue;
            }

            let new_j = current.1 as isize + *dj;
            if new_j < 0 {
                continue;
            }

            let i = new_i as usize;
            if i >= result.len() {
                continue;
            }

            let j = new_j as usize;
            if j >= result[0].len() {
                continue;
            }

            if result[i][j] & VISITED == 0 {
                queue[queue_tail] = (i, j);
                queue_tail += 1;
            } else {
                min_distance = min(result[i][j], min_distance);
            }
        }
        if mat[current.0][current.1] == 0 {
            result[current.0][current.1] = 0 | VISITED;
        } else {
            result[current.0][current.1] = (min_distance + 1) | VISITED;
        }
    }

    for row in &mut result {
        for j in row {
            *j = *j & !VISITED;
        }
    }

    return result;
}

impl Solver for Solution {
    fn read_inputs() -> Self {
        return Solution {
            input: vec![
                vec![0, 1, 0, 1, 1],
                vec![1, 1, 0, 0, 1],
                vec![0, 0, 0, 1, 0],
                vec![1, 0, 1, 1, 1],
                vec![1, 0, 0, 0, 1],
            ],
            solution: vec![],
        };
    }

    fn solve(mut self) {
        self.solution = update_matrix(&self.input);
        dbg!(&self);
    }
}
