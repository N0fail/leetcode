use super::{build_tree, TreeNode};
use super::{Solver, NULL};
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

#[derive(Debug)]
pub struct Solution {
    root: Option<Rc<RefCell<TreeNode>>>,
    solution: bool,
}

impl Solution {
    // pub fn is_even_odd_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    //     // 32ms
    //     let mut q = Vec::new();
    //     let mut height = 0;
    //     q.push(root.unwrap().clone());
    //     while !q.is_empty() {
    //         let cur_level = q.drain(..).collect::<Vec<_>>();
    //         if height % 2 == 0 {
    //             if cur_level
    //                 .iter()
    //                 .try_fold(-1, |prev, cur| {
    //                     let v = cur.borrow().val;
    //                     if v % 2 == 1 && prev < v {
    //                         Ok(v)
    //                     } else {
    //                         Err(())
    //                     }
    //                 })
    //                 .is_err()
    //             {
    //                 return false;
    //             }
    //         } else {
    //             if cur_level
    //                 .iter()
    //                 .try_fold(1_000_001, |prev, cur| {
    //                     let v = cur.borrow().val;
    //                     if v % 2 == 0 && prev > v {
    //                         Ok(v)
    //                     } else {
    //                         Err(())
    //                     }
    //                 })
    //                 .is_err()
    //             {
    //                 return false;
    //             }
    //         }
    //         for node in cur_level {
    //             if let Some(left) = node.borrow().left.clone() {
    //                 q.push(left);
    //             }
    //             if let Some(right) = node.borrow().right.clone() {
    //                 q.push(right);
    //             }
    //         }
    //         height += 1;
    //     }
    //     return true;
    // }

    pub fn is_even_odd_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        // 22ms
        let mut q = VecDeque::new();
        let mut prev_height = 0;
        let mut prev_value = 0;
        q.push_back((root.unwrap().clone(), 0));
        while let Some((node, height)) = q.pop_front() {
            let node = node.borrow();
            if node.val % 2 == height % 2 {
                return false;
            }
            if prev_height == height {
                if height % 2 == 0 && prev_value >= node.val {
                    return false;
                }
                if height % 2 == 1 && prev_value <= node.val {
                    return false;
                }
            } else {
                prev_height = height
            }
            prev_value = node.val;
            if let Some(left) = node.left.clone() {
                q.push_back((left, height + 1));
            }
            if let Some(right) = node.right.clone() {
                q.push_back((right, height + 1));
            }
        }
        return true;
    }
}

impl Solver for Solution {
    fn read_inputs() -> Self {
        return Solution {
            root: build_tree(&vec![1, 10, 4, 3, NULL, 7, 9, 12, 8, 6, NULL, NULL, 2], 0),
            solution: true,
        };
    }

    fn solve(mut self) {
        self.solution = Solution::is_even_odd_tree(self.root);
        dbg!(self.solution);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ut_1() {
        assert_eq!(
            Solution::is_even_odd_tree(build_tree(&vec![1, 10, 4, 3, NULL, 7, 9, 12, 8, 6, NULL, NULL, 2], 0)),
            true
        );
    }

    #[test]
    fn ut_2() {
        assert_eq!(Solution::is_even_odd_tree(build_tree(&vec![5, 4, 2, 3, 3, 7], 0)), false);
    }

    #[test]
    fn ut_3() {
        assert_eq!(Solution::is_even_odd_tree(build_tree(&vec![5, 9, 1, 3, 5, 7], 0)), false);
    }

    #[test]
    fn ut_4() {
        assert_eq!(
            Solution::is_even_odd_tree(build_tree(&vec![2, 12, 8, 5, 9, NULL, NULL, 18, 16], 0)),
            false
        );
    }
}
