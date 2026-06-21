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

## Next: Visual Options

Add more styling controls while keeping the core SVG generation usable from
Rust, WASM, scripts, and asset pipelines.

### User Interface

Add options such as:

- `--fill-color`: fill pattern interiors where applicable.
- `--background-color`: set the SVG canvas background.
- `--stroke-dash <PATTERN>`: emit dashed SVG strokes.

### Implementation Design

- Keep visual styling independent from native file I/O and PNG rasterization.
- Preserve WASM compatibility for SVG string generation.
- Prefer serializable config fields where options should be reusable across
  asset pipelines.

### Done Criteria

- Existing default output remains unchanged unless new options are supplied.
- README documents each new option and its config-file equivalent when present.
- Tests cover SVG structure and interaction with stdout/selective output modes.
