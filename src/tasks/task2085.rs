use super::Solver;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Solution {
    words1: Vec<String>,
    words2: Vec<String>,
    solution: i32,
}

impl Solution {
    pub fn count_words(words1: Vec<String>, words2: Vec<String>) -> i32 {
        let count_fn = |words: Vec<String>| {
            words.into_iter().fold(HashMap::new(), |mut res, word| {
                res.entry(word).and_modify(|count| *count += 1).or_insert(1);
                res
            })
        };
        let count1 = count_fn(words1);
        let count2 = count_fn(words2);
        count1
            .into_iter()
            .filter(|(word, count)| *count == 1 && count2.get(word) == Some(&1))
            .count() as _
    }
}

impl Solver for Solution {
    fn read_inputs() -> Self {
        return Solution {
            words1: ["leetcode", "is", "amazing", "as", "is"]
                .into_iter()
                .map(|x| x.to_string())
                .collect(),
            words2: ["amazing", "leetcode", "is"].into_iter().map(|x| x.to_string()).collect(),
            solution: 0,
        };
    }

    fn solve(mut self) {
        self.solution = Solution::count_words(self.words1, self.words2);
        dbg!(self.solution);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ut_1() {
        assert_eq!(
            Solution::count_words(
                ["leetcode", "is", "amazing", "as", "is"]
                    .into_iter()
                    .map(|x| x.to_string())
                    .collect(),
                ["amazing", "leetcode", "is"].into_iter().map(|x| x.to_string()).collect()
            ),
            2
        );
    }

    #[test]
    fn ut_2() {
        assert_eq!(
            Solution::count_words(
                ["b", "bb", "bbb"].into_iter().map(|x| x.to_string()).collect(),
                ["a", "aa", "aaa"].into_iter().map(|x| x.to_string()).collect()
            ),
            0
        );
    }

    #[test]
    fn ut_3() {
        assert_eq!(
            Solution::count_words(
                ["a", "ab"].into_iter().map(|x| x.to_string()).collect(),
                ["a", "a", "a", "ab"].into_iter().map(|x| x.to_string()).collect()
            ),
            1
        );
    }
}
