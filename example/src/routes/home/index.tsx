import { FunctionalComponent, h } from "preact";
import style from "./style.css";

import { useEffect, useState, useRef } from "preact/hooks";
import { create, next } from "fourier-animation";

import type { Frame, Point } from "../../types/frame";

const Home: FunctionalComponent = () => {
  let canvasRef = useRef(null);
  useEffect(() => {
    create(0, Uint32Array.from([0, 0]));
  }, []);

  const [frame, setFrame] = useState<Frame | null>(null);
  let [trajectory, setTrajectory] = useState<Point[]>([]);

  const pushPoint = (point: Point) => {
    setTrajectory([...trajectory, point]);
  };

  const goNext = () => {
    let frame = next(0) as Frame;
    pushPoint(frame.next_point);
    setFrame(frame);
  };

  useEffect(() => {
    let canvas = canvasRef.current as unknown as HTMLCanvasElement;
    _goNext(canvas, frame!, trajectory);
  }, [frame]);

  return (
    <div class={style.home}>
      <h1>Fourier Animation Example</h1>
      <div>
        <button onClick={goNext}>Next Frame</button>
      </div>
      <canvas ref={canvasRef}></canvas>
      <pre>{JSON.stringify(frame, undefined, 2)}</pre>
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
  ctx.moveTo(0, 0);
  ctx.lineJoin = "round";
  for (let i = 0; i < trajectory.length; i++) {
    const point = trajectory[i];
    ctx.fillStyle = `black`;
    ctx.lineTo(point.x, point.y);
    ctx.stroke();
  }
};

export default Home;
