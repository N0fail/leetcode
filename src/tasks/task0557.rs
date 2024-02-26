use super::Solver;

#[derive(Debug)]
pub struct Solution {
    s: String,
    solution: String,
}

impl Solution {
    // pub fn reverse_words(s: String) -> String {
    //     // 3ms
    //     s.split(" ")
    //         .map(|x| x.chars().rev().chain(" ".chars()))
    //         .flatten()
    //         .take(s.len())
    //         .collect()
    // }
    pub fn reverse_words(s: String) -> String {
        // 0ms
        s.chars()
            .rev()
            .collect::<String>()
            .split_whitespace()
            .rev()
            .collect::<Vec<_>>()
            .join(" ")
    }
    // pub fn reverse_words(s: String) -> String {
    //     // 3ms
    //     s.split(" ")
    //             .map(|x| x.chars().rev().collect::<String>())
    //             .collect::<Vec<_>>()
    //             .join(" ")
    // }
    // pub fn reverse_words(s: String) -> String {
    //     // 3ms
    //     s.split(" ")
    //         .map(|x| x.chars().rev().collect::<String>())
    //         .collect::<Vec<_>>()
    //         .join(" ")
    // }
}

impl Solver for Solution {
    fn read_inputs() -> Self {
        return Solution {
            s: "Let's take LeetCode contest".to_string(),
            solution: "true".to_string(),
        };
    }

    fn solve(mut self) {
        self.solution = Solution::reverse_words(self.s);
        dbg!(self.solution);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ut_1() {
        let s = String::from("God Ding");
        assert_eq!(Solution::reverse_words(s), String::from("doG gniD"));
    }

    #[test]
    fn ut_2() {
        let s = String::from("Let's take LeetCode contest");
        assert_eq!(Solution::reverse_words(s), String::from("s'teL ekat edoCteeL tsetnoc"));
    }
}
