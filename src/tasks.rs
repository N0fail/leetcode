use std::cell::RefCell;
use std::fmt::{Debug, Display, Formatter};
use std::rc::Rc;

pub mod task0019;
pub mod task0049;
pub mod task0076;
pub mod task0100;
pub mod task0119;
pub mod task0150;
pub mod task0169;
pub mod task0198;
pub mod task0201;
pub mod task0222;
pub mod task0232;
pub mod task0242;
pub mod task0268;
pub mod task0273;
pub mod task0279;
pub mod task0301;
pub mod task0336;
pub mod task0368;
pub mod task0378;
pub mod task0387;
pub mod task0392;
pub mod task0451;
pub mod task0513;
pub mod task0518;
pub mod task0521;
pub mod task0538;
pub mod task0542;
pub mod task0543;
pub mod task0557;
pub mod task0647;
pub mod task0661;
pub mod task0739;
pub mod task0787;
pub mod task0806;
pub mod task0815;
pub mod task0820;
pub mod task0833;
pub mod task0845;
pub mod task0879;
pub mod task0881;
pub mod task0905;
pub mod task0907;
pub mod task0915;
pub mod task0931;
pub mod task0977;
pub mod task0997;
pub mod task1026;
pub mod task1043;
pub mod task1048;
pub mod task1095;
pub mod task1291;
pub mod task1347;
pub mod task1361;
pub mod task1365;
pub mod task1422;
pub mod task1425;
pub mod task1441;
pub mod task1456;
mod task1464;
pub mod task1477;
pub mod task1481;
pub mod task1503;
pub mod task1529;
pub mod task1535;
pub mod task1561;
pub mod task1578;
pub mod task1609;
pub mod task1630;
pub mod task1642;
pub mod task1657;
pub mod task1685;
pub mod task1704;
pub mod task1770;
pub mod task1793;
pub mod task1838;
pub mod task1846;
pub mod task1877;
pub mod task1881;
pub mod task1897;
pub mod task1930;
pub mod task1980;
pub mod task2024;
pub mod task2085;
pub mod task2092;
pub mod task2108;
pub mod task2149;
pub mod task2251;
pub mod task2264;
pub mod task2391;
pub mod task2402;
pub mod task2442;
pub mod task2498;
pub mod task2535;
pub mod task2709;
pub mod task2731;
pub mod task2785;
pub mod task2808;
pub mod task2849;
pub mod task2870;
pub mod task2966;
pub mod task2971;

pub use task0019 as current;

pub trait Solver {
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
            right: None,
        }
    }

    pub fn to_vec(&self, my_idx: usize, output: &mut Vec<i32>) {
        if output.len() <= my_idx {
            output.resize(my_idx + 1, NULL)
        }
        output[my_idx] = self.val;
        if let Some(left) = &self.left {
            left.borrow().to_vec(my_idx * 2 + 1, output);
        }
        if let Some(right) = &self.right {
            right.borrow().to_vec(my_idx * 2 + 2, output);
        }
    }
}

impl Display for TreeNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut result: Vec<i32> = vec![];
        self.to_vec(0, &mut result);
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

    let new_node = create_tree_node(input[cur_idx]);
    new_node.borrow_mut().left = build_tree(input, cur_idx * 2 + 1);
    new_node.borrow_mut().right = build_tree(input, cur_idx * 2 + 2);
    return Some(new_node);
}

pub fn create_tree_node(val: i32) -> Rc<RefCell<TreeNode>> {
    Rc::new(RefCell::new(TreeNode::new(val)))
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
    pub fn to_vec(&self) -> Vec<i32> {
        let mut res = Vec::new();
        res.push(self.val);
        let mut p = self;
        loop {
            match &p.next {
                None => break,
                Some(node) => p = node.as_ref(),
            }
            res.push(p.val)
        }
        res
    }
}

impl Display for ListNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.to_vec().fmt(f)
    }
}

pub fn build_list(input: &Vec<i32>) -> Option<Box<ListNode>> {
    if input.len() == 0 {
        return None;
    }
    let mut head = ListNode::new(input[0]);
    let mut p = &mut head;
    for v in &input[1..] {
        let node = ListNode::new(*v);
        p.next = Some(Box::new(node));
        p = p.next.as_mut().unwrap();
    }
    Some(Box::new(head))
}
