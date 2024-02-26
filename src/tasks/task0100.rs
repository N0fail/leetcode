use super::Solver;
use super::{build_tree, TreeNode};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
pub struct Solution {
    p: Option<Rc<RefCell<TreeNode>>>,
    q: Option<Rc<RefCell<TreeNode>>>,
    solution: bool,
}

impl Solution {
    fn is_same_tree(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if p.is_none() && q.is_none() {
            return true;
        }
        if p.is_none() || q.is_none() {
            return false;
        }
        let p = p.as_ref().unwrap().borrow();
        let q = q.as_ref().unwrap().borrow();
        return p.val == q.val
            && Solution::is_same_tree(p.left.clone(), q.left.clone())
            && Solution::is_same_tree(p.right.clone(), q.right.clone());
    }

    // fn is_same_tree(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool {
    //     // also works lol
    //     p == q
    // }
}

impl Solver for Solution {
    fn read_inputs() -> Self {
        return Solution {
            p: build_tree(&vec![1, 2, 3], 0),
            q: build_tree(&vec![1, 2, 3], 0),
            solution: true,
        };
    }

    fn solve(mut self) {
        self.solution = Solution::is_same_tree(self.p, self.q);
        dbg!(self.solution);
    }
}
