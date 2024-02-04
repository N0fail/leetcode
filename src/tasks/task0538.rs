use super::Solver;
use std::rc::Rc;
use std::cell::RefCell;
use std::ops::Deref;
use super::{TreeNode, build_tree, NULL};

#[derive(Debug)]
pub struct Solution{
    root: Option<Rc<RefCell<TreeNode>>>,
    solution: Option<Rc<RefCell<TreeNode>>>,
}

impl Solution {
    pub fn dfs(root: &Option<Rc<RefCell<TreeNode>>>, sum: &mut i32) {
        if let Some(root_ref) = root {
            Self::dfs(&root_ref.borrow().right, sum);
            *sum += root_ref.borrow().val;
            root_ref.borrow_mut().val = *sum;
            Self::dfs(&root_ref.borrow().left, sum);
        }
    }

    pub fn convert_bst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        Self::dfs(&root, & mut 0);
        return root
    }
}

impl Solver for Solution {
    fn read_inputs() -> Self {
        let input = vec![4,1,6,0,2,5,7,NULL,NULL,NULL,3,NULL,NULL,NULL,8];
        return Solution {
            root: build_tree(&input, 0),
            solution: None,
        };
    }

    fn solve(mut self) {
        self.solution = Solution::convert_bst(self.root);
        if let Some(solution) = self.solution {
            println!("{}",solution.borrow().deref());
        }
    }
}