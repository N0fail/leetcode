use super::Solver;
#[derive(Debug)]
pub struct Solution{
    arr: Vec<i32>,
    solution: i32,
}

pub fn maximum_element_after_decrementing_and_rearranging(mut arr: Vec<i32>) -> i32 {
    arr.sort_unstable();
    arr.into_iter().fold(0, |res, el| (res+1).min(el))
}

impl Solver for Solution {
    fn read_inputs() -> Self {
        return Solution {
            arr: vec![73,98,9],
            solution: -1,
        };
    }

    fn solve(mut self) {
        self.solution = maximum_element_after_decrementing_and_rearranging(self.arr);
        dbg!(self.solution);
    }
}
