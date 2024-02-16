use super::Solver;

#[derive(Debug)]
pub struct Solution {
    s: String,
    solution: String,
}

pub fn sort_vowels(s: String) -> String {
    // sort 10 ms
    let vowels = b"aeiouAEIOU";
    let s = s.as_bytes().iter().copied().collect::<Vec<_>>();
    let mut v = s
        .iter()
        .copied()
        .filter(|x| vowels.contains(x))
        .collect::<Vec<_>>();
    v.sort_unstable_by(|a, b| b.cmp(a));
    String::from_utf8(
        s.into_iter()
            .map(|x| {
                if vowels.contains(&x) {
                    v.pop().unwrap()
                } else {
                    x
                }
            })
            .collect::<Vec<_>>(),
    )
    .unwrap()
}

// pub fn sort_vowels(s: String) -> String {
//     // count 11 ms
//     let vowels = b"AEIOUaeiou";
//     let get_idx = |vowel| vowels.iter().position(|v| v==vowel).unwrap();
//     let s = s.as_bytes().iter().copied().collect::<Vec<_>>();
//     let mut v = s.iter()
//         .filter(|x| vowels.contains(x))
//         .fold([0; 10], |mut res, x|{
//         res[get_idx(x)] += 1;
//         res
//     });
//     String::from_utf8(
//         s.into_iter()
//             .map(|x|
//                 if vowels.contains(&x) {
//                     let (idx, count) = v.iter_mut()
//                         .enumerate()
//                         .filter(|(_, count)| **count > 0)
//                         .next()
//                         .unwrap();
//                     *count -= 1;
//                     vowels[idx]
//                 } else {
//                     x
//                 })
//             .collect::<Vec<_>>()
//     ).unwrap()
// }

impl Solver for Solution {
    fn read_inputs() -> Self {
        return Solution {
            s: "lEetcOde".to_string(),
            solution: "".to_string(),
        };
    }

    fn solve(mut self) {
        self.solution = sort_vowels(self.s);
        dbg!(self.solution);
    }
}
