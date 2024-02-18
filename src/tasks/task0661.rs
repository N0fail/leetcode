use super::Solver;

#[derive(Debug)]
pub struct Solution {
    img: Vec<Vec<i32>>,
    solution: Vec<Vec<i32>>,
}

pub fn image_smoother(mut img: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let n = img.len();
    let m = img[0].len();
    for i in 0..n {
        let mut prev = 0;
        let row = &mut img[i];
        for j in 0..m - 1 {
            let save = row[j];
            row[j] += row[j + 1] + prev;
            prev = save;
        }
        img[i][m - 1] += prev;
    }

    for j in 0..m {
        let mut prev = 0;
        for i in 0..n - 1 {
            let save = img[i][j];
            img[i][j] += img[i + 1][j] + prev;
            prev = save;
        }
        img[n - 1][j] += prev;
    }

    if n != 1 && m != 1 {
        img[0][0] /= 4;
        img[0][m - 1] /= 4;
        img[n - 1][0] /= 4;
        img[n - 1][m - 1] /= 4;
        for i in 1..n - 1 {
            img[i][0] /= 6;
            img[i][m - 1] /= 6;
        }
        for j in 1..m - 1 {
            img[n - 1][j] /= 6;
            img[0][j] /= 6;
        }
        for i in 1..n - 1 {
            for j in 1..m - 1 {
                img[i][j] /= 9;
            }
        }
    } else if n != 1 {
        // m == 1
        img[0][0] /= 2;
        img[n - 1][0] /= 2;
        for i in 1..n - 1 {
            img[i][0] /= 3;
        }
    } else if m != 1 {
        // n == 1
        img[0][0] /= 2;
        img[0][m - 1] /= 2;
        for j in 1..m - 1 {
            img[0][j] /= 3;
        }
    }

    img
}

impl Solver for Solution {
    fn read_inputs() -> Self {
        return Solution {
            img: [[1, 1, 1], [1, 0, 1], [1, 1, 1]].into_iter().map(|x| Vec::from(x)).collect(),
            solution: vec![],
        };
    }

    fn solve(mut self) {
        self.solution = image_smoother(self.img);
        dbg!(self.solution);
    }
}
