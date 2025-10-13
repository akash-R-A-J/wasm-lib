import "./App.css";
import { useEffect, useState } from "react";
import * as wasm from "../pkg/my_lib";

function App() {
  const [sum, setSum] = useState<number | null>(null);
  const [greeting, setGreeting] = useState("");
  const [asyncResult, setAsyncResult] = useState<number | null>(null);

  useEffect(() => {
    (async () => {

      // call exported functions
      const result = wasm.add(5, 7);
      setSum(result);

      const greetMsg = wasm.greet("Akash");
      setGreeting(greetMsg);

      // call async rust function [returns a JS promise]
      const doubled = await wasm.async_compute(21);
      setAsyncResult(doubled);
    })();
  }, []);

  return (
    <div style={{ padding: "20px", fontFamily: "sans-serif" }}>
      <h1>Rust + WASM + React Demo</h1>
      <p>5 + 7 = {sum}</p>
      <p>{greeting}</p>
      <p>Asyn result (21 * 2) = {asyncResult}</p>
    </div>
  );
}

export default App;
