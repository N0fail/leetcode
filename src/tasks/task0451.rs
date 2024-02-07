use super::Solver;
#[derive(Debug)]
pub struct Solution{
    s: String,
    solution: String,
}


const START: usize = b'0' as usize;
const END: usize = b'z' as usize;
pub fn frequency_sort(s: String) -> String {
    let mut freq: Vec<_> = (START..END+1).map(|idx| (0, idx as u8 as char)).collect();
    s.chars().for_each(|c| freq[c as usize - START].0 += 1);
    freq.sort_by(|a,b| b.0.cmp(&a.0));
    freq.iter()
        .filter(|el| el.0 > 0)
        .map(
            |el| (0..el.0).map(|_|el.1).collect::<String>()
        )
        .collect::<String>()
}


impl Solver for Solution {
    fn read_inputs() -> Self {
        return Solution {
            s: "tree".to_string(),
            solution: "".to_string(),
        };
    }

    fn solve(mut self) {
        self.solution = frequency_sort(self.s);
        dbg!(self.solution);
    }
}