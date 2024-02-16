use super::Solver;

#[derive(Debug)]
pub struct Solution {
    s: String,
    t: String,
    solution: String,
}

#[derive(Debug, Clone)]
struct CharsCount {
    count: [usize; 52],
}

impl CharsCount {
    fn char_to_idx(c: u8) -> usize {
        if c < b'a' {
            return (c - b'A') as usize;
        }
        return (c - b'a' + 26) as usize;
    }

    fn add(&mut self, b: u8) {
        self.count[CharsCount::char_to_idx(b)] += 1
    }

    fn remove(&mut self, b: u8) {
        let idx = CharsCount::char_to_idx(b);
        if self.count[idx] == 0 {
            return;
        }
        self.count[idx] -= 1
    }

    fn count_b(&self, b: u8) -> usize {
        let idx = CharsCount::char_to_idx(b);
        self.count[idx]
    }

    fn contains_b(&self, b: u8) -> bool {
        let idx = CharsCount::char_to_idx(b);
        if self.count[idx] == 0 {
            return false;
        }
        return true;
    }

    fn contains(&self, other: &Self) -> bool {
        self.count
            .iter()
            .zip(other.count.iter())
            .all(|(my, other)| *my >= *other)
    }
}

pub fn min_window(s: String, t: String) -> String {
    let mut cur = CharsCount { count: [0; 52] };
    let s_bytes: Vec<u8> = s.bytes().collect();

    let mut target = CharsCount { count: [0; 52] };
    t.bytes().for_each(|b| target.add(b));

    const NULL: usize = 1 << 30;
    let mut solution: (usize, usize) = (0, NULL);

    let mut prev_result = solution;
    for right_idx in 0..s.len() {
        let b = s_bytes[right_idx];
        if target.contains_b(b) {
            cur.add(b);
            if prev_result.1 == NULL && cur.contains(&target) {
                prev_result.1 = right_idx;
            }
            if prev_result.1 != NULL {
                let mut new_left = prev_result.0;
                for left_idx in prev_result.0..right_idx + 1 {
                    let b = s_bytes[left_idx];
                    if target.contains_b(b) {
                        let cur_count = cur.count_b(b);
                        let t_count = target.count_b(b);
                        if cur_count < t_count {
                            break;
                        }
                        new_left = left_idx;
                        if cur_count == t_count {
                            break;
                        }
                        cur.remove(b);
                    }
                }
                prev_result = (new_left, right_idx);
                if right_idx - new_left < solution.1 - solution.0 {
                    solution = prev_result
                }
            }
        }
    }

    if solution.1 == NULL {
        return "".to_string();
    }

    String::from(std::str::from_utf8(&s_bytes[solution.0..solution.1 + 1]).unwrap())
}

impl Solver for Solution {
    fn read_inputs() -> Self {
        return Solution {
            s: "abba".to_string(),
            t: "bba".to_string(),
            solution: "".to_string(),
        };
    }

    fn solve(mut self) {
        self.solution = min_window(self.s, self.t);
        dbg!(self.solution);
    }
}
