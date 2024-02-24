use super::Solver;
use std::collections::{BinaryHeap, HashMap, HashSet};

#[derive(Debug)]
pub struct Solution {
    n: i32,
    meetings: Vec<Vec<i32>>,
    first_person: i32,
    solution: Vec<i32>,
}

// pub fn find_all_people(n: i32, meetings: Vec<Vec<i32>>, first_person: i32) -> Vec<i32> {
//     // bfs + heap
//     // 221-257 ms
//     let mut g: HashMap<i32, HashMap<i32, Vec<i32>>> = HashMap::with_capacity(n as usize);
//     for meeting in meetings {
//         g.entry(meeting[0])
//             .and_modify(|to| {
//                 (*to)
//                     .entry(meeting[1])
//                     .and_modify(|time| time.push(meeting[2]))
//                     .or_insert(vec![meeting[2]]);
//             })
//             .or_insert(HashMap::from([(meeting[1], vec![meeting[2]])]));
//         g.entry(meeting[1])
//             .and_modify(|to| {
//                 (*to)
//                     .entry(meeting[0])
//                     .and_modify(|time| time.push(meeting[2]))
//                     .or_insert(vec![meeting[2]]);
//             })
//             .or_insert(HashMap::from([(meeting[0], vec![meeting[2]])]));
//     }
//     let mut pq: BinaryHeap<(i32, i32)> = BinaryHeap::new();
//     pq.push((0, 0));
//     pq.push((0, first_person));
//     let mut visited: HashSet<i32> = HashSet::new();
//     loop {
//         match pq.pop() {
//             None => break,
//             Some((secret_time, person)) => {
//                 if visited.contains(&person) {
//                     continue;
//                 }
//                 visited.insert(person);
//                 if let Some(person_meetings) = g.get(&person) {
//                     for (person, meeting_times) in person_meetings {
//                         if let Some(meeting_time) = meeting_times.iter().filter(|x| **x >= -secret_time).min() {
//                             pq.push((-meeting_time, *person));
//                         }
//                     }
//                 }
//             }
//         }
//     }
//     Vec::from_iter(visited.into_iter())
// }

pub fn find_all_people(n: i32, meetings: Vec<Vec<i32>>, first_person: i32) -> Vec<i32> {
    // bfs + heap simplified
    // 98-113ms
    let mut g: HashMap<i32, Vec<(i32, i32)>> = HashMap::with_capacity(n as usize);
    for meeting in meetings {
        g.entry(meeting[0])
            .and_modify(|person_meetings| (*person_meetings).push((meeting[1], meeting[2])))
            .or_insert(vec![(meeting[1], meeting[2])]);
        g.entry(meeting[1])
            .and_modify(|person_meetings| (*person_meetings).push((meeting[0], meeting[2])))
            .or_insert(vec![(meeting[0], meeting[2])]);
    }
    let mut pq: BinaryHeap<(i32, i32)> = BinaryHeap::new();
    pq.push((0, 0));
    pq.push((0, first_person));
    let mut visited: HashSet<i32> = HashSet::new();
    loop {
        match pq.pop() {
            None => break,
            Some((secret_time, person)) => {
                if visited.contains(&person) {
                    continue;
                }
                visited.insert(person);
                if let Some(person_meetings) = g.get(&person) {
                    for (person, meeting_time) in person_meetings {
                        if *meeting_time >= -secret_time {
                            pq.push((-*meeting_time, *person));
                        }
                    }
                }
            }
        }
    }
    Vec::from_iter(visited.into_iter())
}

impl Solver for Solution {
    fn read_inputs() -> Self {
        return Solution {
            n: 4,
            meetings: [[3, 1, 3], [1, 2, 2], [0, 3, 3]].into_iter().map(|x| Vec::from(x)).collect(),
            first_person: 3,
            solution: vec![],
        };
    }

    fn solve(mut self) {
        self.solution = find_all_people(self.n, self.meetings, self.first_person);
        dbg!(self.solution);
    }
}
