# Browser WASM example

Build the browser package from the repository root:

```bash
wasm-pack build --target web --no-default-features --features wasm
```

Serve the repository root with any static file server, then open
`examples/wasm-browser/index.html`.

The example renders rectangle and circle SVGs in the browser and exposes the
WASM visual options: stroke color, opacity, fill color, background color, and
stroke dash.

For example:

```bash
python3 -m http.server 8000
```

Then visit:

```text
http://localhost:8000/examples/wasm-browser/
```
