use super::Solver;

#[derive(Debug)]
pub struct Solution {
    n: String,
    x: i32,
    solution: String,
}

pub fn max_value(n: String, x: i32) -> String {
    let x = b'0' + x as u8;
    let n = n.as_bytes();
    let pos = if n[0] == b'-' {
        n.iter().position(|b| *b > x).unwrap_or(n.len())
    } else {
        n.iter().position(|b| *b < x).unwrap_or(n.len())
    };
    String::from_utf8(
        n.iter()
            .take(pos)
            .chain([x].iter())
            .chain(n.iter().skip(pos))
            .copied()
            .collect::<Vec<_>>(),
    )
    .unwrap()
}

impl Solver for Solution {
    fn read_inputs() -> Self {
        return Solution {
            n: "-999".to_string(),
            x: 8,
            solution: "".to_string(),
        };
    }

    fn solve(mut self) {
        self.solution = max_value(self.n, self.x);
        dbg!(self.solution);
    }
}
