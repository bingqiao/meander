# Browser WASM example

Build the browser package from the repository root:

```bash
wasm-pack build \
  --target web \
  --out-dir examples/wasm-browser/pkg \
  --out-name greek_meander \
  --no-default-features \
  --features wasm
```

Serve the repository root with any static file server, then open
`examples/wasm-browser/index.html`.

The example renders rectangle and ellipse SVGs in the browser and exposes the
WASM visual options: stroke color, opacity, fill color, background color, and
stroke dash. It also includes SVG and PNG download buttons for each generated
pattern.

For example:

```bash
python3 -m http.server 8000
```

Then visit:

```text
http://localhost:8000/examples/wasm-browser/
```

The GitHub Pages workflow builds the same static layout and publishes this
directory as the site root. In the repository settings, set Pages to deploy from
GitHub Actions.
