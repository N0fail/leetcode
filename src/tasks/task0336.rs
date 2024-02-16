use super::Solver;
use std::cmp::Ordering;
// todo This does not work, should use straightforward hashmap solution
#[derive(Debug)]
pub struct Solution {
    input: Vec<String>,
    solution: Vec<Vec<i32>>,
}

struct InpWord {
    word: String,
    position: usize,
}

impl Eq for InpWord {}

impl PartialEq<Self> for InpWord {
    fn eq(&self, other: &Self) -> bool {
        self.word == other.word
    }
}

impl PartialOrd<Self> for InpWord {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.word.partial_cmp(&other.word)
    }
}

impl Ord for InpWord {
    fn cmp(&self, other: &Self) -> Ordering {
        self.word.cmp(&other.word)
    }
}

// pub fn is_palindrome(s: &str) -> bool {
//     // let arr = s.bytes();
//     // let l = arr.len();
//     // for i in 0..l/2 {
//     //     if arr[i] != arr[l-i-1] {
//     //         return false
//     //     }
//     // }
//     return true
// }

pub fn palindrome_pairs(words: Vec<String>) -> Vec<Vec<i32>> {
    let mut orig_list: Vec<InpWord> = vec![];
    let mut reverse_list: Vec<InpWord> = vec![];
    orig_list.reserve(words.len());
    reverse_list.reserve(words.len());
    for (position, word) in words.iter().enumerate() {
        orig_list.push(InpWord {
            word: word.clone(),
            position,
        });
        reverse_list.push(InpWord {
            word: word.chars().rev().collect::<String>(),
            position,
        })
    }
    orig_list.sort();
    reverse_list.sort();

    let mut result: Vec<Vec<i32>> = vec![];
    for orig_word in &orig_list {
        for reverse_word in &reverse_list {
            if orig_word.position == reverse_word.position {
                continue;
            }
            if reverse_word.word.starts_with(&orig_word.word) {
                result.push(vec![
                    orig_word.position as i32,
                    reverse_word.position as i32,
                ]);
            } else {
                break;
            }
        }
    }

    return result;
}

impl Solver for Solution {
    fn read_inputs() -> Self {
        let inp = ["a", "ab"];
        let mut input: Vec<String> = vec![];
        input.reserve(inp.len());
        for s in inp {
            input.push(String::from(s));
        }
        return Solution {
            input,
            solution: vec![],
        };
    }

    fn solve(mut self) {
        self.solution = palindrome_pairs(self.input);
        dbg!(self.solution);
    }
}

// "a","ab"
// "a","ba"
