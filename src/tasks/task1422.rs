use super::Solver;

#[derive(Debug)]
pub struct Solution {
    s: String,
    solution: i32,
}

impl Solution {
    // pub fn max_score(s: String) -> i32 {
    //     // O(n) mem
    //     let mut zeros_count = Vec::with_capacity(s.len());
    //     s.bytes().fold(&mut zeros_count, |zeros_count, b| {
    //         let last: i32 = zeros_count.last().copied().unwrap_or(0);
    //         if b == b'0' {
    //             zeros_count.push(last + 1);
    //         } else {
    //             zeros_count.push(last);
    //         }
    //         zeros_count
    //     });
    //     let total_zeros = zeros_count.last().copied().unwrap();
    //     zeros_count[..zeros_count.len() - 1]
    //         .iter()
    //         .enumerate()
    //         .map(|(idx, count)| {
    //             let zeros_on_the_right = total_zeros - *count;
    //             let digits_on_the_right = s.len() - idx - 1;
    //             let ones_on_the_right = digits_on_the_right as i32 - zeros_on_the_right;
    //             *count + ones_on_the_right
    //         })
    //         .max()
    //         .unwrap()
    // }

    pub fn max_score(s: String) -> i32 {
        // O(1) mem
        let total_ones = s.bytes().filter(|b| *b == b'1').count() as i32;
        let mut ones_on_the_left = 0;
        let mut zeros_on_the_left = 0;
        s[..s.len() - 1]
            .bytes()
            .map(|b| {
                if b == b'1' {
                    ones_on_the_left += 1;
                } else {
                    zeros_on_the_left += 1;
                }
                let ones_on_the_right = total_ones - ones_on_the_left;
                zeros_on_the_left + ones_on_the_right
            })
            .max()
            .unwrap()
    }
}

impl Solver for Solution {
    fn read_inputs() -> Self {
        return Solution {
            s: "00".to_string(),
            solution: 1,
        };
    }

    fn solve(mut self) {
        self.solution = Solution::max_score(self.s);
        dbg!(self.solution);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ut_1() {
        assert_eq!(Solution::max_score("011101".to_string()), 5);
    }

    #[test]
    fn ut_2() {
        assert_eq!(Solution::max_score("00111".to_string()), 5);
    }

    #[test]
    fn ut_3() {
        assert_eq!(Solution::max_score("1111".to_string()), 3);
    }

    #[test]
    fn ut_4() {
        assert_eq!(Solution::max_score("00".to_string()), 1);
    }
}
