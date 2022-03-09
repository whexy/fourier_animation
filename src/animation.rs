// A frame: circles, vector arrows, trajectory
// circles: x, y, r
// arrows: x, y, x + rcoswt, y + rsinwt
// trajectory: next dot

#[derive(Debug, Copy, Clone)]
struct Circle {
    x: f64,
    y: f64,
    r: f64,
}

#[derive(Debug, Copy, Clone)]
struct Arrow {
    x: f64,
    y: f64,
    x_end: f64,
    y_end: f64,
}

#[derive(Debug, Copy, Clone)]
struct Point {
    x: f64,
    y: f64,
}

#[derive(Debug)]
struct Frame {
    circles: Vec<Circle>,
    arrows: Vec<Arrow>,
    next_point: Point,
}

trait Animation {
    fn next() -> Frame;
}