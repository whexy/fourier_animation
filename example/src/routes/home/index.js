import { h } from "preact";
import style from "./style.css";
import { useState, useEffect } from "preact/hooks";

import { create, next } from "fourier-animation";

const Home = () => {
  useEffect(() => {
    create(0, [0, 0]);
  }, []);
  const [val, setVal] = useState([]);
  const goNext = () => {
    setVal(next(0));
  };
  return (
    <div class={style.home}>
      <h1>Fourier Animation Example</h1>
      <button onClick={goNext}>Next Frame</button>
      <pre>{JSON.stringify(val, undefined, 2)}</pre>
    </div>
  );
};

export default Home;
