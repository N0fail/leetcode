use super::Solver;
use super::{build_tree, TreeNode};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
pub struct Solution {
    root: Option<Rc<RefCell<TreeNode>>>,
    solution: i32,
}

impl Solution {
    pub fn rec(root: Option<Rc<RefCell<TreeNode>>>) -> (i32, i32) {
        match root {
            None => (-1, 0),
            Some(root) => {
                let (left_height, left_diameter) = Self::rec(root.borrow().left.clone());
                let (right_height, right_diameter) = Self::rec(root.borrow().right.clone());
                let diameter = left_height + right_height + 2;
                let height = left_height.max(right_height) + 1;
                (height, left_diameter.max(right_diameter).max(diameter))
            }
        }
    }

    pub fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::rec(root).1
    }
}

impl Solver for Solution {
    fn read_inputs() -> Self {
        return Solution {
            root: build_tree(&vec![1, 2, 3, 4, 5], 0),
            solution: 0,
        };
    }

    fn solve(mut self) {
        self.solution = Solution::diameter_of_binary_tree(self.root);
        dbg!(self.solution);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ut_1() {
        assert_eq!(Solution::diameter_of_binary_tree(build_tree(&vec![1, 2, 3, 4, 5], 0)), 3);
    }

    #[test]
    fn ut_2() {
        assert_eq!(Solution::diameter_of_binary_tree(build_tree(&vec![1, 2], 0)), 1);
    }
}
