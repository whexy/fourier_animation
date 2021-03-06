use super::*;
use crate::fourier::Input;

struct Vector(f64, f64);

impl Vector {
    fn next(&mut self, level: f64) {
        let r = self.r();
        let x = (1.0 / r * level).cos() * self.0 - (1.0 / r * level).sin() * self.1;
        let y = (1.0 / r * level).sin() * self.0 + (1.0 / r * level).cos() * self.1;
        self.0 = x;
        self.1 = y;
    }

    fn r(&self) -> f64 {
        (self.0 * self.0 + self.1 * self.1).sqrt()
    }
}

impl Into<Point> for Vector {
    fn into(self) -> Point {
        Point {
            x: self.0,
            y: self.1,
        }
    }
}

pub struct NaiveAnimation {
    input: Input,
    arc: Vec<Vector>,
}

impl NaiveAnimation {
    pub fn new(input: Input) -> Self {
        let arc = input.path.iter().map(|p| Vector(p.x, p.y)).collect();
        Self { input, arc }
    }

    fn _generate_next_frame(&mut self) -> Frame {
        let mut cursor = Vector(0.0, 0.0);
        let mut circles = Vec::<Circle>::new();
        let mut arrows = Vec::<Arrow>::new();

        let mut level = 1.0;
        for v in &mut self.arc.iter_mut() {
            v.next(level);
            level += 1.0;

            circles.push(Circle {
                x: cursor.0 + 200.0,
                y: cursor.1 + 200.0,
                r: v.r(),
            });
            arrows.push(Arrow {
                x: cursor.0 + 200.0,
                y: cursor.1 + 200.0,
                x_end: cursor.0 + v.0 + 200.0,
                y_end: cursor.1 + v.1 + 200.0,
            });

            // update cursor
            cursor = Vector(cursor.0 + v.0, cursor.1 + v.1);
        }

        cursor = Vector(cursor.0 + 200.0, cursor.1 + 200.0);

        Frame {
            circles,
            arrows,
            next_point: cursor.into(),
        }
    }
}

impl Animation for NaiveAnimation {
    fn next(&mut self) -> Frame {
        self._generate_next_frame()
    }
}
