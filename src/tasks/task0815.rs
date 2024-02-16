use super::Solver;
#[derive(Debug)]
pub struct Solution {
    routes: Vec<Vec<i32>>,
    source: i32,
    target: i32,
    solution: i32,
}

// pub fn num_buses_to_destination(routes: Vec<Vec<i32>>, source: i32, target: i32) -> i32 {
//     // 1192 ms barely passes
//     if source == target {
//         return 0
//     }
//     let mut bus_to_stops: HashMap<usize, HashSet<i32>> = HashMap::with_capacity(routes.len());
//     let mut bus_to_buses: HashMap<usize, HashSet<usize>> = HashMap::with_capacity(routes.len());
//     let mut to_visit = vec![];
//     for (bus, route) in routes.iter().enumerate() {
//         bus_to_stops.insert(bus, HashSet::with_capacity(route.len()));
//         bus_to_buses.insert(bus, HashSet::new());
//         route.iter().copied().for_each(|stop| {
//             bus_to_stops.get_mut(&bus).unwrap().insert(stop);
//             if stop == source {
//                 to_visit.push((bus, 1))
//             }
//         })
//     }
//
//     for (idx, (bus, stops)) in bus_to_stops.iter().enumerate() {
//         for (bus2, stops2) in bus_to_stops.iter().skip(idx+1) {
//             if let Some(_) = stops.intersection(stops2).next() {
//                 bus_to_buses.get_mut(bus).unwrap().insert(*bus2);
//                 bus_to_buses.get_mut(bus2).unwrap().insert(*bus);
//             }
//         }
//     }
//
//     let mut visited_buses: HashSet<usize> = HashSet::new();
//     let mut head = 0;
//     while head < to_visit.len() {
//         let node = to_visit[head];
//
//         if bus_to_stops.get(&node.0).unwrap().contains(&target) {
//             return node.1
//         }
//
//         head += 1;
//         visited_buses.insert(node.0);
//
//         for bus in (*bus_to_buses.get(&node.0).unwrap()).iter() {
//             if !visited_buses.contains(bus) {
//                 to_visit.push((*bus, node.1 + 1))
//             }
//         }
//     }
//
//     -1
// }

// pub fn num_buses_to_destination(routes: Vec<Vec<i32>>, source: i32, target: i32) -> i32 {
//     // 1071 ms barely passes
//     if source == target {
//         return 0
//     }
//     let mut bus_to_stops: Vec<HashSet<i32>> = Vec::with_capacity(routes.len());
//     let mut bus_to_buses: Vec<Vec<bool>> = Vec::with_capacity(routes.len());
//     let mut to_visit = vec![];
//     for (bus, route) in routes.iter().enumerate() {
//         bus_to_stops.push(HashSet::with_capacity(route.len()));
//         bus_to_buses.push(vec![false; routes.len()]);
//         route.iter().copied().for_each(|stop| {
//             bus_to_stops[bus].insert(stop);
//             if stop == source {
//                 to_visit.push((bus, 1))
//             }
//         })
//     }
//
//     for (bus, stops) in bus_to_stops.iter().enumerate() {
//         for (bus2, stops2) in bus_to_stops.iter().enumerate().skip(bus+1) {
//             if let Some(_) = stops.intersection(stops2).next() {
//                 bus_to_buses[bus][bus2] = true;
//                 bus_to_buses[bus2][bus] = true;
//             }
//         }
//     }
//
//     let mut visited_buses: HashSet<usize> = HashSet::new();
//     let mut head = 0;
//     while head < to_visit.len() {
//         let node = to_visit[head];
//
//         if bus_to_stops[node.0].contains(&target) {
//             return node.1
//         }
//
//         head += 1;
//         visited_buses.insert(node.0);
//
//         for (bus, is_connected) in bus_to_buses[node.0].iter().enumerate() {
//             if *is_connected && !visited_buses.contains(&bus) {
//                 to_visit.push((bus, node.1 + 1))
//             }
//         }
//     }
//
//     -1
// }

pub fn num_buses_to_destination(routes: Vec<Vec<i32>>, source: i32, target: i32) -> i32 {
    // 7 ms
    let source = source as usize;
    let target = target as usize;
    const UNREACHABLE: i16 = 1000;
    let mut d = [UNREACHABLE; 1_000_000];
    d[source] = 0;
    loop {
        let mut is_changed = false;
        for route in routes.iter() {
            let mut min_d = route
                .iter()
                .copied()
                .map(|stop| d[stop as usize])
                .min()
                .unwrap();
            if min_d == UNREACHABLE {
                continue;
            }
            min_d += 1;
            for stop in route.iter().copied() {
                if d[stop as usize] > min_d {
                    d[stop as usize] = min_d;
                    is_changed = true;
                }
            }
        }
        if !is_changed {
            break;
        }
    }

    if d[target] == UNREACHABLE {
        -1
    } else {
        d[target] as i32
    }
}

impl Solver for Solution {
    fn read_inputs() -> Self {
        return Solution {
            routes: vec![
                vec![7, 12],
                vec![4, 5, 15],
                vec![6],
                vec![15, 19],
                vec![9, 12, 13],
            ],
            source: 15,
            target: 12,
            solution: -1,
        };
    }

    fn solve(mut self) {
        self.solution = num_buses_to_destination(self.routes, self.source, self.target);
        dbg!(self.solution);
    }
}
