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
    pub fn is_schema_valid(root: Option<Rc<RefCell<TreeNode>>>, schema: i32, mut height: usize) -> bool {
        if let Some(r) = root {
            if height == 0 {
                return true;
            }

            height -= 1;
            let cur_bit = 1 << height;
            if cur_bit & schema != 0 {
                return Solution::is_schema_valid(r.borrow().right.clone(), schema, height);
            }
            return Solution::is_schema_valid(r.borrow().left.clone(), schema, height);
        }
        return false;
    }

    pub fn count_height(root: Option<Rc<RefCell<TreeNode>>>) -> usize {
        if let Some(r) = root {
            return 1 + Solution::count_height(r.borrow().left.clone());
        }
        return 0;
    }

    pub fn count_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if root.is_none() {
            return 0;
        }
        let mut height = Solution::count_height(root.clone());
        height -= 1;
        let mut r_schema = (1 << height) - 1;
        let mut l_schema = 0;
        while l_schema < r_schema {
            let mid = (l_schema + r_schema + 1) / 2;
            if Solution::is_schema_valid(root.clone(), mid, height) {
                l_schema = mid;
            } else {
                r_schema = mid - 1;
            }
        }
        return l_schema + (1 << height);
    }
}

impl Solver for Solution {
    fn read_inputs() -> Self {
        let input = vec![1; 529872];
        return Solution {
            root: build_tree(&input, 0),
            solution: 0,
        };
    }

    fn solve(mut self) {
        self.solution = Solution::count_nodes(self.root);
        dbg!(self.solution);
    }
}
