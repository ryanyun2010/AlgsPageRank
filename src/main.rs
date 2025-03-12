const DAMPING: f64 = 0.85;


pub struct Web {
    pub sites: Vec<Site>,
    pub ranks: Vec<f64>
}

pub struct Site {
    pub links: Vec<usize>
}

impl Web {
    pub fn rank(&mut self, iterations: usize) -> &Vec<f64>{
        // ive tried to do this all in one line without using a semi-colon, so in a way this is 0 lines 
            (0..iterations).map(|_| (0..self.sites.len()).map(|site| self.ranks[site] = (1.0 - DAMPING) + DAMPING * self.sites.iter().enumerate().map(|(i, x)| if x.links.contains(&site) {self.ranks[i]/x.links.len() as f64}else{0.0}).sum::<f64>()).last()).last().map(|_| &self.ranks).unwrap()
    }
}





fn main() {
    let mut web = Web {
        sites: vec![Site {
            links: vec![1,2]
        }, Site{
            links: vec![2]
        }, Site{
            links: vec![0]
        }, Site{
            links: vec![2]
        }],
        ranks: vec![0.25, 0.25, 0.25, 0.25]
    };
    println!("{:?}", web.rank(35));
}
