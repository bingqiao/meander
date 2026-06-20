# Browser WASM example

Build the browser package from the repository root:

```bash
wasm-pack build --target web --no-default-features --features wasm
```

Serve the repository root with any static file server, then open
`examples/wasm-browser/index.html`.

For example:

```bash
python3 -m http.server 8000
```

Then visit:

```text
http://localhost:8000/examples/wasm-browser/
```

