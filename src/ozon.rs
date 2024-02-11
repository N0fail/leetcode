use std::fs::read_to_string;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub mod contest9;
pub mod contest6;
pub mod contest8;
mod contest7;

pub use contest8 as current;

pub trait Solver{
    fn get_input_filename() -> &'static str {
        "input.txt"
    }
    fn get_expected_filename() -> &'static str {
        "expected.txt"
    }
    fn get_output_filename() -> &'static str {
        "output.txt"
    }
    fn get_compare_filename() -> &'static str {
        "compare.txt"
    }
    fn read_inputs() -> Self;
    fn solve(&mut self);
    fn write_output(&self);
    fn dump_example(&self, idx: usize) -> String;
    fn write_compare(&self) {
        let path = Path::new(Self::get_compare_filename());
        let display = path.display();

        // Open a file in write-only mode, returns `io::Result<File>`
        let mut compare_file = match File::create(&path) {
            Err(why) => panic!("couldn't create {}: {}", display, why),
            Ok(file) => file,
        };

        for (test_num, (output, expected)) in
            read_to_string(Self::get_output_filename()).unwrap().lines()
            .zip(read_to_string(Self::get_expected_filename()).unwrap().lines())
            .enumerate(){
            if output != expected {
                compare_file.write(format!("ERROR: OUTPUT: {} EXPECTED: {}, test #{}\n test:{}\n", output, expected, test_num, self.dump_example(test_num)).as_bytes()).unwrap();
            } else {
                compare_file.write(format!("OK: {} == {}, test #{}\n", output, expected, test_num).as_bytes()).unwrap();
            }
        }

    }
}