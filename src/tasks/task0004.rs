use super::Solver;

#[derive(Debug)]
pub struct Solution{
    nums1: Vec<i32>,
    nums2: Vec<i32>,
    solution: f64,
}

pub fn find_median_sorted_arrays(mut nums1: Vec<i32>, mut nums2: Vec<i32>) -> f64 {
    // todo this does not work
    if nums2.len() > nums1.len() {
        (nums1, nums2) = (nums2, nums1)
    }
    let median_index_1 = (nums1.len() + nums2.len() - 1) / 2;
    let median_index_2 = (nums1.len() + nums2.len()) / 2;
    let mut l1 = 0;
    let mut l2 = 0;
    let mut r1 = nums1.len();
    let mut r2 = nums2.len();
    let mut dropped = 0;
    let mut res = 0;

    while dropped < median_index_1 && l1 < r1 && l2 < r2 {
        let max_drop = median_index_1-dropped;
        let mid1 = (l1 + r1 - 1)/2;
        let mid2 = (l2 + r2 - 1)/2;
        if nums1[mid1] > nums2[mid2] {
            let to_drop = (mid2 - l2 + 1).min(max_drop);
            dropped += to_drop;
            l2 += to_drop;
        } else if nums1[mid1] < nums2[mid2] {
            let to_drop = (mid1 - l1 + 1).min(max_drop);
            dropped += to_drop;
            l1 += to_drop;
        } else {
            if nums2[l2] < nums1[l1] {
                l2 += 1;
                dropped += 1;
            } else {
                l1 += 1;
                dropped += 1;
            }
        }
    }

    if dropped > median_index_1 {
        panic!("error")
    }

    if l1 >= r1 {
        res += nums2[l2 + median_index_1 - dropped];
        res += nums2[l2 + median_index_2 - dropped];
        return res as f64 / 2.0
    }

    if l2 >= r2 {
        res += nums1[l1 + median_index_1 - dropped];
        res += nums1[l1 + median_index_2 - dropped];
        return res as f64 / 2.0
    }


    if dropped == median_index_1 {
        if median_index_1 == median_index_2 {
            return nums1[l1].min(nums2[l2]) as f64
        }

        if nums1[l1] < nums2[l2] {
            res += nums1[l1];
            l1 += 1;
            if l1 >= r1 {
                return (res + nums2[l2]) as f64 / 2.0
            }

            return (res + nums1[l1].min(nums2[l2])) as f64 / 2.0
        }

        res += nums2[l2];
        l2 += 1;
        if l2 >= r2 {
            return (res + nums1[l1]) as f64 / 2.0
        }

        return (res + nums1[l1].min(nums2[l2])) as f64 / 2.0
    }
    return res as f64 / 2.0
}

impl Solver for Solution {
    fn read_inputs() -> Self {
        return Solution {
            nums1: vec![2],
            nums2: vec![1,3,4],
            solution: 2.0,
        };
    }

    fn solve(mut self) {
        self.solution = find_median_sorted_arrays(self.nums1, self.nums2);
        dbg!(self.solution);
    }
}
