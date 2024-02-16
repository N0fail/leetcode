use super::Solver;

#[derive(Debug)]
pub struct Solution {
    arr: Vec<i32>,
    target: i32,
    solution: i32,
}

pub fn min_sum_of_lengths(arr: Vec<i32>, target: i32) -> i32 {
    let n = arr.len();
    let mut pref = vec![n; n];
    let mut suff = vec![n; n];

    let mut l: usize = 0;
    let mut r: usize = 0;
    let mut sum = 0;

    while r < n {
        sum += arr[r];
        while sum > target {
            sum -= arr[l];
            l += 1
        }
        if sum == target {
            pref[r] = (r - l + 1).min(pref[r.max(1) - 1])
        } else {
            pref[r] = pref[r.max(1) - 1];
        }
        r += 1
    }

    r = n - 1;
    l = n - 1;
    sum = 0;
    while l > 0 {
        sum += arr[l];
        while sum > target {
            sum -= arr[r];
            r -= 1
        }
        if sum == target {
            suff[l - 1] = (r - l + 1).min(suff[l.min(n - 1)])
        } else {
            suff[l - 1] = suff[l.min(n - 1)];
        }
        l -= 1
    }

    let res = pref
        .into_iter()
        .zip(suff.into_iter())
        .map(|(p, s)| p + s)
        .min()
        .unwrap();

    if res > n {
        return -1;
    }
    res as i32
}

impl Solver for Solution {
    fn read_inputs() -> Self {
        return Solution {
            arr: vec![64, 5, 20, 9, 1, 39],
            target: 69,
            solution: -1,
        };
    }

    fn solve(mut self) {
        self.solution = min_sum_of_lengths(self.arr, self.target);
        dbg!(self.solution);
    }
}
