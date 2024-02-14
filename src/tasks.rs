use std::cell::RefCell;
use std::fmt::{Debug, Display, Formatter};
use std::rc::Rc;

pub mod task0542;
pub mod task0518;
pub mod task0521;
pub mod task0538;
pub mod task0336;
pub mod task0378;
pub mod task0232;
pub mod task2731;
pub mod task0150;
pub mod task0931;
pub mod task0739;
pub mod task0222;
pub mod task2966;
pub mod task1291;
pub mod task1657;
pub mod task1043;
pub mod task1095;
pub mod task0076;
pub mod task0915;
pub mod task0881;
pub mod task0845;
pub mod task0820;
pub mod task0833;
pub mod task0806;
pub mod task0879;
pub mod task0387;
pub mod task1529;
pub mod task1535;
pub mod task1477;
pub mod task0049;
pub mod task1980;
pub mod task0451;
pub mod task1365;
pub mod task1456;
pub mod task0279;
pub mod task0301;
pub mod task0368;
pub mod task1347;
pub mod task0907;
pub mod task1793;
pub mod task0647;
pub mod task1846;
pub mod task0242;
pub mod task2251;
pub mod task0169;
pub mod task2785;
pub mod task2108;
pub mod task1503;
pub mod task0198;
pub mod task0661;
pub mod task2149;
pub mod task2535;
pub mod task2498;

pub use task2498 as current;

pub trait Solver{
    fn read_inputs() -> Self;
    fn solve(self);
}


pub const NULL: i32 = -1_000_000;
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None
        }
    }

    pub fn to_vec(&self, my_idx: usize, output: & mut Vec<i32>) {
        if output.len() <= my_idx {
            output.resize(my_idx+1, NULL)
        }
        output[my_idx] = self.val;
        if let Some(left) = &self.left {
            left.borrow().to_vec(my_idx*2+1, output);
        }
        if let Some(right) = &self.right {
            right.borrow().to_vec(my_idx*2+2, output);
        }
    }
}

impl Display for TreeNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut result: Vec<i32> =  vec![];
        self.to_vec(0, & mut result);
        let mut print_res: Vec<String> = vec![];
        print_res.reserve(result.len());
        for value in &result {
            if *value == NULL {
                print_res.push(String::from("NULL"));
            } else {
                print_res.push(value.to_string());
            }
        }
        print_res.fmt(f)
    }
}


pub fn build_tree(input: &Vec<i32>, cur_idx: usize) -> Option<Rc<RefCell<TreeNode>>> {
    if cur_idx >= input.len() {
        return None;
    }
    if input[cur_idx] == NULL {
        return None;
    }

    let new_node = create_node(input[cur_idx]);
    new_node.borrow_mut().left = build_tree(input, cur_idx*2 + 1);
    new_node.borrow_mut().right = build_tree(input, cur_idx*2 + 2);
    return Some(new_node);
}

pub fn create_node(val: i32) -> Rc<RefCell<TreeNode>> {
    Rc::new(RefCell::new(TreeNode::new(val)))
}
