use super::Solver;

#[derive(Debug)]
pub struct Solution {
    tokens: Vec<i32>,
    power: i32,
    solution: i32,
}

impl Solution {
    pub fn bag_of_tokens_score(mut tokens: Vec<i32>, mut power: i32) -> i32 {
        if tokens.is_empty() {
            return 0;
        }
        tokens.sort_unstable();
        let mut res = 0;
        let mut l = 0;
        let mut r = tokens.len() - 1;
        loop {
            if power < tokens[l] {
                break;
            }
            while l <= r && power >= tokens[l] {
                power -= tokens[l];
                l += 1;
                res += 1;
            }
            if l + 1 >= r {
                break;
            }
            power += tokens[r];
            r -= 1;
            res -= 1;
        }
        res
    }
}

impl Solver for Solution {
    fn read_inputs() -> Self {
        return Solution {
            tokens: vec![100, 200, 300, 400],
            power: 200,
            solution: 2,
        };
    }

    fn solve(mut self) {
        self.solution = Solution::bag_of_tokens_score(self.tokens, self.power);
        dbg!(self.solution);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ut_1() {
        assert_eq!(Solution::bag_of_tokens_score(vec![100, 200, 300, 400], 200), 2);
    }

    #[test]
    fn ut_2() {
        assert_eq!(Solution::bag_of_tokens_score(vec![200, 100], 150), 1);
    }

    #[test]
    fn ut_3() {
        assert_eq!(Solution::bag_of_tokens_score(vec![100], 50), 0);
    }

    #[test]
    fn ut_4() {
        assert_eq!(Solution::bag_of_tokens_score(vec![], 50), 0);
    }

    #[test]
    fn ut_5() {
        assert_eq!(Solution::bag_of_tokens_score(vec![26], 51), 1);
    }
}
