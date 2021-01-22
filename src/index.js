import React from "react";
import ReactDOM from "react-dom";
import "./index.css";
import App from "./App";
import reportWebVitals from "./reportWebVitals";
import samples from "./datasets/samples.json";
import studies from "./datasets/studies.json";

(async () => {
  const pkg = await import("../public/pkg/rust_wasm_upwork.js");

  await pkg.default("/pkg/rust_wasm_upwork.wasm");

  const res = pkg.execute(studies, samples);

  console.log(res);
})();

ReactDOM.render(
  <React.StrictMode>
    <App />
  </React.StrictMode>,
  document.getElementById("root")
);

// If you want to start measuring performance in your app, pass a function
// to log results (for example: reportWebVitals(console.log))
// or send to an analytics endpoint. Learn more: https://bit.ly/CRA-vitals
reportWebVitals();
