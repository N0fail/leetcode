use super::Solver;
use std::fs::read_to_string;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

#[derive(Debug)]
pub struct Solution {
    inputs: Vec<Vec<i32>>,
    results: Vec<Vec<i32>>,
}

pub fn k_saw(arr: &Vec<i32>) -> Vec<i32> {
    let mut k_streak_at_idx = vec![(0, 0); arr.len()];
    let mut center = 1;
    while center < arr.len() - 1 {
        let mut k = 1;
        while arr[center - k + 1] > arr[center - k] && arr[center + k - 1] > arr[center + k] {
            k += 1;
            if k + center >= arr.len() {
                break;
            }
            if k > center {
                break;
            }
        }
        k -= 1;
        if k > 0 {
            k_streak_at_idx[center] = (k, 1);
            if center > k * 2 && k_streak_at_idx[center - 2 * k].0 == k {
                k_streak_at_idx[center].1 = k_streak_at_idx[center - 2 * k].1 + 1
            }
        }
        center = center + k + 1;
    }

    let mut res = vec![0; arr.len()];
    for (k, streak) in k_streak_at_idx {
        if k > 0 {
            res[k - 1] = res[k - 1].max(streak)
        }
    }

    let mut is_bigger_exists = false;
    for i in (0..res.len()).rev() {
        if is_bigger_exists {
            res[i] = res[i].max(1);
        }
        is_bigger_exists = is_bigger_exists || res[i] > 0;
    }

    res
}

impl Solver for Solution {
    fn read_inputs() -> Self {
        let mut inputs: Vec<Vec<i32>> = Vec::new();
        for (_, line) in read_to_string(Self::get_input_filename())
            .unwrap()
            .lines()
            .skip(1)
            .enumerate()
            .filter(|(idx, _)| idx % 2 != 0)
        {
            inputs.push(
                line.to_string()
                    .split(" ")
                    .map(|x| x.parse().unwrap())
                    .collect(),
            )
        }

        return Solution {
            inputs,
            results: vec![],
        };
    }

    fn solve(&mut self) {
        for input in &self.inputs {
            self.results.push(k_saw(input))
        }
    }

    fn write_output(&self) {
        let path = Path::new(Self::get_output_filename());
        let display = path.display();

        // Open a file in write-only mode, returns `io::Result<File>`
        let mut output_file = match File::create(&path) {
            Err(why) => panic!("couldn't create {}: {}", display, why),
            Ok(file) => file,
        };

        for result in &self.results {
            output_file
                .write(
                    result
                        .iter()
                        .map(|x| x.to_string())
                        .collect::<Vec<_>>()
                        .join(" ")
                        .as_bytes(),
                )
                .unwrap();
            output_file.write("\n".as_bytes()).unwrap();
        }
    }

    fn dump_example(&self, idx: usize) -> String {
        return self.inputs[idx]
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(" ");
    }
}
