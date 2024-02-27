use super::Solver;

#[derive(Debug)]
pub struct Solution {
    nums: Vec<i32>,
    k: i32,
    solution: i32,
}

impl Solution {
    pub fn max_frequency(mut nums: Vec<i32>, k: i32) -> i32 {
        nums.sort_unstable();
        let k = k as i64;
        let mut l = 0;
        let mut sum = 0;
        let mut res = 1;
        for r in 1..nums.len() {
            let diff = nums[r] - nums[r - 1];
            sum += diff as i64 * (r - l) as i64;
            while sum > k {
                sum -= (nums[r] - nums[l]) as i64;
                l += 1;
            }
            res = res.max(r - l + 1);
        }
        res as _
    }
}

impl Solver for Solution {
    fn read_inputs() -> Self {
        return Solution {
            nums: vec![1, 2, 4],
            k: 5,
            solution: 3,
        };
    }

    fn solve(mut self) {
        self.solution = Solution::max_frequency(self.nums, self.k);
        dbg!(self.solution);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ut_1() {
        assert_eq!(Solution::max_frequency(vec![1, 2, 4], 5), 3);
    }

    #[test]
    fn ut_2() {
        assert_eq!(Solution::max_frequency(vec![1, 4, 8, 13], 5), 2);
    }

    #[test]
    fn ut_3() {
        assert_eq!(Solution::max_frequency(vec![3, 9, 6], 2), 1);
    }

    #[test]
    fn ut_4() {
        let mut x = vec![1; 29999];
        x.push(100000);
        assert_eq!(Solution::max_frequency(x, 1), 29999);
    }
}
