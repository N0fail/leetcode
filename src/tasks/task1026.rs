use super::Solver;
use super::{build_tree, TreeNode, NULL};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
pub struct Solution {
    root: Option<Rc<RefCell<TreeNode>>>,
    solution: i32,
}

fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, cur_max: i32, cur_min: i32) -> i32 {
    match node {
        None => -1,
        Some(node) => {
            let node = node.borrow();
            return (node.val - cur_max)
                .abs()
                .max((node.val - cur_min).abs())
                .max(dfs(&node.left, cur_max.max(node.val), cur_min.min(node.val)))
                .max(dfs(&node.right, cur_max.max(node.val), cur_min.min(node.val)));
        }
    }
}

pub fn max_ancestor_diff(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    if let Some(root) = root {
        dfs(&Some(root.clone()), root.borrow().val, root.borrow().val)
    } else {
        -1
    }
}

impl Solver for Solution {
    fn read_inputs() -> Self {
        return Solution {
            root: build_tree(&vec![8, 3, 10, 1, 6, NULL, 14, NULL, NULL, 4, 7, 13], 0),
            solution: -1,
        };
    }

    fn solve(mut self) {
        self.solution = max_ancestor_diff(self.root);
        dbg!(self.solution);
    }
}
