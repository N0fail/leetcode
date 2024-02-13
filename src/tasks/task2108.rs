use super::Solver;

#[derive(Debug)]
pub struct Solution{
    words: Vec<String>,
    solution: String,
}

pub fn first_palindrome(words: Vec<String>) -> String {
    words.into_iter()
        .find(|word|
            word.bytes()
                .zip(word.bytes().rev())
                .all(|(a,b)| a==b))
        .unwrap_or("".to_string())
}

impl Solver for Solution {
    fn read_inputs() -> Self {
        return Solution {
            words: ["abc","car","ada","racecar","cool"].map(|x| x.to_string()).into_iter().collect(),
            solution: "".to_string(),
        };
    }

    fn solve(mut self) {
        self.solution = first_palindrome(self.words);
        dbg!(self.solution);
    }
}
