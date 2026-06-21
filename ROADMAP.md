# Roadmap

## Completed

### Library API

Status: completed in v0.1.5.

- Exposed rectangle and circle generation through the public Rust library API.
- Added SVG string generation so other crates can use `greek-meander`
  programmatically without going through the CLI.
- Kept native file output available behind the default `native` feature.

### WebAssembly Support

Status: completed in v0.1.5.

- Added a WASM build path for browser-based use.
- Exposed JavaScript-callable SVG generation functions for rectangle and circle
  patterns.
- Added a browser example showing generated SVG output.

### Pipeline-Friendly Output

Status: completed for the next release.

- Added `--stdout` so generated SVG can be piped to other tools.
- Added `--no-svg` and `--no-png` for selective file output.
- Added `--scale <FACTOR>` for higher-resolution PNG rasterization.
- Kept the default behavior compatible: commands still write both SVG and PNG
  unless output flags say otherwise.

### Config File Input

Status: completed for the next release.

- Added `--config <PATH>` for TOML config files.
- Config files can set shared options and rectangle or circle defaults.
- Explicit CLI flags override config file values.
- Missing fields fall back to the same defaults as CLI-only usage.
- Kept output routing flags (`--stdout`, `--no-svg`, `--no-png`) as run-specific
  command-line choices.

### Visual Options

Status: completed for the next release.

- Added `--fill-color` to fill the pattern interior.
- Added `--background-color` to set an SVG canvas background.
- Added `--stroke-dash` for dashed SVG strokes (any `stroke-dasharray` value).
- All three options are available as TOML config file fields.
- Visual styling consolidated into `VisualOptions` in the public Rust API.
- WASM exports accept the new options as optional trailing parameters.
- Default output is unchanged when no new options are supplied.

## Next: Shape Expansion

Add new shape families while keeping the core geometry API stable.

- Add ellipse borders with separate horizontal and vertical radii.

