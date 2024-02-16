use super::Solver;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Solution {
    arr: Vec<i32>,
    k: i32,
    solution: i32,
}

// pub fn find_least_num_of_unique_ints(arr: Vec<i32>, k: i32) -> i32 {
//     // 40ms
//     let mut counts: HashMap<i32, i32> = HashMap::new();
//     for num in arr {
//         counts.entry(num).and_modify(|count| *count+=1).or_insert(1);
//     }
//     let mut heap = BinaryHeap::from_iter(
//         counts.into_iter()
//             .map(|(num, count)| vec![(-count, num); count as usize])
//             .flatten()
//     );
//     for _ in 0..k {
//         heap.pop();
//     }
//     heap.into_iter().fold(HashSet::new(),|mut res, (_,num)| {
//         res.insert(num);
//         res
//     }).len() as i32
// }

// pub fn find_least_num_of_unique_ints(arr: Vec<i32>, k: i32) -> i32 {
//     // 37 ms, no unnecessary alloc
//     let mut counts: HashMap<i32, i32> = HashMap::new();
//     for num in arr {
//         counts.entry(num).and_modify(|count| *count+=1).or_insert(1);
//     }
//     let mut heap = BinaryHeap::from_iter(
//         counts.into_iter()
//             .map(|(num, count)| iter::repeat((-count,num)).take(count as usize))
//             .flatten()
//     );
//     for _ in 0..k {
//         heap.pop();
//     }
//     heap.into_iter().fold(HashSet::new(),|mut res, (_,num)| {
//         res.insert(num);
//         res
//     }).len() as i32
// }

// pub fn find_least_num_of_unique_ints(mut arr: Vec<i32>, k: i32) -> i32 {
//     // 168 ms
//     let k = k as usize;
//     let mut counts: HashMap<i32, i32> = HashMap::new();
//     for num in &arr {
//         counts.entry(*num).and_modify(|count| *count+=1).or_insert(1);
//     }
//     arr.sort_by(|a, b| {
//         let res = counts.get(a).unwrap().cmp(counts.get(b).unwrap());
//         if res == Equal {
//             a.cmp(b)
//         } else {
//             res
//         }
//     });
//
//     arr.into_iter().skip(k).fold((-1, 0),|res, num| {
//         if num != res.0 {
//             (num, res.1 + 1)
//         } else {
//             (num, res.1)
//         }
//     }).1
// }

// pub fn find_least_num_of_unique_ints(mut arr: Vec<i32>, mut k: i32) -> i32 {
//     // 21 ms
//     let mut counts: HashMap<i32, i32> = HashMap::new();
//     for num in arr {
//         counts.entry(num).and_modify(|count| *count-=1).or_insert(-1);
//     }
//     let mut heap = BinaryHeap::from_iter(counts.into_values());
//     loop {
//         match heap.peek() {
//             Some(count) => if -count > k {
//                 return heap.len() as i32;
//             } else {
//                 k += heap.pop().unwrap();
//             },
//             None => return heap.len() as i32
//         }
//     }
// }

// pub fn find_least_num_of_unique_ints(mut arr: Vec<i32>, mut k: i32) -> i32 {
//     // 17 ms
//     let mut counts: HashMap<i32, i32> = HashMap::new();
//     for num in arr {
//         counts.entry(num).and_modify(|count| *count-=1).or_insert(-1);
//     }
//     let mut heap = BinaryHeap::from(counts.into_values().collect::<Vec<_>>());
//     loop {
//         match heap.peek() {
//             Some(count) => if -count > k {
//                 return heap.len() as i32;
//             } else {
//                 k += heap.pop().unwrap();
//             },
//             None => return heap.len() as i32
//         }
//     }
// }

pub fn find_least_num_of_unique_ints(arr: Vec<i32>, mut k: i32) -> i32 {
    // 13 ms
    let mut counts: HashMap<i32, i32> = HashMap::new();
    for num in arr {
        counts
            .entry(num)
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }
    let mut counts_vec = counts.into_values().collect::<Vec<_>>();
    counts_vec.sort_unstable();
    counts_vec
        .into_iter()
        .skip_while(|num| {
            k -= num;
            k >= 0
        })
        .count() as i32
}

impl Solver for Solution {
    fn read_inputs() -> Self {
        return Solution {
            arr: vec![4, 3, 1, 1, 3, 3, 2],
            k: 3,
            solution: -1,
        };
    }

    fn solve(mut self) {
        self.solution = find_least_num_of_unique_ints(self.arr, self.k);
        dbg!(self.solution);
    }
}
