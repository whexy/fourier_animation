import { FunctionalComponent, h } from "preact";
import style from "./style.css";

import { useEffect, useState, useRef } from "preact/hooks";
import { create, next } from "fourier-animation";

import type { Frame, Point } from "../../types/frame";

const Home: FunctionalComponent = () => {
  let canvasRef = useRef(null);

  const [pause, setPause] = useState(true);
  const [t, setT] = useState<Point[]>([]);

  useEffect(() => {
    create(0, Float64Array.from([100, 0, 70, 0, 5, 0])); // cabbage-like
    // create(0, Float64Array.from([100, 0, 30, 0, 10, 0])); // chrysanthemum-like
  }, []);

  useEffect(() => {
    if (!pause) {
      let canvas = canvasRef.current as unknown as HTMLCanvasElement;
      let trajectory: Point[] = t;
      let loop = setInterval(() => {
        let frame = next(0) as Frame;
        trajectory.push(frame.next_point);
        _goNext(canvas, frame!, trajectory);
      }, 1000 / 120);
      return () => {
        clearInterval(loop);
        setT(trajectory);
      };
    }
  }, [pause]);

  return (
    <div class={style.home}>
      <h1>Fourier Animation Example</h1>
      <div>
        <p>{pause ? "Pause" : "Play"}</p>
        <button
          onClick={() => {
            setPause(!pause);
          }}
        >
          Go
        </button>
      </div>
      <canvas width={800} height={800} ref={canvasRef}></canvas>
    </div>
  );
};

const _goNext = (
  canvasRef: HTMLCanvasElement,
  frame: Frame,
  trajectory: Point[]
) => {
  const ctx = canvasRef.getContext("2d")!;
  ctx.clearRect(0, 0, canvasRef.width, canvasRef.height);

  ctx.beginPath();
  ctx.moveTo(trajectory[0].x, trajectory[0].y);
  for (let i = 0; i < trajectory.length; i++) {
    const point = trajectory[i];
    ctx.lineTo(point.x, point.y);
  }
  ctx.lineWidth = 3;
  ctx.strokeStyle = `#ff4757`;
  ctx.lineJoin = "round";
  ctx.stroke();

  ctx.lineWidth = 1;
  ctx.strokeStyle = `#ced6e0`;
  ctx.lineJoin = "round";
  for (let i = 0; i < frame.circles.length; i++) {
    const circle = frame.circles[i];
    ctx.beginPath();
    ctx.arc(circle.x, circle.y, circle.r, 0, 2 * Math.PI);
    ctx.stroke();
  }

  ctx.lineWidth = 1;
  ctx.strokeStyle = `green`;
  ctx.lineJoin = "round";
  ctx.beginPath();
  for (let i = 0; i < frame.arrows.length; i++) {
    const arrow = frame.arrows[i];
    ctx.moveTo(arrow.x, arrow.y);
    ctx.lineTo(arrow.x_end, arrow.y_end);
  }
  ctx.stroke();
};

export default Home;
