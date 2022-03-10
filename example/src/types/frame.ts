export type { Frame, Circle, Arrow, Point };

interface Frame {
  circles: Circle[];
  arrows: Arrow[];
  next_point: Point;
}

interface Circle {
  x: number;
  y: number;
  r: number;
}

interface Arrow {
  x: number;
  y: number;
  x_end: number;
  y_end: number;
}

interface Point {
  x: number;
  y: number;
}
