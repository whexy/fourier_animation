use crate::animation::*;
use std::iter::Iterator;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Input {
    pub path: Vec<Point>,
}

impl From<Vec<f64>> for Input {
    fn from(raw: Vec<f64>) -> Self {
        let path = raw.chunks(2)
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
