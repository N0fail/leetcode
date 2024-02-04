use super::Solver;

#[derive(Debug)]
pub struct Solution{
    temperatures: Vec<i32>,
    solution: Vec<i32>,
}


// const RANGE: (i32, i32) = (30, 101);
// // LOL it passed
// pub fn daily_temperatures(mut temperatures: Vec<i32>) -> Vec<i32> {
//     let mut counter: Vec<Vec<usize>> = vec![];
//     counter.resize((RANGE.1 - RANGE.0) as usize, vec![]);
//     for (index, temp) in temperatures.iter().enumerate() {
//         counter[(temp - RANGE.0) as usize].push(index)
//     }
//
//     for (temp, indexes) in counter.iter().enumerate(){
//         for index in indexes {
//             let mut min_index = 1_000_000_000;
//             for indexes_with_higher_temp in &counter[temp+1 ..] {
//                 let lower_bound = indexes_with_higher_temp.binary_search(&index).unwrap_or_else(|index| index);
//                 if lower_bound >= indexes_with_higher_temp.len() {
//                     continue
//                 }
//                 min_index = min(indexes_with_higher_temp[lower_bound], min_index)
//             }
//             if min_index == 1_000_000_000 {
//                 temperatures[*index] = 0
//             } else {
//                 temperatures[*index] = (min_index - *index) as i32
//             }
//         }
//     }
//     return temperatures
// }

pub fn daily_temperatures(mut temperatures: Vec<i32>) -> Vec<i32> {
    let mut stack: Vec<(usize, i32)> = vec![];

    for idx in 0..temperatures.len() {
        let temp = temperatures[idx];
        while let Some(el) = stack.last() {
            if el.1 >= temp {
                break
            }
            temperatures[el.0] = (idx - el.0) as i32;
            stack.pop();
        }
        stack.push((idx, temp))
    }

    while let Some(el) = stack.pop() {
        temperatures[el.0] = 0;
    }

    return temperatures
}

impl Solver for Solution {
    fn read_inputs() -> Self {
        return Solution {
            temperatures: vec![73,74,75,71,69,72,76,73],
            solution: vec![],
        };
    }

    fn solve(mut self) {
        self.solution = daily_temperatures(self.temperatures);
        dbg!(self.solution);
    }
}