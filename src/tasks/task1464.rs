use super::Solver;

#[derive(Debug)]
pub struct Solution {
    nums: Vec<i32>,
    solution: i32,
}

impl Solution {
    // pub fn max_product(nums: Vec<i32>) -> i32 {
    //     // 2 pass 1ms
    //     let (max, maxi) = nums.iter().zip(0..nums.len()).max().unwrap();
    //     (nums.iter().take(maxi).chain(nums.iter().skip(maxi + 1)).max().unwrap() - 1) * (max - 1)
    // }

    pub fn max_product(nums: Vec<i32>) -> i32 {
        // 1 pass 1ms
        nums.into_iter()
            .fold([0, 0], |res, num| {
                if num - 1 >= res[0] {
                    [res[1].min(num - 1), res[1].max(num - 1)]
                } else {
                    res
                }
            })
            .into_iter()
            .reduce(|acc, x| acc * x)
            .unwrap()
    }

    // pub fn max_product(mut nums: Vec<i32>) -> i32 {
    //     // sort 1ms
    //     nums.sort_unstable_by_key(|n| -n);
    //     (nums[0]-1) * (nums[1] - 1)
    // }

    // pub fn max_product(nums: Vec<i32>) -> i32 {
    //     // heap 1ms
    //     let mut heap = BinaryHeap::from(nums);
    //     (heap.pop().unwrap() - 1) * (heap.pop().unwrap() - 1)
    // }
}

impl Solver for Solution {
    fn read_inputs() -> Self {
        return Solution {
            nums: vec![3, 4, 5, 2],
            solution: 12,
        };
    }

    fn solve(mut self) {
        self.solution = Solution::max_product(self.nums);
        dbg!(self.solution);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ut_1() {
        assert_eq!(Solution::max_product(vec![3, 4, 5, 2]), 12);
    }

    #[test]
    fn ut_2() {
        assert_eq!(Solution::max_product(vec![1, 5, 4, 5]), 16);
    }

    #[test]
    fn ut_3() {
        assert_eq!(Solution::max_product(vec![3, 7]), 12);
    }

    #[test]
    fn ut_4() {
        assert_eq!(Solution::max_product(vec![10, 2, 5, 2]), 36);
    }
}
