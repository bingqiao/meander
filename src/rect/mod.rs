use svg::Document;
use svg::node::element::path::Data;
use svg::node::element::{Path as SvgPath, Rectangle};

#[cfg(feature = "native")]
use crate::common::save_and_convert_svg;
use crate::config::{GreekKeyRectConfig, VisualOptions};

fn draw_horizontal_unit(data: Data, key_unit_length: i32) -> Data {
    data.line_by((0, -4 * key_unit_length))
        .line_by((4 * key_unit_length, 0))
        .line_by((0, 3 * key_unit_length))
        .line_by((-2 * key_unit_length, 0))
        .line_by((0, -key_unit_length))
        .line_by((key_unit_length, 0))
        .line_by((0, -key_unit_length))
        .line_by((-2 * key_unit_length, 0))
        .line_by((0, 3 * key_unit_length))
        .line_by((4 * key_unit_length, 0))
}

fn draw_vertical_unit(data: Data, key_unit_length: i32) -> Data {
    data.line_by((4 * key_unit_length, 0))
        .line_by((0, 4 * key_unit_length))
        .line_by((-3 * key_unit_length, 0))
        .line_by((0, -2 * key_unit_length))
        .line_by((key_unit_length, 0))
        .line_by((0, key_unit_length))
        .line_by((key_unit_length, 0))
        .line_by((0, -2 * key_unit_length))
        .line_by((-3 * key_unit_length, 0))
        .line_by((0, 4 * key_unit_length))
}

fn draw_horizontal_unit_right_to_left(data: Data, key_unit_length: i32) -> Data {
    data.line_by((-4 * key_unit_length, 0))
        .line_by((0, -3 * key_unit_length))
        .line_by((2 * key_unit_length, 0))
        .line_by((0, key_unit_length))
        .line_by((-key_unit_length, 0))
        .line_by((0, key_unit_length))
        .line_by((2 * key_unit_length, 0))
        .line_by((0, -3 * key_unit_length))
        .line_by((-4 * key_unit_length, 0))
        .line_by((0, 4 * key_unit_length))
}

fn draw_vertical_unit_bottom_up(data: Data, key_unit_length: i32) -> Data {
    data.line_by((0, -4 * key_unit_length))
        .line_by((3 * key_unit_length, 0))
        .line_by((0, 2 * key_unit_length))
        .line_by((-key_unit_length, 0))
        .line_by((0, -key_unit_length))
        .line_by((-key_unit_length, 0))
        .line_by((0, 2 * key_unit_length))
        .line_by((3 * key_unit_length, 0))
        .line_by((0, -4 * key_unit_length))
        .line_by((-4 * key_unit_length, 0))
}

fn draw_frame(
    x: f64,
    y: f64,
    w: i32,
    h: i32,
    stroke_color: &str,
    stroke_width: f32,
    stroke_opacity: f32,
) -> SvgPath {
    let data = Data::new()
        .move_to((x, y))
        .line_by((w, 0))
        .line_by((0, h))
        .line_by((-w, 0))
        .close();
    SvgPath::new()
        .set("fill", "none")
        .set("stroke", stroke_color.to_string())
        .set("stroke-width", stroke_width)
        .set("stroke-opacity", stroke_opacity)
        .set("d", data)
}

fn draw_greek_key_patterns(config: &GreekKeyRectConfig) -> Data {
    let (start_x, start_y) = config.get_start_position();
    let key_unit_length = config.key_unit_length;
    let width_units = config.width_units;
    let height_units = config.height_units;

    let mut data = Data::new().move_to((start_x, start_y));
    data = data.line_by((0, -key_unit_length));

    for _ in 0..width_units - 1 {
        data = draw_horizontal_unit(data, key_unit_length);
    }

    data = data.line_by((0, -4 * key_unit_length));
    data = data.line_by((key_unit_length, 0));

    for _ in 0..height_units - 1 {
        data = draw_vertical_unit(data, key_unit_length);
    }

    data = data.line_by((4 * key_unit_length, 0));
    data = data.line_by((0, 5 * key_unit_length));

    for _ in 0..width_units - 1 {
        data = draw_horizontal_unit_right_to_left(data, key_unit_length);
    }

    data = data.line_by((-5 * key_unit_length, 0));

    for _ in 0..height_units - 1 {
        data = draw_vertical_unit_bottom_up(data, key_unit_length);
    }

    data.close()
}

fn apply_dash(path: SvgPath, dash: Option<&str>) -> SvgPath {
    match dash {
        Some(d) => path.set("stroke-dasharray", d),
        None => path,
    }
}

fn build_document(config: &GreekKeyRectConfig, visual: &VisualOptions) -> Document {
    let stroke_width = config.stroke_width;
    let stroke_color = visual.stroke_color.as_str();
    let stroke_opacity = visual.stroke_opacity;
    let dash = visual.stroke_dash.as_deref();
    let (width, height) = config.get_canvas_size();
    let mut document = Document::new().set("viewBox", (0, 0, width, height));

    if let Some(bg) = &visual.background_color {
        document = document.add(
            Rectangle::new()
                .set("width", width)
                .set("height", height)
                .set("fill", bg.as_str()),
        );
    }

    let path_data = draw_greek_key_patterns(config);
    let path = apply_dash(
        SvgPath::new()
            .set("fill", visual.fill_color.as_deref().unwrap_or("none"))
            .set("stroke", stroke_color)
            .set("stroke-width", stroke_width)
            .set("stroke-opacity", stroke_opacity)
            .set("d", path_data),
        dash,
    );
    document = document.add(path);

    let (outer_x, outer_y, outer_width, outer_height) = config.get_outer_frame_size();
    document = document.add(apply_dash(
        draw_frame(
            outer_x,
            outer_y,
            outer_width,
            outer_height,
            stroke_color,
            stroke_width,
            stroke_opacity,
        ),
        dash,
    ));

    let (inner_x, inner_y, inner_width, inner_height) = config.get_inner_frame_size();
    document = document.add(apply_dash(
        draw_frame(
            inner_x,
            inner_y,
            inner_width,
            inner_height,
            stroke_color,
            stroke_width,
            stroke_opacity,
        ),
        dash,
    ));

    document
}

/// Returns the rectangle Greek Key pattern as an SVG string.
///
/// Available on all targets including WASM. For file output, use
/// [`generate_pattern_svg`] (requires the `native` feature).
pub fn generate_svg_string(config: &GreekKeyRectConfig, visual: &VisualOptions) -> String {
    build_document(config, visual).to_string()
}

/// Generates a rectangle Greek Key pattern and writes `<filename>.svg` and `<filename>.png`.
///
/// Requires the `native` feature (enabled by default). For WASM targets, use
/// [`generate_svg_string`] instead.
#[cfg(feature = "native")]
pub fn generate_pattern_svg(
    config: &GreekKeyRectConfig,
    visual: &VisualOptions,
    filename: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    save_and_convert_svg(build_document(config, visual), filename)
}
