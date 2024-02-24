use super::Solver;

#[derive(Debug)]
pub struct Solution {
    n: i32,
    left_child: Vec<i32>,
    right_child: Vec<i32>,
    solution: bool,
}

// pub fn dfs(idx: usize, left_child: &Vec<i32>, right_child: &Vec<i32>, visited: &mut Vec<bool>) {
//     if visited[idx] {
//         return;
//     }
//
//     visited[idx] = true;
//     if left_child[idx] != -1 {
//         dfs(left_child[idx] as usize, left_child, right_child, visited);
//     }
//     if right_child[idx] != -1 {
//         dfs(right_child[idx] as usize, left_child, right_child, visited);
//     }
// }
//
// pub fn validate_binary_tree_nodes(n: i32, left_child: Vec<i32>, right_child: Vec<i32>) -> bool {
//     // 3-6ms 3Mb
//     let mut parents = vec![None; n as usize];
//
//     for (parent, c) in left_child.iter().enumerate() {
//         if *c == -1 {
//             continue;
//         }
//         parents[*c as usize] = Some(parent);
//     }
//
//     for (parent, c) in right_child.iter().enumerate() {
//         if *c == -1 {
//             continue;
//         }
//         match parents[*c as usize] {
//             Some(_) => return false,
//             None => parents[*c as usize] = Some(parent),
//         }
//     }
//
//     let prob_roots = parents.iter().enumerate().filter(|(_,parent)| parent.is_none()).map(|(idx, _)| idx).collect::<Vec<_>>();
//     if prob_roots.len() != 1 {
//         return false
//     }
//
//     if n == 1 {
//         return true
//     }
//
//     let root = prob_roots[0];
//     let mut visited = vec![false; n as usize];
//     dfs(root, &left_child, &right_child, &mut visited);
//
//     visited.iter().all(|x| *x)
// }

pub fn dfs(idx: i32, left_child: &Vec<i32>, right_child: &Vec<i32>, visited: &mut Vec<bool>) -> bool {
    if idx == -1 {
        return true;
    }

    let idx = idx as usize;

    if visited[idx] {
        return false;
    }

    visited[idx] = true;
    dfs(left_child[idx], left_child, right_child, visited) && dfs(right_child[idx], left_child, right_child, visited)
}

pub fn validate_binary_tree_nodes(n: i32, left_child: Vec<i32>, right_child: Vec<i32>) -> bool {
    // 0-6ms 3Mb
    let mut is_root = vec![true; n as usize];
    left_child
        .iter()
        .chain(right_child.iter())
        .filter(|c| **c != -1)
        .for_each(|c| is_root[*c as usize] = false);

    let root = is_root.iter().position(|x| *x);
    if root.is_none() {
        return false;
    }
    // reuse vector as visited tracker
    is_root.iter_mut().for_each(|x| *x = false);
    dfs(root.unwrap() as i32, &left_child, &right_child, &mut is_root) && is_root.iter().all(|x| *x)
}

impl Solver for Solution {
    fn read_inputs() -> Self {
        return Solution {
            n: 4,
            left_child: vec![1, 0, 3, -1],
            //left_child: vec![1, 2, 0, -1],
            right_child: vec![-1, -1, -1, -1],
            solution: false,
        };
    }

    fn solve(mut self) {
        self.solution = validate_binary_tree_nodes(self.n, self.left_child, self.right_child);
        dbg!(self.solution);
    }
}
