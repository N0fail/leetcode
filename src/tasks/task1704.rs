use super::Solver;
#[derive(Debug)]
pub struct Solution {
    s: String,
    solution: bool,
}

pub fn halves_are_alike(s: String) -> bool {
    let s = s.as_bytes();
    let vowels = [b'a', b'e', b'i', b'o', b'u', b'A', b'E', b'I', b'O', b'U'];
    let is_vowels = |b: &u8| vowels.iter().any(|v| v == b);
    s.iter().take(s.len() / 2).filter(|b| is_vowels(b)).count() == s.iter().skip(s.len() / 2).filter(|b| is_vowels(b)).count()
}

impl Solver for Solution {
    fn read_inputs() -> Self {
        return Solution {
            s: "book".to_string(),
            solution: false,
        };
    }

    fn solve(mut self) {
        self.solution = halves_are_alike(self.s);
        dbg!(self.solution);
    }
}
