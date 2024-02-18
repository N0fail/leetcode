use super::Solver;

#[derive(Debug)]
pub struct Solution {
    s: String,
    solution: i32,
}

pub fn count_palindromic_subsequence(s: String) -> i32 {
    // 7ms 3.85mb
    let bytes = s.as_bytes();
    let mut indexes = vec![vec![]; 26];
    let get_idx = |b: &u8| (b - b'a') as usize;
    for (idx, b) in bytes.iter().enumerate() {
        indexes[get_idx(b)].push(idx);
    }
    let mut res = 0;
    for edges_idx in &indexes {
        if edges_idx.len() < 2 {
            continue;
        }
        let from = edges_idx[0];
        let to = edges_idx[edges_idx.len() - 1];
        for center_idx in &indexes {
            let cut_from = center_idx.partition_point(|idx| idx <= &from);
            let cut_to = center_idx.partition_point(|idx| idx < &to);
            if cut_to > cut_from {
                res += 1;
            }
        }
    }

    res
}

// pub fn count_palindromic_subsequence(s: String) -> i32 {
//     // 9ms 2.44mb
//     let bytes = s.as_bytes();
//     let get_idx = |b: &u8| (b-b'a') as usize;
//     let mut have_to_the_left_of_most_right= [[-1; 26];26];
//     let mut have_to_the_left_of_most_left= [[-1; 26];26];
//     let mut seen = [0; 26];
//     for b in bytes {
//         let idx = get_idx(b);
//         have_to_the_left_of_most_right[idx] = seen;
//         if have_to_the_left_of_most_left[idx][0] == -1 {
//             have_to_the_left_of_most_left[idx] = seen;
//         }
//         seen[idx] += 1;
//     }
//
//     let mut res = 0;
//     for idx in 0..26 {
//         have_to_the_left_of_most_right[idx].iter().zip(have_to_the_left_of_most_left[idx].iter()).enumerate().for_each(|(i, (right, left))|{
//             if i == idx {
//                 if right - left > 1 {
//                     res += 1;
//                 }
//             } else if right - left > 0 {
//                 res += 1;
//             }
//         });
//     }
//
//     res
// }

// pub fn count_palindromic_subsequence(s: String) -> i32 {
//     // 3ms 2.29mb
//     let bytes = s.as_bytes();
//     let get_idx = |b: &u8| (b-b'a') as usize;
//     let mut have_to_the_left_of_most_right= [[-1; 26];26];
//     let mut have_to_the_left_of_most_left= [[-1; 26];26];
//     let mut most_right_idx_of_b = [0;26];
//     for (idx, b) in bytes.iter().enumerate().rev() {
//         let b_idx = get_idx(b);
//         if most_right_idx_of_b[b_idx] == 0 {
//             most_right_idx_of_b[b_idx] = idx;
//         }
//     }
//     let mut seen = [0; 26];
//     for (idx, b) in bytes.iter().enumerate() {
//         let b_idx = get_idx(b);
//         if idx == most_right_idx_of_b[b_idx] {
//             have_to_the_left_of_most_right[b_idx] = seen;
//         }
//         if have_to_the_left_of_most_left[b_idx][0] == -1 {
//             have_to_the_left_of_most_left[b_idx] = seen;
//         }
//         seen[b_idx] += 1;
//     }
//
//     let mut res = 0;
//     for idx in 0..26 {
//         have_to_the_left_of_most_right[idx].iter().zip(have_to_the_left_of_most_left[idx].iter()).enumerate().for_each(|(i, (right, left))|{
//             if i == idx {
//                 if right - left > 1 {
//                     res += 1;
//                 }
//             } else if right - left > 0 {
//                 res += 1;
//             }
//         });
//     }
//
//     res
// }

impl Solver for Solution {
    fn read_inputs() -> Self {
        return Solution {
            s: "caac".to_string(),
            solution: -1,
        };
    }

    fn solve(mut self) {
        self.solution = count_palindromic_subsequence(self.s);
        dbg!(self.solution);
    }
}
