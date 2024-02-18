use super::Solver;
use std::fs::read_to_string;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

#[derive(Debug)]
pub struct Solution {
    inputs: Vec<String>,
    results: Vec<String>,
}

pub fn is_possible(_events: String) -> bool {
    return true;
}

impl Solver for Solution {
    fn read_inputs() -> Self {
        let mut inputs: Vec<String> = Vec::new();
        for (_, line) in read_to_string(Self::get_input_filename())
            .unwrap()
            .lines()
            .skip(1)
            .enumerate()
            .filter(|(idx, _)| idx % 2 != 0)
        {
            inputs.push(line.to_string())
        }

        return Solution { inputs, results: vec![] };
    }

    fn solve(&mut self) {
        for input in &self.inputs {
            if is_possible(input.clone()) {
                self.results.push("Yes\n".to_string());
            } else {
                self.results.push("No\n".to_string());
            }
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
            output_file.write(result.as_bytes()).unwrap();
        }
    }

    fn dump_example(&self, idx: usize) -> String {
        return self.inputs[idx].clone();
    }
}
