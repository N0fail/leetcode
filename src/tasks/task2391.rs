use super::Solver;

#[derive(Debug)]
pub struct Solution {
    garbage: Vec<String>,
    travel: Vec<i32>,
    solution: i32,
}

pub fn garbage_collection(garbage: Vec<String>, travel: Vec<i32>) -> i32 {
    // 3 pass
    let mut res = 0;
    for garbage_type in [b'G', b'P', b'M'] {
        let mut cur_travel = 0;
        for (g, travel) in garbage.iter().zip([0].iter().chain(travel.iter())) {
            cur_travel += travel;
            let mut chars_count = 0;
            for c in g.bytes() {
                if c == garbage_type {
                    chars_count += 1;
                }
            }
            if chars_count > 0 {
                res += cur_travel + chars_count;
                cur_travel = 0;
            }
        }
    }

    res
}

// pub fn garbage_collection(garbage: Vec<String>, travel: Vec<i32>) -> i32 {
//     // 1 pass
//     let mut res = 0;
//     let mut cur_travel = [0;3];
//     for (g, travel) in garbage.iter().zip([0].iter().chain(travel.iter())) {
//         cur_travel.iter_mut().for_each(|t| *t+=travel);
//         let mut chars_count = [0;3];
//         for c in g.bytes() {
//             match c {
//                 b'G' => chars_count[0] += 1,
//                 b'P' => chars_count[1] += 1,
//                 b'M' => chars_count[2] += 1,
//                 _ => ()
//             }
//         }
//         chars_count.iter().zip(cur_travel.iter_mut()).for_each(|(chars_count, cur_travel)| {
//             if *chars_count > 0 {
//                 res += *cur_travel + *chars_count;
//                 *cur_travel = 0;
//             }
//         });
//
//     }
//
//     res
// }

impl Solver for Solution {
    fn read_inputs() -> Self {
        return Solution {
            garbage: ["G", "P", "GP", "GG"].into_iter().map(|x| x.to_string()).collect(),
            travel: vec![2, 4, 3],
            solution: 0,
        };
    }

    fn solve(mut self) {
        self.solution = garbage_collection(self.garbage, self.travel);
        dbg!(self.solution);
    }
}
