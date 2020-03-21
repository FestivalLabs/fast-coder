import React, { useEffect, useState } from 'react';
import './App.css';

function App() {
  const [error, setError] = useState(null);
  const [loading, setLoading] = useState(false);
  const [wasmGame, setWasmGame] = useState(null);

  useEffect(loadWasmGame, []);

  async function loadWasmGame() {
    try {
      setLoading(true);
      const wasm = await import('game');
      setWasmGame(wasm);
    } catch (err) {
      setError(err);
    }
    setLoading(false);
  }

  return (
    <div className="App">
      <header className="App-header">
        <p>Fast Coder</p>
        {loading && <Loader />}
        {error && <Error error={error} />}
        {wasmGame && <Game wasm={wasmGame} />}
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
  return <div onClick={props.wasm.greet}>Click me!</div>;
};

export default App;
