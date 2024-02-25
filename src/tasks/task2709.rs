use super::Solver;
use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Debug)]
pub struct Solution {
    nums: Vec<i32>,
    solution: bool,
}

// pub fn can_traverse_all_pairs(mut nums: Vec<i32>) -> bool {
//     // tle
//     if nums.len() == 1{
//         return true
//     }
//     nums.sort_unstable();
//     let mut clusters: Vec<HashSet<usize>> = Vec::new();
//     let mut primes: Vec<usize> = Vec::new();
//     for i in 2..=(100_000f64.sqrt() as usize) {
//         primes.push(i);
//         for j in 2..=((i as f64).sqrt() as usize) {
//             if i % j == 0 {
//                 primes.pop();
//                 break;
//             }
//         }
//     }
//     for num in nums {
//         if num == 1 {
//             return false
//         }
//         let mut num = num as usize;
//         let mut divisers: HashSet<usize> = HashSet::new();
//         for divisor in &primes {
//             if divisor * divisor > num {
//                 break;
//             }
//             if num % divisor == 0 {
//                 divisers.insert(*divisor);
//                 while num % divisor == 0 {
//                     num /= divisor;
//                 }
//             }
//         }
//         if num != 1 {
//             divisers.insert(num);
//             primes.push(num);
//         }
//         let intersecting_clusters = clusters
//             .iter()
//             .enumerate()
//             .filter(|(_, cluster)| cluster.intersection(&divisers).next().is_some())
//             .map(|(idx, _)| idx)
//             .collect::<Vec<_>>();
//         if intersecting_clusters.len() == 0 {
//             clusters.push(divisers)
//         } else {
//             for idx in &intersecting_clusters {
//                 divisers = divisers.union(&clusters[*idx]).copied().collect();
//                 clusters[*idx].clear();
//             }
//             clusters[intersecting_clusters[0]] = divisers;
//         }
//     }
//
//     clusters.into_iter().filter(|c| !c.is_empty()).take(2).count() == 1
// }

pub fn can_traverse_all_pairs(mut nums: Vec<i32>) -> bool {
    // bfs
    if nums.len() == 1 {
        return true;
    }
    nums.sort_unstable();
    let mut primes: Vec<usize> = Vec::new();
    let mut primes_to_num: HashMap<usize, HashSet<usize>> = HashMap::new();
    let mut num_to_primes: HashMap<usize, HashSet<usize>> = HashMap::new();
    for i in 2..=(100_000f64.sqrt() as usize) {
        primes.push(i);
        for j in 2..=((i as f64).sqrt() as usize) {
            if i % j == 0 {
                primes.pop();
                break;
            }
        }
    }
    for num in &nums {
        if *num == 1 {
            return false;
        }
        let mut new_prime = *num as usize;
        let num_orig = new_prime;
        if num_to_primes.contains_key(&num_orig) {
            continue;
        }
        num_to_primes.insert(num_orig, HashSet::new());
        for divisor in &primes {
            if divisor * divisor > new_prime {
                break;
            }
            if new_prime % divisor == 0 {
                primes_to_num
                    .entry(*divisor)
                    .and_modify(|nums| {
                        nums.insert(num_orig);
                    })
                    .or_insert(HashSet::from([num_orig]));
                num_to_primes.entry(num_orig).and_modify(|primes| {
                    primes.insert(*divisor);
                });
                while new_prime % divisor == 0 {
                    new_prime /= divisor;
                }
            }
        }
        if new_prime != 1 {
            primes.push(new_prime);
            primes_to_num
                .entry(new_prime)
                .and_modify(|nums| {
                    nums.insert(num_orig);
                })
                .or_insert(HashSet::from([num_orig]));
            num_to_primes.entry(num_orig).and_modify(|primes| {
                primes.insert(new_prime);
            });
        }
    }
    let mut primes_visited: HashSet<usize> = HashSet::with_capacity(primes_to_num.len());
    let mut nums_visited: HashSet<usize> = HashSet::with_capacity(num_to_primes.len());
    let mut q: VecDeque<usize> = VecDeque::new();
    q.push_back(*primes_to_num.iter().next().unwrap().0);
    while !q.is_empty() {
        let prime = q.pop_front().unwrap();
        for num in primes_to_num.get(&prime).unwrap() {
            if nums_visited.insert(*num) {
                for prime in num_to_primes.get(num).unwrap() {
                    if primes_visited.insert(*prime) {
                        q.push_back(*prime);
                    }
                }
            }
        }
    }

    nums.iter().all(|num| nums_visited.contains(&(*num as usize)))
}

impl Solver for Solution {
    fn read_inputs() -> Self {
        return Solution {
            nums: vec![2, 1],
            solution: true,
        };
    }

    fn solve(mut self) {
        self.solution = can_traverse_all_pairs(self.nums);
        dbg!(self.solution);
    }
}
