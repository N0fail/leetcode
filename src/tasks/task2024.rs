use super::Solver;

#[derive(Debug)]
pub struct Solution {
    answer_key: String,
    k: i32,
    solution: i32,
}

impl Solution {
    // fn count_for_byte(s: &[u8], byte: u8, mut k: i32) -> i32 {
    //     // 2 pass 6-9ms
    //     s.iter().enumerate().fold((0, 1), |(mut l, best), (r, b)| {
    //         if *b != byte {
    //             k -= 1;
    //             if k < 0 {
    //                 l += s.iter().skip(l).position(|b| *b != byte).unwrap()+1;
    //             }
    //         }
    //         (l, best.max(r+1-l))
    //     }).1 as _
    // }
    //
    // pub fn max_consecutive_answers(answer_key: String, k: i32) -> i32 {
    //     Self::count_for_byte(answer_key.as_bytes(), b'T', k).max(Self::count_for_byte(answer_key.as_bytes(), b'F', k))
    // }

    pub fn max_consecutive_answers(answer_key: String, k: i32) -> i32 {
        // 2ms one pass
        // f - кол-во F в текущем окне
        // t - кол-во T в текущем окне
        // min(f,t) <= k - ограничение
        // max(f,t) среди всех валидных окон - это ответ
        // путь у нас найдено лучшее окно размера best
        // пусть у нас сейчас какое то окно размера best и мы добавляем новый символ справа, то
        // если min(f,t) <= k, то у нас получилось валидно окно размером больше best, запоминаем новый best
        // если min(f,t) >  k, то у нас получилось невалидное окно, выкидываем первый символ из него
        // тк нас интересуют только окна размером не меньше best, то делать его валидным сейчас не нужно
        // возможно оно станет таковым позже, тогда мы и запомним результат
        let answer_key = answer_key.as_bytes();
        answer_key
            .iter()
            .enumerate()
            .fold((0, 0, 0), |(mut t, mut f, best), (idx, b)| {
                if *b == b'F' {
                    f += 1;
                } else {
                    t += 1;
                }
                if t.min(f) <= k {
                    (t, f, best + 1)
                } else {
                    if answer_key[idx - best] == b'F' {
                        f -= 1;
                    } else {
                        t -= 1;
                    }
                    (t, f, best)
                }
            })
            .2 as _
    }

    // pub fn max_consecutive_answers(answer_key: String, k: i32) -> i32 {
    //     // some bs to get 0ms
    //     unsafe {
    //         let mut counts = [0; 256];
    //         let f_idx = b'F' as usize;
    //         let t_idx = b'T' as usize;
    //         let answer_key = answer_key.into_bytes();
    //         (answer_key.len() - answer_key.iter().fold(0, |w_start, b| {
    //             *counts.get_unchecked_mut(*b as usize) += 1;
    //             if counts[f_idx].min(counts[t_idx]) > k {
    //                 *counts.get_unchecked_mut(answer_key[w_start] as usize) -= 1;
    //                 w_start + 1
    //             } else {
    //                 w_start
    //             }
    //         })) as _
    //     }
    // }
}

impl Solver for Solution {
    fn read_inputs() -> Self {
        return Solution {
            answer_key: "FFFTFFTF".to_string(),
            k: 1,
            solution: 6,
        };
    }

    fn solve(mut self) {
        self.solution = Solution::max_consecutive_answers(self.answer_key, self.k);
        dbg!(self.solution);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ut_1() {
        assert_eq!(Solution::max_consecutive_answers("TTFF".to_string(), 2), 4);
    }

    #[test]
    fn ut_2() {
        assert_eq!(Solution::max_consecutive_answers("TFFT".to_string(), 1), 3);
    }

    #[test]
    fn ut_3() {
        assert_eq!(Solution::max_consecutive_answers("TTFTTFTT".to_string(), 1), 5);
    }

    #[test]
    fn ut_4() {
        assert_eq!(Solution::max_consecutive_answers("TTFTTTTTFT".to_string(), 1), 8);
    }

    #[test]
    fn ut_5() {
        assert_eq!(Solution::max_consecutive_answers("FFFTTFTTFT".to_string(), 3), 8);
    }

    #[test]
    fn ut_6() {
        assert_eq!(Solution::max_consecutive_answers("FFFTFFTF".to_string(), 1), 6);
    }

    #[test]
    fn ut_7() {
        assert_eq!(Solution::max_consecutive_answers("FFFTFFTF".to_string(), 1), 6);
    }
}
