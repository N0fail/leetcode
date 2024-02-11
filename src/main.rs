// use leetcode::tasks::Solver;
// use leetcode::tasks::current::Solution;
//
// fn main() {
//     Solution::read_inputs().solve();
// }

use leetcode::ozon::current::Solution;
use leetcode::ozon::Solver;
fn main() {
    let mut solution = Solution::read_inputs();
    solution.solve();
    solution.write_output();
    solution.write_compare();
}
