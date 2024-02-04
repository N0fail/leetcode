use super::Solver;

#[derive(Debug)]
pub struct Solution{
    arr: Vec<i32>,
    solution: i32,
}


pub fn longest_mountain(arr: Vec<i32>) -> i32 {
    let mut left = 0;
    let n = arr.len();
    let mut result = 0;

    loop {
        while left + 2 < n && arr[left] >= arr[left+1] {
            left += 1;
        }

        let mut peak = left;
        while peak + 1 < n && arr[peak] < arr[peak+1] {
            peak += 1;
        }

        let mut right = peak;
        while right + 1 < n && arr[right] > arr[right+1] {
            right += 1
        }

        if right > peak {
            result = result.max(right - left + 1);
        }
        left = right;

        if left + 2 >= n {
            break
        }
    }

    if result < 3 {
        return 0
    }

    return result as i32
}

impl Solver for Solution {
    fn read_inputs() -> Self {
        return Solution {
            arr: vec![4,3,4,3,2],
            solution: 0,
        };
    }

    fn solve(mut self) {
        self.solution = longest_mountain(self.arr);
        dbg!(self.solution);
    }
}
