use crate::animation::*;
use std::iter::Iterator;

pub struct Input {
    pub path: Vec<Point>
}

impl From<Vec<u32>> for Input {
    fn from(_: Vec<u32>) -> Self {
        todo!()
    }
}

trait Fourier {
    fn calc(input: Input) -> Box<dyn Iterator<Item=Frame>>;
}