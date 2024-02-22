use super::Solver;

#[derive(Debug)]
pub struct Solution {
    colors: String,
    needed_time: Vec<i32>,
    solution: i32,
}

// pub fn min_cost(colors: String, needed_time: Vec<i32>) -> i32 {
//     let res = colors
//         .as_bytes()
//         .into_iter()
//         .zip(needed_time.into_iter())
//         // (prev_char, prev_max_time, sum)
//         .fold((0u8, 0, 0), |res, (b, time)| {
//             if *b == res.0 {
//                 (
//                     *b,
//                     res.1.max(time),
//                     res.2 + time,
//                 )
//             } else {
//                 (*b, time, res.2 - res.1 + time)
//             }
//         });
//     res.2 - res.1
// }

// pub fn min_cost(colors: String, needed_time: Vec<i32>) -> i32 {
//     let mut prev_char = 0u8;
//     let mut prev_max_time = 0;
//     let mut sum = 0;
//     colors
//         .as_bytes()
//         .into_iter()
//         .zip(needed_time.into_iter())
//         .for_each(|(b, time)| {
//             sum += time;
//             if *b == prev_char {
//                 prev_max_time = prev_max_time.max(time);
//             } else {
//                 sum -= prev_max_time;
//                 prev_max_time = time;
//             }
//             prev_char = *b;
//         });
//     sum - prev_max_time
// }

// pub fn min_cost(colors: String, needed_time: Vec<i32>) -> i32 {
//     let mut prev_char = 0u8;
//     let mut prev_max_time = 0;
//     let mut sum = needed_time.iter().sum::<i32>();
//     colors
//         .as_bytes()
//         .into_iter()
//         .zip(needed_time.into_iter())
//         .for_each(|(b, time)| {
//             if *b == prev_char {
//                 prev_max_time = prev_max_time.max(time);
//             } else {
//                 sum -= prev_max_time;
//                 prev_max_time = time;
//             }
//             prev_char = *b;
//         });
//     sum - prev_max_time
// }

pub fn min_cost(colors: String, needed_time: Vec<i32>) -> i32 {
    let (_, prev_max, sum) = colors.bytes().zip(needed_time.iter().copied()).fold(
        (0u8, 0, needed_time.iter().sum::<i32>()),
        |(prev_byte, prev_max, sum), (b, time)| {
            if b == prev_byte {
                (b, prev_max.max(time), sum)
            } else {
                (b, time, sum - prev_max)
            }
        },
    );
    sum - prev_max
}

impl Solver for Solution {
    fn read_inputs() -> Self {
        return Solution {
            colors: "aabaa".to_string(),
            needed_time: vec![1, 2, 3, 4, 1],
            solution: 0,
        };
    }

    fn solve(mut self) {
        self.solution = min_cost(self.colors, self.needed_time);
        dbg!(self.solution);
    }
}
