use super::Solver;

#[derive(Debug)]
pub struct Solution {
    n: i32,
    trust: Vec<Vec<i32>>,
    solution: i32,
}

// pub fn find_judge(n: i32, trust: Vec<Vec<i32>>) -> i32 {
//     // 14 ms, 2.76 Mb
//     let mut trust_in_me = vec![0; n as usize];
//     let mut i_trust = vec![0; n as usize];
//     for rel in trust {
//         i_trust[rel[0] as usize - 1] += 1;
//         trust_in_me[rel[1] as usize - 1] += 1;
//     }
//
//     let candidates = trust_in_me
//         .into_iter()
//         .enumerate()
//         .filter(|(_, x)| *x == n - 1)
//         .map(|(i, _)| i)
//         .collect::<HashSet<_>>();
//     let candidates2 = i_trust
//         .into_iter()
//         .enumerate()
//         .filter(|(_, x)| *x == 0)
//         .map(|(i, _)| i)
//         .collect::<HashSet<_>>();
//
//     let judge = candidates.intersection(&candidates2).collect::<Vec<_>>();
//
//     if judge.len() != 1 {
//         return -1;
//     }
//
//     *judge[0] as i32 + 1
// }

pub fn find_judge(n: i32, trust: Vec<Vec<i32>>) -> i32 {
    // 9ms 2.68Mb
    trust
        .into_iter()
        .fold(vec![0; n as usize], |mut trust_diff, rel| {
            trust_diff[rel[0] as usize - 1] -= 1;
            trust_diff[rel[1] as usize - 1] += 1;
            trust_diff
        })
        .into_iter()
        .enumerate()
        .find_map(|(idx, x)| if x == n - 1 { Some(idx as i32 + 1) } else { None })
        .unwrap_or(-1)
}

// pub fn find_judge(n: i32, trust: Vec<Vec<i32>>) -> i32 {
//     // 15ms 2.68Mb
//     trust
//         .into_iter()
//         .fold(vec![0; n as usize], |mut trust_diff, rel| {
//             trust_diff[rel[0] as usize - 1] -= 1;
//             trust_diff[rel[1] as usize - 1] += 1;
//             trust_diff
//         })
//         .into_iter()
//         .position(|x| x == n-1)
//         .map_or(-1, |idx| idx as i32 + 1)
// }

impl Solver for Solution {
    fn read_inputs() -> Self {
        return Solution {
            n: 3,
            trust: [[1, 3], [2, 3], [3, 1]].into_iter().map(|x| Vec::from(x)).collect(),
            solution: -1,
        };
    }

    fn solve(mut self) {
        self.solution = find_judge(self.n, self.trust);
        dbg!(self.solution);
    }
}
