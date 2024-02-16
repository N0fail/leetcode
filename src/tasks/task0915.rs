use super::Solver;

#[derive(Debug)]
pub struct Solution {
    nums: Vec<i32>,
    solution: i32,
}

fn partition_disjoint(nums: Vec<i32>) -> i32 {
    let mut maxs: Vec<i32> = nums.clone();
    let mut mins: Vec<i32> = nums.clone();

    for idx in 1..maxs.len() {
        maxs[idx] = maxs[idx].max(maxs[idx - 1]);
    }

    for idx in (0..mins.len() - 1).rev() {
        mins[idx] = mins[idx].min(mins[idx + 1]);
    }

    maxs[..maxs.len() - 1]
        .iter()
        .enumerate()
        .zip(mins[1..].iter())
        .find(|((_idx, l), r)| r >= l)
        .unwrap()
        .0
         .0 as i32
        + 1
}

impl Solver for Solution {
    fn read_inputs() -> Self {
        return Solution {
            nums: vec![1, 1],
            solution: 0,
        };
    }

    fn solve(mut self) {
        self.solution = partition_disjoint(self.nums);
        dbg!(self.solution);
    }
}
