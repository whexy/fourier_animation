use crate::animation::*;
use std::iter::Iterator;

#[derive(Debug)]
pub struct Input {
    pub path: Vec<Point>,
}

impl From<Vec<u32>> for Input {
    fn from(raw: Vec<u32>) -> Self {
        let path = raw.windows(2)
            .map(|p| Point {
                x: p[0] as f64,
                y: p[1] as f64,
            })
            .collect();
        Input { path }
    }
}

trait Fourier {
    fn calc(input: Input) -> Box<dyn Iterator<Item = Frame>>;
}
