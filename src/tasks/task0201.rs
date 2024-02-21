use super::Solver;

#[derive(Debug)]
pub struct Solution {
    left: i32,
    right: i32,
    solution: i32,
}

pub fn range_bitwise_and(left: i32, right: i32) -> i32 {
    (0..30)
        .try_rfold(0, |res, i| {
            let next = 1 << i;
            let and = left & next;
            if and == (right & next) {
                Ok(res + and)
            } else {
                Err(res)
            }
        })
        .unwrap_or_else(|res| res)
}

impl Solver for Solution {
    fn read_inputs() -> Self {
        return Solution {
            left: 11,
            right: 11,
            solution: 4,
        };
    }

    fn solve(mut self) {
        self.solution = range_bitwise_and(self.left, self.right);
        dbg!(self.solution);
    }
}
