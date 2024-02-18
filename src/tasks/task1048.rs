use super::Solver;

#[derive(Debug)]
pub struct Solution {
    words: Vec<String>,
    solution: i32,
}

fn is_predecessor(pred: &[u8], w: &[u8]) -> bool {
    for skip_char in 0..w.len() {
        if w.iter()
            .take(skip_char)
            .chain(w.iter().skip(skip_char + 1))
            .zip(pred.iter())
            .all(|(w_b, pred_b)| w_b == pred_b)
        {
            return true;
        }
    }

    false
}

pub fn longest_str_chain(words: Vec<String>) -> i32 {
    // 10ms 2.18mb
    let mut words = words.iter().map(|x| x.as_bytes()).collect::<Vec<_>>();
    words.sort_unstable_by_key(|word| word.len());
    let mut longest_by_idx = vec![1; words.len()];
    for (i, pred) in words.iter().enumerate().rev() {
        let mut longest = 0;
        for (j, w) in words.iter().enumerate().skip(i + 1) {
            let len_diff = w.len() - pred.len();
            if len_diff == 0 {
                continue;
            }
            if len_diff > 1 {
                break;
            }
            if longest <= longest_by_idx[j] && is_predecessor(pred, w) {
                longest = longest_by_idx[j];
            }
        }
        longest_by_idx[i] += longest;
    }

    longest_by_idx.into_iter().max().unwrap()
}

impl Solver for Solution {
    fn read_inputs() -> Self {
        return Solution {
            words: ["xbc", "pcxbcf", "xb", "cxbc", "pcxbc"]
                .into_iter()
                .map(|x| x.to_string())
                .collect(),
            solution: -1,
        };
    }

    fn solve(mut self) {
        self.solution = longest_str_chain(self.words);
        dbg!(self.solution);
    }
}
