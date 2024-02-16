use super::Solver;

#[derive(Debug)]
pub struct Solution {
    people: Vec<i32>,
    limit: i32,
    solution: i32,
}

pub fn num_rescue_boats(mut people: Vec<i32>, limit: i32) -> i32 {
    people.sort();
    let mut l = 0;
    let mut r = people.len() - 1;
    let mut res = 0;
    while l < r {
        res += 1;
        if people[l] > limit / 2 {
            return (res + r - l) as i32;
        }
        if people[l] + people[r] <= limit {
            l += 1;
        }
        r -= 1;
    }
    if l == r {
        res += 1
    }
    return res as i32;
}

impl Solver for Solution {
    fn read_inputs() -> Self {
        return Solution {
            people: vec![3, 2, 1, 1],
            limit: 3,
            solution: 0,
        };
    }

    fn solve(mut self) {
        self.solution = num_rescue_boats(self.people, self.limit);
        dbg!(self.solution);
    }
}
