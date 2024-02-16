use super::Solver;

#[derive(Debug)]
pub struct Solution {
    s: String,
    solution: i32,
}

pub fn first_uniq_char(s: String) -> i32 {
    const NOT_PRESENT: i32 = 1 << 30;
    const DUPLICATE: i32 = 1 << 30 + 1;
    s.chars()
        .map(|c| c as usize - 'a' as usize)
        .enumerate()
        .fold([NOT_PRESENT; 26], |mut array, (idx, c)| {
            array[c] = match array[c] {
                NOT_PRESENT => idx as i32,
                _ => DUPLICATE,
            };
            array
        })
        .iter()
        .copied()
        .filter(|x| *x != NOT_PRESENT && *x != DUPLICATE)
        .min()
        .unwrap_or(-1)
}

impl Solver for Solution {
    fn read_inputs() -> Self {
        return Solution {
            s: "aabb".to_string(),
            solution: -1,
        };
    }

    fn solve(mut self) {
        self.solution = first_uniq_char(self.s);
        dbg!(self.solution);
    }
}
