use super::Solver;

#[derive(Debug)]
pub struct Solution {
    words: Vec<String>,
    solution: bool,
}

pub fn make_equal(words: Vec<String>) -> bool {
    // 2 ms
    words
        .iter()
        .flat_map(|word| word.bytes())
        // .fold([0;26], |counts, b| { this is 8ms, counts is copied on each call
        .fold(&mut [0; 26], |counts, b| {
            counts[(b - b'a') as usize] += 1;
            counts
        })
        .iter()
        .all(|count| count % words.len() == 0)
}

impl Solver for Solution {
    fn read_inputs() -> Self {
        return Solution {
            words: ["abc", "aabc", "bc"]
                .into_iter()
                .map(|x| x.to_string())
                .collect(),
            solution: false,
        };
    }

    fn solve(mut self) {
        self.solution = make_equal(self.words);
        dbg!(self.solution);
    }
}
