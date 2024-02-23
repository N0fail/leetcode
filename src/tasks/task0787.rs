use super::Solver;
use std::collections::VecDeque;

#[derive(Debug)]
pub struct Solution {
    n: i32,
    flights: Vec<Vec<i32>>,
    src: i32,
    dst: i32,
    k: i32,
    solution: i32,
}

pub fn find_cheapest_price(n: i32, flights: Vec<Vec<i32>>, src: i32, dst: i32, k: i32) -> i32 {
    let n = n as usize;
    let src = src as usize;
    let dst = dst as usize;
    let mut g: Vec<Vec<Option<i32>>> = vec![vec![None; n]; n];
    for flight in flights {
        g[flight[0] as usize][flight[1] as usize] = Some(flight[2]);
    }

    let mut q: VecDeque<(usize, i32, i32)> = VecDeque::new();
    q.push_back((src, 0, 0));

    loop {
        let elem = q.pop_front();
        match elem {
            None => break,
            Some((from, cur_cost, cur_k)) => {
                if cur_k > k {
                    break;
                }
                for to in 0..n {
                    match g[from][to] {
                        None => continue,
                        Some(flight) => {
                            let new_cost = cur_cost + flight;
                            if new_cost < g[src][dst].unwrap_or(i32::MAX) && new_cost <= g[src][to].unwrap_or(i32::MAX) {
                                g[src][to] = Some(new_cost);
                                q.push_back((to, new_cost, cur_k + 1));
                            }
                        }
                    }
                }
            }
        }
    }

    g[src][dst].unwrap_or(-1)
}

impl Solver for Solution {
    fn read_inputs() -> Self {
        return Solution {
            n: 4,
            flights: [[0, 1, 100], [1, 2, 100], [2, 0, 100], [1, 3, 600], [2, 3, 200]]
                .into_iter()
                .map(|x| Vec::from(x))
                .collect(),
            src: 0,
            dst: 3,
            k: 1,
            solution: 0,
        };
    }

    fn solve(mut self) {
        self.solution = find_cheapest_price(self.n, self.flights, self.src, self.dst, self.k);
        dbg!(self.solution);
    }
}
