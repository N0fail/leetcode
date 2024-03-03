use super::Solver;
use super::{build_list, ListNode};

#[derive(Debug)]
pub struct Solution {
    head: Option<Box<ListNode>>,
    n: i32,
    solution: Option<Box<ListNode>>,
}

impl Solution {
    pub fn remove_nth_from_end(mut head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        // 2 pass
        let mut elems_count = 0;
        let mut p = &head;
        loop {
            match &p {
                None => break,
                Some(node) => {
                    p = &node.next;
                    elems_count += 1;
                }
            }
        }
        let idx_to_remove = elems_count - n;
        if idx_to_remove == 0 {
            return head.unwrap().next;
        }

        let mut p = &mut head;
        for _ in 1..idx_to_remove {
            p = &mut p.as_mut().unwrap().next;
        }

        p.as_mut().unwrap().next = p.as_ref().unwrap().next.as_ref().unwrap().next.clone();
        head
    }

    // pub fn remove_nth_from_end(mut head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
    //     // 1 pass
    //     // borrow checker won't allow this, idk
    //     let mut l = &mut head;
    //     let mut r = &head.clone();
    //
    //     for _ in 0..n {
    //         r = &r.as_ref().unwrap().next;
    //     }
    //     while r.is_some() {
    //         r = &r.as_ref().unwrap().next;
    //         l = &mut l.as_mut().unwrap().next;
    //     }
    //
    //     l.as_mut().unwrap().next = l.as_ref().unwrap().next.as_ref().unwrap().next.clone();
    //     head
    // }
}

impl Solver for Solution {
    fn read_inputs() -> Self {
        return Solution {
            head: build_list(&vec![1, 2, 3, 4, 5]),
            n: 2,
            solution: build_list(&vec![1, 2, 3, 5]),
        };
    }

    fn solve(mut self) {
        self.solution = Solution::remove_nth_from_end(self.head, self.n);
        dbg!(self.solution);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ut_1() {
        assert_eq!(
            Solution::remove_nth_from_end(build_list(&vec![1, 2, 3, 4, 5]), 2),
            build_list(&vec![1, 2, 3, 5])
        );
    }

    #[test]
    fn ut_2() {
        assert_eq!(Solution::remove_nth_from_end(build_list(&vec![1]), 1), build_list(&vec![]));
    }

    #[test]
    fn ut_3() {
        assert_eq!(
            Solution::remove_nth_from_end(build_list(&vec![1, 2]), 1),
            build_list(&vec![1])
        );
    }
}
