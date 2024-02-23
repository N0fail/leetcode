use super::Solver;

#[derive(Debug)]
pub struct Solution {
    nums: Vec<i32>,
    solution: i32,
}

// pub fn min_operations(nums: Vec<i32>) -> i32 {
//     // hashmap 18-20ms
//     nums.into_iter()
//         .fold(HashMap::new(), |mut count: HashMap<i32, i32>, num| {
//             count.entry(num).and_modify(|x| *x += 1).or_insert(1);
//             count
//         })
//         .values()
//         .copied()
//         .try_fold(0, |res, count| {
//             if count == 1 {
//                 Err(())
//             } else {
//                 match count % 3 {
//                     0 => Ok(res + count / 3),
//                     1 => Ok(res + 2 + (count - 4) / 3),
//                     2 => Ok(res + 1 + (count - 2) / 3),
//                     _ => Err(()), // not all cases lol
//                 }
//             }
//         })
//         .map_or(-1, |x| x)
// }

// pub fn min_operations(mut nums: Vec<i32>) -> i32 {
//     // sort 11-15ms
//     nums.sort_unstable();
//     nums.iter()
//         .copied()
//         .chain([i32::MAX]) // for last streak handle
//         .try_fold((nums[0], 0, 0), |(prev_num, streak, res), num| {
//             if num == prev_num {
//                 Ok((num, streak + 1, res))
//             } else {
//                 if streak == 1 {
//                     Err(())
//                 } else {
//                     match streak % 3 {
//                         0 => Ok((num, 1, res + streak / 3)),
//                         1 => Ok((num, 1, res + 2 + (streak - 4) / 3)),
//                         2 => Ok((num, 1, res + 1 + (streak - 2) / 3)),
//                         _ => Err(()), // not all cases lol
//                     }
//                 }
//             }
//         })
//         .map_or(-1, |x| x.2)
// }

pub fn min_operations(mut nums: Vec<i32>) -> i32 {
    // sort without match 7-10ms
    nums.sort_unstable();
    nums.iter()
        .copied()
        .chain([i32::MAX]) // for last streak handle
        .try_fold((nums[0], 0, 0), |(prev_num, streak, res), num| {
            if num == prev_num {
                Ok((num, streak + 1, res))
            } else {
                if streak == 1 {
                    Err(())
                } else {
                    Ok((num, 1, res + (streak + 2) / 3))
                }
            }
        })
        .map_or(-1, |x| x.2)
}

// pub fn min_operations(nums: Vec<i32>) -> i32 {
//     // hashmap without match 21ms
//     nums.into_iter()
//         .fold(HashMap::new(), |mut count: HashMap<i32, i32>, num| {
//             count.entry(num).and_modify(|x| *x += 1).or_insert(1);
//             count
//         })
//         .values()
//         .copied()
//         .try_fold(0, |res, count| {
//             if count == 1 {
//                 Err(())
//             } else {
//                 Ok(res + (count + 2) / 3)
//             }
//         })
//         .map_or(-1, |x| x)
// }

impl Solver for Solution {
    fn read_inputs() -> Self {
        return Solution {
            nums: vec![13, 13, 13, 13, 13, 7, 7, 7, 7],
            solution: 0,
        };
    }

    fn solve(mut self) {
        self.solution = min_operations(self.nums);
        dbg!(self.solution);
    }
}
