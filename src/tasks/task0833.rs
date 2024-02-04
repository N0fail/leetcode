use super::Solver;

#[derive(Debug)]
pub struct Solution{
    s: String,
    indices: Vec<i32>,
    sources: Vec<String>,
    targets: Vec<String>,
    solution: String,
}

pub fn find_replace_string(s: String, indices: Vec<i32>, sources: Vec<String>, targets: Vec<String>) -> String {
    // this is accepted, but incorrect
    // let mut commands = indices
    //     .into_iter()
    //     .zip(sources.into_iter())
    //     .zip(targets.into_iter())
    //     .map(|((idx, source), target)|(idx as usize, source, target))
    //     .collect::<Vec<_>>();
    //
    // commands.sort_by_key(|(idx, _, _)| *idx);
    //
    // let mut last_used_idx: usize = 1 << 30;
    // commands.into_iter()
    //     .rev()
    //     .fold(s, | mut s, (idx, source, target)| {
    //         if idx != last_used_idx && idx+source.len() <= s.len() && s[idx..idx+source.len()] == source {
    //             s.replace_range(idx..idx+source.len(), &target);
    //             last_used_idx = idx
    //         }
    //         s
    //     })
    let s_chars = s.chars();
    let mut result: Vec<String> = s_chars.clone().map(|x| x.to_string()).collect();
    let s_chars: Vec<char> = s_chars.collect();
    indices.into_iter()
        .zip(sources.into_iter())
        .zip(targets.into_iter())
        .for_each(|((idx, source), target)| {
            if source.len() <= s_chars[idx as usize..].len()
                &&source.chars().into_iter()
                .zip(s_chars[idx as usize..].iter())
                .all(|(t,sc)| t == *sc) {
                result[idx as usize] = target;
                for i in 1..source.len() {
                    result[idx as usize + i] = "".to_string()
                }
            }
        });
    result.into_iter().reduce(|acc, x| acc + &x).unwrap()
}

impl Solver for Solution {
    fn read_inputs() -> Self {
        return Solution {
            // s: "abcd".to_string(),
            // indices: vec![0, 2],
            // sources: vec!["a", "cd"].iter().map(|el| el.to_string()).collect(),
            // targets: vec!["eee", "ffff"].iter().map(|el| el.to_string()).collect(),
            // solution: "".to_string(),

            // s: "abcde".to_string(),
            // indices: vec![2, 2],
            // sources: vec!["cdef","bc"].iter().map(|el| el.to_string()).collect(),
            // targets: vec!["f","fe"].iter().map(|el| el.to_string()).collect(),
            // solution: "".to_string(),

            s: "ab".to_string(),
            indices: vec![1, 0],
            sources: vec!["b","abb"].iter().map(|el| el.to_string()).collect(),
            targets: vec!["bb","bbb"].iter().map(|el| el.to_string()).collect(),
            solution: "".to_string(),
        };
    }

    fn solve(mut self) {
        self.solution = find_replace_string(self.s, self.indices, self.sources, self.targets);
        dbg!(self.solution);
    }
}
