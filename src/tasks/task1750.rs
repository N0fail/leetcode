use super::Solver;

#[derive(Debug)]
pub struct Solution {
    s: String,
    solution: i32,
}

impl Solution {
    pub fn minimum_length(s: String) -> i32 {
        let s = s.as_bytes();
        let mut l = 0;
        let mut r = s.len() - 1;
        while l < r && s[r] == s[l] {
            let b = s[l];
            loop {
                r -= 1;
                if r <= l || s[r] != b {
                    break;
                }
            }
            loop {
                l += 1;
                if r < l || s[l] != b {
                    break;
                }
            }
        }

        (r + 1 - l) as _
    }
}

impl Solver for Solution {
    fn read_inputs() -> Self {
        return Solution {
            s: "a".to_string(),
            solution: 1,
        };
    }

    fn solve(mut self) {
        self.solution = Solution::minimum_length(self.s);
        dbg!(self.solution);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ut_1() {
        assert_eq!(Solution::minimum_length("ca".to_string()), 2);
    }

    #[test]
    fn ut_2() {
        assert_eq!(Solution::minimum_length("cabaabac".to_string()), 0);
    }

    #[test]
    fn ut_3() {
        assert_eq!(Solution::minimum_length("aabccabba".to_string()), 3);
    }

    #[test]
    fn ut_4() {
        assert_eq!(
            Solution::minimum_length("bbbbbbbbbbbbbbbbbbbbbbbbbbbabbbbbbbbbbbbbbbccbcbcbccbbabbb".to_string()),
            1
        );
    }

    #[test]
    fn ut_5() {
        assert_eq!(Solution::minimum_length("c".to_string()), 1);
    }

    #[test]
    fn ut_6() {
        assert_eq!(Solution::minimum_length("abbbbbbbbbbbbbbbbbbba".to_string()), 0);
    }
}
