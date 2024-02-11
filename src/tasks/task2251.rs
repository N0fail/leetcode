use super::Solver;

#[derive(Debug)]
pub struct Solution{
    flowers: Vec<Vec<i32>>,
    people: Vec<i32>,
    solution: Vec<i32> ,
}

pub fn full_bloom_flowers(flowers: Vec<Vec<i32>>, people: Vec<i32>) -> Vec<i32> {
    let mut starts: Vec<_> = flowers.iter().map(|x| x[0]).collect();
    let mut ends: Vec<_> = flowers.iter().map(|x| x[1]).collect();
    starts.sort();
    ends.sort();
    people.iter().fold(Vec::with_capacity(people.len()),|mut res, x|{
        let amount_starts = starts.partition_point(|start| start<=x);
        let amount_ends = ends.partition_point(|end| end<x);
        res.push((amount_starts - amount_ends)as i32);
        res
    })
}


impl Solver for Solution {
    fn read_inputs() -> Self {
        let x = [[32,36],[20,26],[32,32],[43,46],[40,50],[9,10],[19,19],[2,23],[36,37],[38,48],[13,25],[12,48],[21,33],[4,43],[43,49],[35,46],[41,44],[36,44],[40,50],[42,47],[27,50],[7,43],[5,41],[32,35],[24,31],[33,42],[44,47],[32,46],[39,46],[48,50],[10,49],[14,19],[13,20],[41,43],[39,48],[33,44],[23,37],[34,48],[36,36],[6,12],[14,17],[31,34],[28,40],[11,31],[17,50],[31,47],[17,21],[33,49],[20,29],[27,43],[18,47],[46,47],[29,49],[50,50],[5,24],[19,27],[16,24],[18,42],[5,17],[17,26]];
        return Solution {
            flowers: x.into_iter().fold(Vec::new(), |mut res, x| {res.push(Vec::from(x)); res}),
            people: vec![19,17,42,36,43,42,25,35,31,21,49,14,1,4,24,12,38,48,33,36,37,8,45,50,27,20,45,42,12,5,32,41,16,23,30,29,1,37,16,42,43,5,50,6,49,22,34,24,6],
            solution: vec![],
        };
    }

    fn solve(mut self) {
        self.solution = full_bloom_flowers(self.flowers, self.people);
        dbg!(self.solution);
    }
}
