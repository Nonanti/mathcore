# MathCore Wasm Eval

This example builds a small `wasm32-unknown-unknown` module and a static HTML page that calls it directly from the browser.

It uses:

- `no_std`
- `alloc`
- a tiny manual wasm ABI
- no dependency on `wasm-bindgen`
- no dependency on `std`

## Build

From this directory:

```sh
cargo build --target wasm32-unknown-unknown --release --no-default-features
```

That produces the wasm module used by `index.html`.

## Run Locally

Browsers generally will not load the wasm module correctly when `index.html` is opened directly from disk with `file://`.

Serve this directory over HTTP instead with (for example):

```sh
python3 -m http.server
```

Then open:

```text
http://localhost:8000
```

## Notes

- The HTML page loads the generated wasm module directly with `fetch` and `WebAssembly.instantiate`.
- The wasm module exports a minimal interface for allocating input, evaluating an expression, and returning the result string.
- This example is intentionally simple and avoids JS binding layers because they are unnecessary for this use case.