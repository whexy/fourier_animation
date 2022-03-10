// A frame: circles, vector arrows, trajectory
// circles: x, y, r
// arrows: x, y, x + rcoswt, y + rsinwt
// trajectory: next dot

pub mod naive_animation;

use serde::Serialize;
use wasm_bindgen::prelude::*;

#[derive(Debug, Clone, Serialize)]
pub struct Frame {
    pub circles: Vec<Circle>,
    pub arrows: Vec<Arrow>,
    pub next_point: Point,
}

#[derive(Debug, Copy, Clone, Serialize)]
pub struct Circle {
    pub x: f64,
    pub y: f64,
    pub r: f64,
}

#[derive(Debug, Copy, Clone, Serialize)]
pub struct Arrow {
    pub x: f64,
    pub y: f64,
    pub x_end: f64,
    pub y_end: f64,
}

#[derive(Debug, Copy, Clone, Serialize)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

pub trait Animation {
    fn next(&mut self) -> Frame; // invoked by JavaScript, must be stateful (&mut self).
}
