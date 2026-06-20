# Roadmap

## Library API

Expose `rect` and `circle` generation as a public Rust library via `lib.rs` so other crates can use greek-meander programmatically as a dependency, not just via CLI.

Include a WebAssembly (WASM) target so the library can run in browser-based design tools.

Status: completed in v0.1.5.

## Output Control

- `--no-png` / `--no-svg` flags to skip generating one of the two output files
- `--scale <FACTOR>` to multiply the PNG pixel dimensions (e.g. `--scale 2` doubles the output resolution; currently 1:1 with the SVG viewBox)
- Additional export formats: JPEG, PDF (useful for print and laser cutting)
- `--stdout` to pipe SVG to stdout for integration with other tools

## Visual Options

- `--fill-color` to fill the interior of the pattern (currently always transparent)
- `--background-color` to set a canvas background colour
- `--direction cw|ccw` to select clockwise or counter-clockwise meander (the two classical chiralities)
- `--stroke-dash <PATTERN>` for dashed stroke lines, adding aesthetic variety
- `--theme <NAME>` for named colour presets (e.g. `gold`, `greek-blue`, `black`) so non-designers can get good results without specifying hex codes
- Animated SVG via `--animate`: uses CSS animation to draw the meander path progressively — makes the output visually striking for web use

## New Shapes

- **Ellipse** — meander ring on an ellipse with separate `--rx` and `--ry` radii; natural extension of the circle command
- **Polygon** — meander border on a regular n-sided polygon (hexagon, octagon, etc.); mathematically complex, requires research into path fitting along polygon edges

## Nested Borders

Support multiple concentric meander bands in a single output — e.g. an inner and outer ring on the circle pattern, or double borders on the rectangle. A common traditional use of the motif.

## Config File Input

Accept a TOML config file as an alternative to CLI flags, making it easy to save, share, and reproduce complex designs.

## Developer Experience

- Shell completions for bash, zsh, fish, and PowerShell via `clap_complete` (low effort, high impact)
- Improved error messages that reference CLI parameter names rather than internal variables (e.g. `"--pattern-count must be at least 4"` instead of `"n must be greater or equal to 19"`)
