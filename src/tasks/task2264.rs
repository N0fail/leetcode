use super::Solver;

#[derive(Debug)]
pub struct Solution {
    num: String,
    solution: String,
}

// pub fn largest_good_integer(num: String) -> String {
//     // 1ms lazy solution
//     ["999", "888", "777", "666", "555", "444", "333", "222", "111", "000"]
//         .into_iter()
//         .filter(|s| num.contains(s))
//         .map(|x| x.to_string())
//         .next()
//         .unwrap_or("".to_string())
// }

pub fn largest_good_integer(num: String) -> String {
    // 1ms
    String::from_utf8(
        num.as_bytes()
            .windows(3)
            .filter(|window| window.iter().all(|el| *el == window[0]))
            .max_by(|a, b| a[0].cmp(&b[0]))
            .unwrap_or(&[])
            .to_vec(),
    )
    .unwrap()
}

impl Solver for Solution {
    fn read_inputs() -> Self {
        return Solution {
            num: "2300019".to_string(),
            solution: "".to_string(),
        };
    }

    fn solve(mut self) {
        self.solution = largest_good_integer(self.num);
        dbg!(self.solution);
    }
}
