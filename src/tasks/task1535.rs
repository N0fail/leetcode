use super::Solver;

#[derive(Debug)]
pub struct Solution {
    arr: Vec<i32>,
    k: i32,
    solution: i32,
}

pub fn get_winner(arr: Vec<i32>, k: i32) -> i32 {
    let mut winner: (i32, i32) = (arr[0], 0);
    for el in &arr[1..] {
        if winner.1 >= k {
            break;
        }
        if winner.0 > *el {
            winner.1 += 1
        } else {
            winner = (*el, 1)
        }
    }
    winner.0
}

impl Solver for Solution {
    fn read_inputs() -> Self {
        return Solution {
            arr: vec![3, 2, 1],
            k: 10,
            solution: -1,
        };
    }

    fn solve(mut self) {
        self.solution = get_winner(self.arr, self.k);
        dbg!(self.solution);
    }
}
