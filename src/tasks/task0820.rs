use super::Solver;

#[derive(Debug)]
pub struct Solution {
    words: Vec<String>,
    solution: i32,
}

pub fn minimum_length_encoding(words: Vec<String>) -> i32 {
    let mut rev_words: Vec<String> = Vec::with_capacity(words.len());
    for word in words {
        rev_words.push(word.chars().into_iter().rev().collect())
    }
    rev_words.sort();

    rev_words
        .windows(2)
        .fold(rev_words[0].len() + 1, |res, lr| {
            if lr[1].starts_with(&lr[0]) {
                return res - lr[0].len() + lr[1].len();
            }
            return res + lr[1].len() + 1;
        }) as i32
}

impl Solver for Solution {
    fn read_inputs() -> Self {
        return Solution {
            // words: vec!["time".to_string(), "me".to_string(), "bell".to_string()],
            // words: vec!["t".to_string()],
            // words: vec!["me".to_string(), "time".to_string()],
            words: vec!["p".to_string(), "grah".to_string(), "qwosp".to_string()],
            solution: 0,
        };
    }

    fn solve(mut self) {
        self.solution = minimum_length_encoding(self.words);
        dbg!(self.solution);
    }
}
