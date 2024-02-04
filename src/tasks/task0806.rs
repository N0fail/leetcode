use super::Solver;

#[derive(Debug)]
pub struct Solution{
    widths: Vec<i32>,
    s: String,
    solution: Vec<i32>,
}

pub fn number_of_lines(widths: Vec<i32>, s: String) -> Vec<i32> {
    s.chars().into_iter()
        .map(|x| widths[x as usize - b'a' as usize])
        .fold(vec![1,0], |mut result, width| {
            if result[1] + width > 100 {
                result[0] += 1;
                result[1] = width;
            } else {
                result[1] += width
            }
            result
        })
}

impl Solver for Solution {
    fn read_inputs() -> Self {
        return Solution {
            widths: vec![4,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10],
            s: "bbbcccdddaaa".to_string(),
            solution: vec![],
        };
    }

    fn solve(mut self) {
        self.solution = number_of_lines(self.widths, self.s);
        dbg!(self.solution);
    }
}
