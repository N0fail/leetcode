use super::Solver;

#[derive(Debug)]
pub struct Solution {
    n: i32,
    first_player: i32,
    second_player: i32,
    solution: Vec<i32>,
}

// 12---- -> 12- -> 12 -> end
// 1-2--- -> 1-2 12- -> end 12 -> end
// 1--2-- -> 1-2 12- -> end 12 -> end
// 1---2- -> 1-2 -> end
// 1----2 -> end
// -1--2- 5 1 4
// --1--2 -> -12
// ---1---2 -> --12

// -1-2 -> 12

// 1--2- -> 1-2
// --12- -> 12-

// -12-- -> -12 12- / 1-2

// -1--2 -> 1-2

fn rec(n: usize, first_player: usize, second_player: usize, cache: &mut Vec<Vec<Vec<Option<(i32, i32)>>>>) -> (i32, i32) {
    if first_player == n - second_player {
        return (1, 1)
    }

    if let Some(x) = cache[n][first_player][second_player] {
        return x
    }
    // --1-2 -> -12 1-2
    // ---12 -> -12
    let new_n = n/2;
    let mut res = (i32::MAX-1, i32::MIN);
    let dist_from_first_to_center = first_player as i32 - new_n as i32;
    let mut dist_from_second_to_center = second_player as i32 - new_n as i32;
    let mut max_new_first = new_n - 1;
    if dist_from_second_to_center + dist_from_first_to_center > 0 {
        max_new_first = max_new_first.saturating_sub(1);
    }
    if dist_from_first_to_center < 0 {
        dist_from_second_to_center += 1;
    }
    let min_new_first = max_new_first.min(dist_from_first_to_center.max(0) as usize);
    let min_new_second = new_n.min(dist_from_second_to_center.max(0) as usize);


    for i in min_new_first..=first_player.min(max_new_first) {
        for j in (i+1).max(min_new_second)..=second_player.min(new_n).min(i+second_player-first_player) {
            let other_min_max = rec(new_n, i, j, cache);
            res.0 = res.0.min(other_min_max.0 + 1);
            res.1 = res.1.max(other_min_max.1 + 1);
        }
    }

    cache[n][first_player][second_player] = Some(res);
    return res
}

pub fn earliest_and_latest(n: i32, mut first_player: i32, mut second_player: i32) -> Vec<i32> {
    if first_player > second_player {
        (first_player, second_player) = (second_player, first_player)
    }
    let n= n as usize;
    let first_player= first_player as usize;
    let second_player = second_player as usize;
    let res = rec(n - 1, first_player - 1, second_player - 1, &mut vec![vec![vec![None; second_player]; first_player]; n]);
    vec![res.0, res.1]
}

impl Solver for Solution {
    fn read_inputs() -> Self {
        return Solution {
            n: 5,
            first_player: 3,
            second_player: 5,
            solution: vec![],
        };
    }

    fn solve(mut self) {
        self.solution = earliest_and_latest(self.n, self.first_player, self.second_player);
        dbg!(self.solution);
    }
}
