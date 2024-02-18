use super::Solver;

#[derive(Debug)]
pub struct Solution {
    n: i32,
    meetings: Vec<Vec<i32>>,
    solution: i32,
}

pub fn most_booked(n: i32, mut meetings: Vec<Vec<i32>>) -> i32 {
    meetings.sort_unstable();
    let n = n as usize;
    let mut res = vec![0; n];
    let mut end_at_by_room: Vec<i64> = vec![0; n];
    for meeting in meetings {
        let mut min_idx = n - 1;
        for (idx, end_at) in end_at_by_room.iter().enumerate().rev() {
            if *end_at <= meeting[0] as i64 || *end_at <= end_at_by_room[min_idx] {
                min_idx = idx;
            }
        }
        res[min_idx] += 1;
        end_at_by_room[min_idx] = meeting[1] as i64 + 0.max(end_at_by_room[min_idx] - meeting[0] as i64);
    }

    res.into_iter().enumerate().rev().max_by(|a, b| a.1.cmp(&b.1)).unwrap().0 as i32
}

impl Solver for Solution {
    fn read_inputs() -> Self {
        return Solution {
            n: 3,
            meetings: [[0, 10], [1, 11], [2, 12], [3, 13], [4, 14]]
                .into_iter()
                .map(|x| Vec::from(x))
                .collect(),
            solution: -1,
        };
    }

    fn solve(mut self) {
        self.solution = most_booked(self.n, self.meetings);
        dbg!(self.solution);
    }
}
