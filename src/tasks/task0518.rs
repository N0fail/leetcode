use super::Solver;
#[derive(Debug)]
pub struct Solution {
    coins: Vec<i32>,
    amount: i32,
    solution: i32,
}

pub fn change(amount: i32, coins: Vec<i32>) -> i32 {
    let u_amount = amount as usize;
    let mut possibles = vec![0; u_amount + 1];
    possibles[0] = 1;
    for coin in coins {
        let u_coin = coin as usize;
        for amount in u_coin..=u_amount {
            possibles[amount] += possibles[amount - u_coin];
        }
    }

    return possibles[u_amount];
}

impl Solver for Solution {
    fn read_inputs() -> Self {
        return Solution {
            coins: vec![10],
            amount: 10,
            solution: 0,
        };
    }

    fn solve(mut self) {
        self.solution = change(self.amount, self.coins);
        self.coins = vec![];
        dbg!(&self);
    }
}
