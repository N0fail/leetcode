use super::Solver;
use std::collections::BinaryHeap;

#[derive(Debug)]
pub struct Solution {
    heights: Vec<i32>,
    bricks: i32,
    ladders: i32,
    solution: i32,
}

// pub fn furthest_building(heights: Vec<i32>, bricks: i32, ladders: i32) -> i32 {
//     // loop on ladders is difficult to read
//     let mut heap = BinaryHeap::new();
//     let mut next = 1;
//     let mut bricks_left = bricks;
//     let diff = |x| 0.max(heights[x] - heights[x-1]);
//     for _ladders_count in 0..ladders {
//         while bricks_left >= 0 && next < heights.len() {
//             heap.push(diff(next));
//             bricks_left -= diff(next);
//             next += 1;
//         }
//         if next == heights.len() {
//             return (next-1) as i32
//         }
//         bricks_left += heap.pop().unwrap();
//     }
//
//     while next < heights.len() && bricks_left >= diff(next) {
//         bricks_left -= diff(next);
//         next += 1;
//     }
//
//     (next-1) as i32
// }

pub fn furthest_building(heights: Vec<i32>, mut bricks: i32, mut ladders: i32) -> i32 {
    let mut heap = BinaryHeap::new();
    let diff = |x| 0.max(heights[x] - heights[x - 1]);
    for next in 1..heights.len() {
        let d = diff(next);
        bricks -= d;
        heap.push(d);
        if bricks < 0 {
            if ladders <= 0 {
                return (next - 1) as _;
            }
            ladders -= 1;
            bricks += heap.pop().unwrap();
        }
    }

    (heights.len() - 1) as _
}

impl Solver for Solution {
    fn read_inputs() -> Self {
        return Solution {
            // heights: vec![1,29,1,3,4,5],
            // bricks: 0,
            // ladders: 2,
            // heights: vec![4,12,2,7,3,18,20,3,19],
            // bricks: 10,
            // ladders: 2,
            heights: vec![1, 3, 4, 5, 6],
            bricks: 2,
            ladders: 1,
            solution: -1,
        };
    }

    fn solve(mut self) {
        self.solution = furthest_building(self.heights, self.bricks, self.ladders);
        dbg!(self.solution);
    }
}
