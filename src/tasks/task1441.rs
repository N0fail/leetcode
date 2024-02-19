use super::Solver;

#[derive(Debug)]
pub struct Solution {
    target: Vec<i32>,
    n: i32,
    solution: Vec<String>,
}

pub fn build_array(target: Vec<i32>, _: i32) -> Vec<String> {
    let push = "Push".to_string();
    let pop = "Pop".to_string();
    let mut res = vec![];
    let mut top = 1;
    for num in target {
        for _ in top..num {
            res.push(push.clone());
            res.push(pop.clone());
        }
        res.push(push.clone());
        top = num + 1;
    }

    res
}

impl Solver for Solution {
    fn read_inputs() -> Self {
        return Solution {
            target: vec![1, 3],
            n: 3,
            solution: vec![],
        };
    }

    fn solve(mut self) {
        self.solution = build_array(self.target, self.n);
        dbg!(self.solution);
    }
}
