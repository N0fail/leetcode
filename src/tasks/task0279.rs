use super::Solver;
#[derive(Debug)]
pub struct Solution{
    n: i32,
    solution: i32,
}


pub fn num_squares(n: i32) -> i32 {
    let n = n as usize;
    let mut dp = (1..n+1).collect::<Vec<_>>(); // sum of 1's as baseline
    for i in 2..(n as f32).sqrt() as usize + 1 {
        let x = i * i;
        dp[x-1] = 1; // perfect square can not be better
        for j in 0..n-x {
            dp[j+x] = dp[j+x].min(dp[j] + 1)
        }
    };

    dp[n-1] as i32
}

impl Solver for Solution {
    fn read_inputs() -> Self {
        return Solution {
            n: 17,
            solution: -1,
        };
    }

    fn solve(mut self) {
        self.solution = num_squares(self.n);
        dbg!(self.solution);
    }
}
