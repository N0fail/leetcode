use super::Solver;

#[derive(Debug)]
pub struct Solution{
    n: i32,
    min_profit: i32,
    group: Vec<i32>,
    profit: Vec<i32>,
    solution: i32,
}

pub fn profitable_schemes(n: i32, min_profit: i32, group: Vec<i32>, profit: Vec<i32>) -> i32 {
    const MOD:i32 = 1_000_000_000 + 7;
    let mut dp = [[0; 102];102];
    dp[0][0] = 1;
    let n = n as usize;
    let min_profit = min_profit as usize;

    profit.into_iter()
        .zip(group.into_iter())
        .for_each(|(profit, people)| {
            for pr in (0..min_profit+1).rev() {
                for ppl in (0..n+1).rev() {
                    dp[(pr+profit as usize).min(min_profit)][(ppl+people as usize).min(n+1)] += dp[pr][ppl] % MOD;
                    dp[(pr+profit as usize).min(min_profit)][(ppl+people as usize).min(n+1)] %= MOD
                }
            }
        });

    dp[min_profit][..n+1].iter().fold(0, |acc, x| (acc + *x) % MOD)
}

impl Solver for Solution {
    fn read_inputs() -> Self {
        return Solution {
            n: 100,
            min_profit: 100,
            group: vec![2,5,36,2,5,5,14,1,12,1,14,15,1,1,27,13,6,59,6,1,7,1,2,7,6,1,6,1,3,1,2,11,3,39,21,20,1,27,26,22,11,17,3,2,4,5,6,18,4,14,1,1,1,3,12,9,7,3,16,5,1,19,4,8,6,3,2,7,3,5,12,6,15,2,11,12,12,21,5,1,13,2,29,38,10,17,1,14,1,62,7,1,14,6,4,16,6,4,32,48],
            profit: vec![21,4,9,12,5,8,8,5,14,18,43,24,3,0,20,9,0,24,4,0,0,7,3,13,6,5,19,6,3,14,9,5,5,6,4,7,20,2,13,0,1,19,4,0,11,9,6,15,15,7,1,25,17,4,4,3,43,46,82,15,12,4,1,8,24,3,15,3,6,3,0,8,10,8,10,1,21,13,10,28,11,27,17,1,13,10,11,4,36,26,4,2,2,2,10,0,11,5,22,6],
            solution: 0,

            // n: 10,
            // min_profit: 5,
            // group: vec![2,3,5],
            // profit: vec![6,7,8],
            // solution: 0,
        };
    }

    fn solve(mut self) {
        self.solution = profitable_schemes(self.n, self.min_profit, self.group, self.profit);
        dbg!(self.solution);
    }
}
