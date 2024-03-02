use super::Solver;

#[derive(Debug)]
pub struct Solution {
    nums: Vec<i32>,
    solution: Vec<i32>,
}

impl Solution {
    pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
        let idx = nums.partition_point(|x| *x < 0);
        let mut res = Vec::with_capacity(nums.len());
        let mut neg = nums[..idx].iter().map(|x| x.abs()).rev();
        let mut pos = nums[idx..].iter().map(|x| x.abs());
        let mut n = neg.next();
        let mut p = pos.next();
        loop {
            match (n, p) {
                (Some(nv), Some(pv)) => {
                    if nv.abs() < pv.abs() {
                        res.push(nv * nv);
                        n = neg.next();
                    } else {
                        res.push(pv * pv);
                        p = pos.next();
                    }
                }
                (Some(nv), None) => {
                    res.push(nv * nv);
                    n = neg.next()
                }
                (None, Some(pv)) => {
                    res.push(pv * pv);
                    p = pos.next()
                }
                (None, None) => break,
            }
        }

        res
    }
}

impl Solver for Solution {
    fn read_inputs() -> Self {
        return Solution {
            nums: vec![-4, -1, 0, 3, 10],
            solution: vec![0, 1, 9, 16, 100],
        };
    }

    fn solve(mut self) {
        self.solution = Solution::sorted_squares(self.nums);
        dbg!(self.solution);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ut_1() {
        assert_eq!(Solution::sorted_squares(vec![-4, -1, 0, 3, 10]), vec![0, 1, 9, 16, 100]);
    }
}
