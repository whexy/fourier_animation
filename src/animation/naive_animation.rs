use super::*;

pub struct NaiveAnimation {
    frame: Option<Frame>,
}

impl NaiveAnimation {
    pub fn new() -> Self {
        Self { frame: None }
    }

    fn _generate_next_frame(&mut self) {
        if let Some(current_frame) = self.frame.take() {
            let current_point = current_frame.next_point;
            let next_point = Point {
                x: current_point.x + 3.0,
                y: current_point.y + 4.0,
            };
            let circles = vec![Circle {
                x: 0.0,
                y: 0.0,
                r: (next_point.x * next_point.x + next_point.y * next_point.y).sqrt(),
            }];
            let arrows = vec![Arrow {
                x: 0.0,
                y: 0.0,
                x_end: next_point.x,
                y_end: next_point.y,
            }];
            self.frame = Some(Frame {
                circles: circles,
                arrows: arrows,
                next_point: next_point,
            });
        } else {
            self.frame = Some(Frame {
                circles: vec![Circle {
                    x: 0.0,
                    y: 0.0,
                    r: 0.0,
                }],
                arrows: vec![Arrow {
                    x: 0.0,
                    y: 0.0,
                    x_end: 0.0,
                    y_end: 0.0,
                }],
                next_point: Point { x: 0.0, y: 0.0 },
            });
        }
    }
}

impl Animation for NaiveAnimation {
    fn next(&mut self) -> Frame {
        self._generate_next_frame();
        self.frame.as_ref().unwrap().clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next() {
        let mut animation = NaiveAnimation {
            frame: Some(Frame {
                circles: vec![Circle {
                    x: 0.0,
                    y: 0.0,
                    r: 0.0,
                }],
                arrows: vec![Arrow {
                    x: 0.0,
                    y: 0.0,
                    x_end: 0.0,
                    y_end: 0.0,
                }],
                next_point: Point { x: 0.0, y: 0.0 },
            }),
        };
        let frame = animation.next();
        println!("{:?}", frame);
        let frame = animation.next();
        println!("{:?}", frame);
        let frame = animation.next();
        println!("{:?}", frame);
    }
}
