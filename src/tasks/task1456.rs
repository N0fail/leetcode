use super::Solver;
#[derive(Debug)]
pub struct Solution {
    s: String,
    k: i32,
    solution: i32,
}

pub fn max_vowels(s: String, k: i32) -> i32 {
    let k = k as usize;
    let x = "aeiou".chars();
    let is_vowels = |c| x.clone().any(|el| el == c);
    let mut best = s[..k].chars().filter(|c| is_vowels(*c)).count();
    let mut cur = best;
    s[..s.len() - k]
        .chars()
        .zip(s[k..].chars())
        .for_each(|(l_char, r_char)| {
            if is_vowels(l_char) {
                cur -= 1;
            }
            if is_vowels(r_char) {
                cur += 1;
            }
            best = best.max(cur)
        });
    best as i32
}

impl Solver for Solution {
    fn read_inputs() -> Self {
        return Solution {
            s: "leetcode".to_string(),
            k: 3,
            solution: -1,
        };
    }

    fn solve(mut self) {
        self.solution = max_vowels(self.s, self.k);
        dbg!(self.solution);
    }
}
