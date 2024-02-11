// use std::collections::HashSet;
// pub fn want_is_available(have: Vec<String>, want: Vec<String>) -> Vec<i32> {
//     let mut all_unavailable: HashSet<String> = HashSet::new();
//     for login in have {
//         let chars: Vec<_> = login.chars().collect();
//         for idx in 1..chars.len() {
//             let mut new_login = chars.clone();
//             new_login.swap(idx, idx-1);
//             all_unavailable.insert(new_login.iter().collect());
//         }
//         all_unavailable.insert(login);
//     }
//
//     let mut res = Vec::with_capacity(want.len());
//     for login in want {
//         if all_unavailable.contains(&login) {
//             res.push(1)
//         } else {
//             res.push(0)
//         }
//     }
//
//     res
// }
