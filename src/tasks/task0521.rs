use super::Solver;
#[derive(Debug)]
pub struct Solution{
    a: String,
    b: String,
    solution: i32,
}

pub fn find_lu_slength(a: String, b: String) -> i32 {
    let a_len = a.len() as i32;
    let b_len = b.len() as i32;

    if a_len > b_len {
        return a_len;
    }

    if b_len > a_len {
        return b_len;
    }

    if a == b {
        return -1;
    }

    return a_len;
}

impl Solver for Solution {
    fn read_inputs() -> Self {
        return Solution {
            a: String::from("aaa"),
            b: String::from("aaa"),
            solution: 0,
        };
    }

    fn solve(mut self) {
        self.solution = find_lu_slength(self.a, self.b);
        println!("{}",self.solution);
    }
}