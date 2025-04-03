#![allow(clippy::all)]

use core::f64;
pub struct Web {
    pub s: Vec<Vec<usize>>,
    pub r: Vec<f64>
}

impl Web {
    pub fn rank(self, i: usize, d: f64) -> Vec<f64>{
        // page rank calculation in one line, 143 characters
(0..i).fold(self.r,|z,_|(0..z.len()).map(|q|self.s.iter().zip(&z).fold(1.-d,|c,(s,r)|if s.contains(&q){c+r*d/s.len()as f64}else{c})).collect())
    }
    pub fn rank_explained(self, i: usize, d: f64) -> Vec<f64> {

(0..i)  // we use 0..i while ignoring the actual value to do it the number of iterations
    .fold(self.r, |z, _| // Fold over the range (0..i) starting with the initial value self.r (the rank vec) storing the current rank vec in z
        (0..z.len()).map(|q| // iterate through each site id, storing the current id in q
            self.s.iter().zip(&z) // iterate through all the sites and their ranks
                // Fold over the zipped iterator, starting with an initial value of (1 - d)
                .fold(1. - d, |c, (s, r)| 
                    if s.contains(&q) { // if the site contains the current site id in its links
                        c + r * d / s.len() as f64 // add its pagerank / links * d
                    } else {
                        c // otherwise dont add anything 
                    }
                )
        ).collect() // collect the results of this mapping into the new vec of z 
    )

    }
}


fn main() {
    let web = Web {
        s: vec![vec![1,2], vec![2], vec![0], vec![2]],
        r: vec![0.25, 0.25, 0.25, 0.25]
    };
        let time = std::time::Instant::now();
    println!("{:?}", web.rank(35, 0.85));
    println!("time {:?}", time.elapsed());
}
