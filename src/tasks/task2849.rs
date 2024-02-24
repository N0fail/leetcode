use super::Solver;
#[derive(Debug)]
pub struct Solution {
    sx: i32,
    sy: i32,
    fx: i32,
    fy: i32,
    t: i32,
    solution: bool,
}
pub fn is_reachable_at_time(sx: i32, sy: i32, fx: i32, fy: i32, t: i32) -> bool {
    let min_dist = (sx - fx).abs().max((sy - fy).abs());
    if min_dist == 0 && t == 1 {
        return false;
    }
    min_dist <= t
}

impl Solver for Solution {
    fn read_inputs() -> Self {
        return Solution {
            sx: 2,
            sy: 2,
            fx: 7,
            fy: 7,
            t: 6,
            solution: false,
        };
    }

    fn solve(mut self) {
        self.solution = is_reachable_at_time(self.sx, self.sy, self.fx, self.fy, self.t);
        dbg!(self.solution);
    }
}
