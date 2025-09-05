# Meander

[![Crates.io](https://img.shields.io/crates/v/greek-meander.svg)](https://crates.io/crates/greek-meander)

Create a frame of Greek Key (Meander) design in SVG format.

This is a Rust crate for creating both rectangle and circle border designs of the Greek Key (Meander).

## Images

Here are some examples of the images that can be generated:

**Rectangle**

<img src="https://raw.githubusercontent.com/bingqiao/meander/refs/heads/master/images/meander_rect.png" width="250">

**Circle**

<img src="https://raw.githubusercontent.com/bingqiao/meander/refs/heads/master/images/meander_circle.png" width="250">

## Install

```bash
cargo install greek-meander
```

## Usage

### General Options

| Option | Description | Default |
|---|---|---|
| `stroke-width` | The width of the stroke | 6.0 |
| `stroke-color` | The color of the stroke | "#AB8E0E" |
| `stroke-opacity`| The opacity of the stroke | 0.7 |
| `border-margin` | The margin of the border | 1 |
| `file` | The base name of the output file | "meander" |

### Rectangle

To generate a rectangle meander design, use the `rect` command:

```bash
meander rect --size <SIZE> --width <WIDTH> --height <HEIGHT>
```

**Options**

| Option | Description | Default |
|---|---|---|
| `size` | The size of the greek key cells | 25 |
| `width` | The number of cells for the top and bottom borders | 16 |
| `height` | The number of cells for the left and right borders | 9 |

**Example**

```bash
meander --stroke-color "blue" --file "my_design" rect --size 12 --width 22 --height 14
```

This will generate a `my_design_rect.svg` file.

### Circle

To generate a circle meander design, use the `circle` command:

```bash
meander circle --radius <RADIUS> --pattern-count <PATTERN_COUNT>
```

**Options**

| Option | Description | Default |
|---|---|---|
| `radius` | The radius of the circle | 300.0 |
| `pattern-count` | The number of patterns in the circle | 30 |

**Example**

```bash
meander --stroke-color "red" --file "my_circle_design" circle --radius 120 --pattern-count 24
```

This will generate a `my_circle_design_circle.svg` file.

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

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.