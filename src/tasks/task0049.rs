use super::Solver;
use std::collections::HashMap;
#[derive(Debug)]
pub struct Solution {
    strs: Vec<String>,
    solution: Vec<Vec<String>>,
}

// pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
//     // using sort and iterators 13ms, unnecessary allocations in the end
//     let mut encoded: Vec<([usize;26], String)> = Vec::with_capacity(strs.len());
//     let encode = |s: &String| s.chars().fold([0usize;26], |mut enc, c| {enc[c as usize - b'a' as usize] += 1; enc});
//     strs.into_iter().for_each(|s| encoded.push((encode(&s),s)));
//     encoded.sort_by(|a, b| a.0.cmp(&b.0));
//     let mut res: Vec<Vec<&([usize;26], String)>> = vec![vec![&encoded[0]]];
//     encoded.windows(2).for_each(|lr|{
//         if lr[0].0 == lr[1].0 {
//             let n = res.len();
//             res[n-1].push(&lr[1])
//         } else {
//             res.push(vec![&lr[1]])
//         }
//     });
//     res.into_iter()
//         .map(|arr| arr.into_iter()
//             .map(|x| x.1.clone())
//             .collect())
//         .collect()
// }

// pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
//     // using sort and iterators 7ms
//     let mut encoded: Vec<([usize;26], String)> = Vec::with_capacity(strs.len());
//     let encode = |s: &String| s.chars().fold([0usize;26], |mut enc, c| {enc[c as usize - b'a' as usize] += 1; enc});
//     strs.into_iter().for_each(|s| encoded.push((encode(&s),s)));
//     encoded.sort_by(|a, b| a.0.cmp(&b.0));
//     let mut last = &encoded[0];
//     encoded[1..].iter().fold(vec![vec![encoded[0].1.clone()]], |mut res, enc|{
//         if enc.0 == last.0 {
//             let n = res.len();
//             res[n-1].push(enc.1.clone())
//         } else {
//             res.push(vec![enc.1.clone()])
//         }
//         last = enc;
//         res
//     })
// }

// pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
//     // using sort and imperative 10ms
//     let mut encoded: Vec<([usize;26], String)> = Vec::with_capacity(strs.len());
//     for s in strs {
//         let mut new_encoding = [0usize;26];
//         for c in s.chars() {
//             new_encoding[c as usize - b'a' as usize] += 1
//         }
//         encoded.push((new_encoding, s))
//     }
//     encoded.sort_by(|a, b| a.0.cmp(&b.0));
//     let mut last = &encoded[0];
//     let mut res: Vec<Vec<String>> = vec![vec![last.1.clone()]];
//     for enc in encoded[1..].iter() {
//         if enc.0 == last.0 {
//             let n = res.len();
//             res[n-1].push(enc.1.clone())
//         } else {
//             res.push(vec![enc.1.clone()])
//         }
//         last = enc
//     }
//     res
// }

pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    // using hashmap and iterators 12ms
    let mut encoded: HashMap<[usize; 26], Vec<String>> = HashMap::new();
    let encode = |s: &String| {
        s.chars().fold([0usize; 26], |mut enc, c| {
            enc[c as usize - b'a' as usize] += 1;
            enc
        })
    };
    strs.into_iter().for_each(|s| {
        let enc = encode(&s);
        match encoded.get_mut(&enc) {
            None => {
                encoded.insert(enc, vec![s]);
            }
            Some(v) => v.push(s),
        };
    });
    // encoded.iter().map(|(k, v)| v.clone()).collect() this is 16 ms
    encoded.into_values().collect()
}

impl Solver for Solution {
    fn read_inputs() -> Self {
        let input = ["eat", "tea", "tan", "ate", "nat", "bat"];
        return Solution {
            strs: input.map(|s| String::from(s)).into_iter().collect(),
            solution: vec![],
        };
    }

    fn solve(mut self) {
        self.solution = group_anagrams(self.strs);
        dbg!(self.solution);
    }
}
