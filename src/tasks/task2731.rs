use super::Solver;

#[derive(Debug)]
pub struct Solution {
    nums: Vec<i32>,
    s: String,
    d: i32,
    solution: i32,
}

const MOD: i64 = 1_000_000_000 + 7;
pub fn sum_distance(nums: Vec<i32>, s: String, d: i32) -> i32 {
    let mut end_positions: Vec<i64> = vec![];
    end_positions.reserve(nums.len());
    for (i, c) in s.chars().enumerate() {
        let num = nums[i] as i64;
        if c == 'R' {
            end_positions.push(num + d as i64)
        } else {
            end_positions.push(num - d as i64)
        }
    }
    end_positions.sort();
    let mut res: i64 = 0;
    let mut prev_pos = end_positions[0];
    for pos in &end_positions[1..] {
        res += *pos - prev_pos;
    }
    let mut prev_dist = res;
    res %= MOD;
    for (i, pos) in end_positions[1..].iter().enumerate() {
        prev_dist = prev_dist - (*pos - prev_pos) * ((end_positions.len() - 1 - i) as i64);
        prev_pos = *pos;
        res += prev_dist;
        res %= MOD;
    }

    return res as i32;
}

impl Solver for Solution {
    fn read_inputs() -> Self {
        return Solution {
            nums: vec![-2, 0, 2],
            s: String::from("RLL"),
            d: 3,
            solution: 0,
        };
    }

    fn solve(mut self) {
        self.solution = sum_distance(self.nums, self.s, self.d);
        dbg!(self.solution);
    }
}
