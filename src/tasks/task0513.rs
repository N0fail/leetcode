use super::{build_tree, TreeNode};
use super::{Solver, NULL};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
pub struct Solution {
    root: Option<Rc<RefCell<TreeNode>>>,
    solution: i32,
}

impl Solution {
    pub fn rec(root: Option<Rc<RefCell<TreeNode>>>) -> (i32, Option<i32>) {
        match root {
            None => (-1, None),
            Some(root) => {
                let (left_height, left_value) = Self::rec(root.borrow().left.clone());
                let (right_height, right_value) = Self::rec(root.borrow().right.clone());
                match (left_value, right_value) {
                    (None, None) => (0, Some(root.borrow().val)),
                    (Some(v), None) => (left_height + 1, Some(v)),
                    (None, Some(v)) => (right_height + 1, Some(v)),
                    (Some(l), Some(r)) => {
                        if left_height >= right_height {
                            (left_height + 1, Some(l))
                        } else {
                            (right_height + 1, Some(r))
                        }
                    }
                }
            }
        }
    }

    pub fn find_bottom_left_value(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::rec(root).1.unwrap()
    }
}

impl Solver for Solution {
    fn read_inputs() -> Self {
        return Solution {
            root: build_tree(&vec![1, 2, 3, 4, NULL, 5, 6, NULL, NULL, NULL, NULL, 7], 0),
            solution: 7,
        };
    }

    fn solve(mut self) {
        self.solution = Solution::find_bottom_left_value(self.root);
        dbg!(self.solution);
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::tasks::NULL;

    #[test]
    fn ut_1() {
        assert_eq!(Solution::find_bottom_left_value(build_tree(&vec![2, 1, 3], 0)), 1);
    }

    #[test]
    fn ut_2() {
        assert_eq!(
            Solution::find_bottom_left_value(build_tree(&vec![1, 2, 3, 4, NULL, 5, 6, NULL, NULL, NULL, NULL, 7], 0)),
            7
        );
    }
}
