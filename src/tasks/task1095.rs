use super::Solver;

#[derive(Debug)]
pub struct Solution {
    target: i32,
    mountain_arr: MountainArray,
    solution: i32,
}

#[derive(Debug)]
struct MountainArray {
    arr: Vec<i32>,
}
impl MountainArray {
    fn get(&self, index: i32) -> i32 {
        self.arr[index as usize]
    }
    fn length(&self) -> i32 {
        self.arr.len() as i32
    }
}

struct CacheValue {
    idx: i32,
    val: i32,
}

fn find_in_mountain_array(target: i32, mountain_arr: &MountainArray) -> i32 {
    let len = mountain_arr.length();
    let mut left = CacheValue { idx: 1, val: -1 };

    let mut right = CacheValue {
        idx: len - 2,
        val: -1,
    };

    let peak = loop {
        let peak = CacheValue {
            idx: (left.idx + right.idx) / 2,
            val: mountain_arr.get((left.idx + right.idx) / 2),
        };

        if left.idx == right.idx {
            // prevent out of bounds on right check
            break peak;
        }

        let test_left = CacheValue {
            idx: peak.idx - 1,
            val: mountain_arr.get(peak.idx - 1),
        };
        if peak.val < test_left.val {
            right = test_left;
            continue;
        }

        let test_right = CacheValue {
            idx: peak.idx + 1,
            val: mountain_arr.get(peak.idx + 1),
        };
        if peak.val < test_right.val {
            left = test_right;
            continue;
        }

        break peak;
    };

    left = CacheValue {
        idx: 0,
        val: mountain_arr.get(0),
    };
    right = CacheValue {
        idx: peak.idx,
        val: peak.val,
    };

    let ans = loop {
        if left.idx >= right.idx {
            if left.val == target {
                break left;
            }
            break CacheValue { idx: -1, val: -1 };
        }

        let mid = CacheValue {
            idx: (left.idx + right.idx) / 2,
            val: mountain_arr.get((left.idx + right.idx) / 2),
        };

        if mid.val == target {
            break mid;
        }

        if mid.val > target {
            if mid.idx == 0 {
                break CacheValue { idx: -1, val: -1 };
            }
            right = CacheValue {
                idx: mid.idx - 1,
                val: mountain_arr.get(mid.idx - 1),
            };
            continue;
        }

        if mid.val < target {
            left = CacheValue {
                idx: mid.idx + 1,
                val: mountain_arr.get(mid.idx + 1),
            };
            continue;
        }
    };

    if ans.idx != -1 {
        return ans.idx;
    }

    left = CacheValue {
        idx: peak.idx,
        val: peak.val,
    };
    right = CacheValue {
        idx: len - 1,
        val: mountain_arr.get(len - 1),
    };

    let ans = loop {
        if left.idx >= right.idx {
            if left.val == target {
                break left;
            }
            break CacheValue { idx: -1, val: -1 };
        }

        let mid = CacheValue {
            idx: (left.idx + right.idx) / 2,
            val: mountain_arr.get((left.idx + right.idx) / 2),
        };

        if mid.val < target {
            right = CacheValue {
                idx: mid.idx - 1,
                val: mountain_arr.get(mid.idx - 1),
            };
            continue;
        }

        if mid.val > target {
            if mid.idx + 1 >= len {
                break CacheValue { idx: -1, val: -1 };
            }
            left = CacheValue {
                idx: mid.idx + 1,
                val: mountain_arr.get(mid.idx + 1),
            };
            continue;
        }

        break mid;
    };

    return ans.idx;
}

impl Solver for Solution {
    fn read_inputs() -> Self {
        return Solution {
            target: 0,
            mountain_arr: MountainArray { arr: vec![1, 5, 2] },
            solution: 5,
        };
    }

    fn solve(mut self) {
        self.solution = find_in_mountain_array(self.target, &self.mountain_arr);
        dbg!(self.solution);
    }
}
