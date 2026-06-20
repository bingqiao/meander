# Meander

[![Crates.io](https://img.shields.io/crates/v/greek-meander.svg)](https://crates.io/crates/greek-meander)

Create a frame of Greek Key (Meander) design in SVG and PNG format.

This is a Rust crate for creating both rectangle and circle border designs of the Greek Key (Meander).

## Images

Here are some examples of the images that can be generated:

**Rectangle**

<img src="https://raw.githubusercontent.com/bingqiao/meander/refs/heads/master/images/meander_rect.png" width="700">

**Circle**

<img src="https://raw.githubusercontent.com/bingqiao/meander/refs/heads/master/images/meander_circle.png" width="700">

## Install

```bash
cargo install greek-meander
```

## Usage

### General Options

| Option | Description | Default |
|---|---|---|
| `--stroke-width` | The width of the stroke | 6.0 |
| `--stroke-color` | The color of the stroke | "#AB8E0E" |
| `--stroke-opacity` | The opacity of the stroke | 0.7 |
| `--border-margin` | The margin of the border | 1 |
| `--file` | The base name of the output file | "meander" |
| `--stdout` | Write generated SVG markup to stdout | false |
| `--no-svg` | Skip writing the SVG file | false |
| `--no-png` | Skip writing the PNG file | false |
| `--scale` | Multiply PNG output dimensions without changing the SVG viewBox | 1.0 |

### Rectangle

To generate a rectangle meander design, use the `rect` command:

```bash
greek-meander rect --size <SIZE> --width <WIDTH> --height <HEIGHT>
```

**Options**

| Option | Description | Default |
|---|---|---|
| `--size` | The size of the greek key cells | 25 |
| `--width` | The number of cells for the top and bottom borders | 16 |
| `--height` | The number of cells for the left and right borders | 9 |

**Example**

```bash
greek-meander --stroke-color "blue" --file "my_design" rect --size 12 --width 22 --height 14
```

This will generate `my_design.svg` and `my_design.png`.

### Circle

To generate a circle meander design, use the `circle` command:

```bash
greek-meander circle --radius <RADIUS> --pattern-count <PATTERN_COUNT>
```

**Options**

| Option | Description | Default |
|---|---|---|
| `--radius` | The radius of the circle | 300.0 |
| `--pattern-count` | The number of patterns in the circle | 30 |

**Example**

```bash
greek-meander --stroke-color "red" --file "my_circle_design" circle --radius 120 --pattern-count 24
```

This will generate `my_circle_design.svg` and `my_circle_design.png`.

### Output Control

By default, `greek-meander` writes both `<file>.svg` and `<file>.png`.

Use `--stdout` to pipe SVG markup to another command:

```bash
greek-meander --stdout --no-svg --no-png rect > meander.svg
```

Use `--no-png` or `--no-svg` to generate only one file type:

```bash
greek-meander --no-png rect
greek-meander --no-svg circle
```

Use `--scale` to increase PNG resolution while preserving the SVG viewBox:

```bash
greek-meander --scale 2 rect
```

## Build and Run

To build this project, navigate to the project root directory and run:

```bash
cargo build
```

To run the project, use:

```bash
cargo run -- <GENERAL_OPTIONS> <COMMAND> <OPTIONS>
```

For example:

```bash
cargo run -- --stroke-color "blue" --file "my_design" rect --size 12 --width 22 --height 14
```

This will generate `my_design.svg` and `my_design.png`.

## Browser WASM

Install `wasm-pack` if it is not already available:

```bash
cargo install wasm-pack
```

To build the browser WebAssembly package, disable native file output and enable
the `wasm` feature:

```bash
wasm-pack build --target web --no-default-features --features wasm
```

This creates a `pkg/` directory with JavaScript bindings for
`rect_generate_svg` and `circle_generate_svg`, which return SVG markup strings.

To try the browser example:

```bash
python3 -m http.server 8000
```

Then visit:

```text
http://localhost:8000/examples/wasm-browser/
```

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
