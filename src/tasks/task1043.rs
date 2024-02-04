use super::Solver;

#[derive(Debug)]
pub struct Solution{
    arr: Vec<i32>,
    k: i32,
    solution: i32,
}

pub fn max_sum_after_partitioning(arr: Vec<i32>, k: i32) -> i32 {
    let n = arr.len();
    let mut dp: Vec<i32> = vec![0; n+1];
    let ku = k as usize;
    for i in 0..n {
        let mut max_el = 0;
        let mut max_res = 0;
        for j in 0..ku.min(i + 1){
            max_el = max_el.max(arr[i-j]);
            max_res = max_res.max(dp[i-j] + max_el * (j + 1) as i32);
        }
        dp[i+1] = max_res;
    }

    return dp[n]
}

impl Solver for Solution {
    fn read_inputs() -> Self {
        return Solution {
            arr: vec![1,4,1,5,7,3,6,1,9,9,3],
            k: 4,
            solution: 0,
        };
    }

    fn solve(mut self) {
        self.solution = max_sum_after_partitioning(self.arr, self.k);
        dbg!(self.solution);
    }
}

