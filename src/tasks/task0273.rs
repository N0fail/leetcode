use super::Solver;

#[derive(Debug)]
pub struct Solution {
    num: i32,
    solution: String,
}

const NUMBERS: &[(usize, &str)] = &[
    (1, "One"),
    (2, "Two"),
    (3, "Three"),
    (4, "Four"),
    (5, "Five"),
    (6, "Six"),
    (7, "Seven"),
    (8, "Eight"),
    (9, "Nine"),
    (10, "Ten"),
    (11, "Eleven"),
    (12, "Twelve"),
    (13, "Thirteen"),
    (14, "Fourteen"),
    (15, "Fifteen"),
    (16, "Sixteen"),
    (17, "Seventeen"),
    (18, "Eighteen"),
    (19, "Nineteen"),
    (20, "Twenty"),
    (30, "Thirty"),
    (40, "Forty"),
    (50, "Fifty"),
    (60, "Sixty"),
    (70, "Seventy"),
    (80, "Eighty"),
    (90, "Ninety"),
];

const COUNTS: &[(usize, &str)] = &[
    (100, "Hundred"),
    (1000, "Thousand"),
    (1000_000, "Million"),
    (1000_000_000, "Billion"),
];

impl Solution {
    pub fn rec(mut num: usize) -> String {
        let mut res = Vec::new();
        for count in COUNTS.iter().rev() {
            if num / count.0 > 0 {
                res.push(Solution::rec(num / count.0));
                res.push(count.1.to_string());
                if num % count.0 > 0 {
                    res.push(Solution::rec(num % count.0));
                }
                return res.join(" ");
            }
        }

        for number in NUMBERS.iter().rev() {
            if number.0 <= num {
                num -= number.0;
                res.push(number.1.to_string());
            }
        }

        return res.join(" ");
    }

    pub fn number_to_words(num: i32) -> String {
        if num == 0 {
            return "Zero".to_string();
        }
        Self::rec(num as usize)
    }
}

impl Solver for Solution {
    fn read_inputs() -> Self {
        return Solution {
            num: 100,
            solution: "true".to_string(),
        };
    }

    fn solve(mut self) {
        self.solution = Solution::number_to_words(self.num);
        dbg!(self.solution);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ut_1() {
        assert_eq!(Solution::number_to_words(123), String::from("One Hundred Twenty Three"));
    }

    #[test]
    fn ut_2() {
        assert_eq!(
            Solution::number_to_words(12345),
            String::from("Twelve Thousand Three Hundred Forty Five")
        );
    }

    #[test]
    fn ut_3() {
        assert_eq!(
            Solution::number_to_words(1234567),
            String::from("One Million Two Hundred Thirty Four Thousand Five Hundred Sixty Seven")
        );
    }

    #[test]
    fn ut_4() {
        assert_eq!(Solution::number_to_words(100), String::from("One Hundred"));
    }

    #[test]
    fn ut_5() {
        assert_eq!(Solution::number_to_words(101), String::from("One Hundred One"));
    }
}
