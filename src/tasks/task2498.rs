use super::Solver;

#[derive(Debug)]
pub struct Solution {
    stones: Vec<i32>,
    solution: i32,
}

// struct Accum {
//     el: i32,
//     prev_el: i32,
//     res: i32,
// }

// pub fn max_jump(stones: Vec<i32>) -> i32 {
//     11 ms
//     stones.into_iter().fold(Accum{el:0,prev_el:0,res:0}, |acc, stone|{
//         Accum{
//             el: stone,
//             prev_el: acc.el,
//             res: acc.res.max(stone-acc.prev_el)
//         }
//     }).res
// }

pub fn max_jump(stones: Vec<i32>) -> i32 {
    // 3ms
    stones.windows(3).fold(stones[1] - stones[0], |res, elems| {
        res.max(elems[2] - elems[0])
    })
}

impl Solver for Solution {
    fn read_inputs() -> Self {
        return Solution {
            stones: vec![1, 2],
            solution: -1,
        };
    }

    fn solve(mut self) {
        self.solution = max_jump(self.stones);
        dbg!(self.solution);
    }
}
