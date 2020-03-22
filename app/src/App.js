import React, { useEffect, useState } from "react";
import "./App.css";

function App() {
  const [error, setError] = useState(null);
  const [loading, setLoading] = useState(false);
  const [wasmGame, setWasmGame] = useState(null);

  useEffect(loadWasmGame, []);
  function loadWasmGame() {
    async function load() {
      try {
        setLoading(true);
        const wasm = await import("game");
        setWasmGame(wasm);
      } catch (err) {
        console.error(err);
        setError(err);
      }
      setLoading(false);
    }
    load();
  }

  return (
    <div className="App">
      <header className="App-header">
        <p>Fast Coder</p>
        {loading && <Loader />}
        {error && <Error error={error} />}
        {<Game wasm={wasmGame} />}
      </header>
    </div>
  );
};

function Loader() {
  return <div>Loading...</div>;
};

function Error() {
  return <div>Error :(</div>;
};

function Game(props) {
  return (
    <div>
      <canvas id="game-canvas" height={400} width={600}/>
    </div>
  );
};

export default App;
