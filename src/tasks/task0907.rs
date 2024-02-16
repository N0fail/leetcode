use super::Solver;
#[derive(Debug)]
pub struct Solution {
    arr: Vec<i32>,
    solution: i32,
}

const MOD: i64 = 1_000_000_007;
pub fn sum_subarray_mins(arr: Vec<i32>) -> i32 {
    // 10 ms some weird solution
    let mut ll: Vec<usize> = (0..arr.len()).collect(); // at what index left array would start so this element will be last and minimal
    let mut rl: Vec<usize> = (0..arr.len()).collect(); // at what index right array would end so this element will be first and minimal

    for idx in 1..arr.len() {
        let mut next_min_idx = idx - 1;
        loop {
            if arr[idx] >= arr[next_min_idx] {
                ll[idx] = next_min_idx + 1;
                break;
            }
            if next_min_idx == 0 {
                ll[idx] = next_min_idx;
                break;
            }
            if ll[next_min_idx] == 0 {
                ll[idx] = 0;
                break;
            }
            next_min_idx = ll[next_min_idx] - 1;
        }
    }

    for idx in (0..arr.len() - 1).rev() {
        let mut next_max_idx = idx + 1;
        loop {
            if arr[idx] > arr[next_max_idx] {
                rl[idx] = next_max_idx - 1;
                break;
            }
            if next_max_idx == arr.len() - 1 {
                rl[idx] = next_max_idx;
                break;
            }
            if rl[next_max_idx] == arr.len() - 1 {
                rl[idx] = arr.len() - 1;
                break;
            }
            next_max_idx = rl[next_max_idx] + 1;
        }
    }

    let mut res: i64 = 0;
    for idx in 0..arr.len() {
        res += (arr[idx] as i64 * (idx - ll[idx] + 1) as i64 * (rl[idx] - idx + 1) as i64) % MOD
    }

    (res % MOD) as i32
}

// pub fn sum_subarray_mins(arr: Vec<i32>) -> i32 {
//     // todo solution with stack
//     -1
// }

impl Solver for Solution {
    fn read_inputs() -> Self {
        return Solution {
            arr: vec![71, 55, 82, 55],
            solution: -1,
        };
    }

    fn solve(mut self) {
        self.solution = sum_subarray_mins(self.arr);
        dbg!(self.solution);
    }
}
