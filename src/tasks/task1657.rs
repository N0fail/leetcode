use super::Solver;

#[derive(Debug)]
pub struct Solution {
    word1: String,
    word2: String,
    solution: bool,
}

pub fn close_strings(word1: String, word2: String) -> bool {
    if word1.len() != word2.len() {
        return false;
    }

    let mut count1 = [0; 26];
    let mut count2 = [0; 26];

    word1
        .chars()
        .for_each(|ch| count1[ch as usize - 'a' as usize] += 1);
    word2
        .chars()
        .for_each(|ch| count2[ch as usize - 'a' as usize] += 1);

    for i in 0..26 {
        if (count1[i] == 0 && count2[i] != 0) || (count1[i] != 0 && count2[i] == 0) {
            return false;
        }
    }

    count1.sort_unstable();
    count2.sort_unstable();

    count1 == count2
}

impl Solver for Solution {
    fn read_inputs() -> Self {
        return Solution {
            word1: String::from("cabbba"),
            word2: String::from("abbccc"),
            solution: false,
        };
    }

    fn solve(mut self) {
        self.solution = close_strings(self.word1, self.word2);
        dbg!(self.solution);
    }
}
