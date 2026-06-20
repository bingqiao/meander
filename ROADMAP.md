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

## Next: Config File Input

Add TOML config files so designs can be saved, shared, reviewed, and reproduced
without long CLI commands.

### User Interface

Add a top-level CLI option:

- `--config <PATH>`: load shared options, shape selection, and shape-specific
  settings from a TOML file.

CLI flags should continue to work without config files. When both are provided,
explicit CLI flags should override values loaded from the config file.

### Behavior

- A config file should be able to describe either a rectangle or circle pattern.
- Missing fields should fall back to the same defaults as the CLI.
- Invalid config values should return errors that mention the relevant config
  field and matching CLI option where possible.
- Config loading should not change the existing Rust library API unless shared
  config types make that cleaner.

### Implementation Design

- Add a serializable CLI config model separate from the existing validated
  geometry config structs.
- Convert loaded config plus CLI overrides into `GreekKeyRectConfig` or
  `GreekKeyCircleConfig` only after all defaults are resolved.
- Keep config file parsing native-only for now.
- Include at least one example config for each supported shape.

### Done Criteria

- Existing CLI commands continue to work unchanged.
- `README.md` documents the config format and override behavior.
- Tests cover rectangle config, circle config, CLI override behavior, and invalid
  config errors.
