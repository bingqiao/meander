use svg::Document;
use svg::node::element::path::Data;
use svg::node::element::{Circle, Path as SvgPath};

#[cfg(feature = "native")]
use crate::common::save_and_convert_svg;
use crate::config::GreekKeyCircleConfig;

fn draw_greek_key_patterns(config: &GreekKeyCircleConfig) -> Data {
    let (mut points_a, mut points_b, mut points_c, mut points_d, mut points_e) =
        config.get_coords_for_patterns();

    let mut data = Data::new().move_to((points_a[0].x, points_a[0].y));

    for _ in 0..config.pattern_count {
        data = data
            .line_to((points_e[0].x, points_e[0].y))
            .line_to((points_e[4].x, points_e[4].y))
            .line_to((points_b[4].x, points_b[4].y))
            .line_to((points_b[2].x, points_b[2].y))
            .line_to((points_c[2].x, points_c[2].y))
            .line_to((points_c[3].x, points_c[3].y))
            .line_to((points_d[3].x, points_d[3].y))
            .line_to((points_d[1].x, points_d[1].y))
            .line_to((points_a[1].x, points_a[1].y))
            .line_to((points_a[5].x, points_a[5].y));

        (points_a, points_b, points_c, points_d, points_e) = config.get_coords_for_patterns_by_p0(
            points_a[5],
            points_b[5],
            points_c[5],
            points_d[5],
            points_e[5],
        );
    }

    data.close()
}

fn draw_frame(
    x: f64,
    y: f64,
    r: f64,
    stroke_color: &str,
    stroke_width: f32,
    stroke_opacity: f32,
) -> Circle {
    Circle::new()
        .set("cx", x)
        .set("cy", y)
        .set("r", r)
        .set("fill", "none")
        .set("stroke", stroke_color.to_string())
        .set("stroke-width", stroke_width)
        .set("stroke-opacity", stroke_opacity)
}

fn build_document(
    config: &GreekKeyCircleConfig,
    stroke_color: &str,
    stroke_opacity: f32,
) -> Document {
    let stroke_width = config.stroke_width;
    let (width, height) = config.get_canvas_size();
    let mut document = Document::new().set("viewBox", (0, 0, width, height));

    let path_data = draw_greek_key_patterns(config);
    let path = SvgPath::new()
        .set("fill", "none")
        .set("stroke", stroke_color.to_string())
        .set("stroke-width", stroke_width)
        .set("stroke-opacity", stroke_opacity)
        .set("d", path_data);

    document = document.add(path);

    let centre = config.get_centre();
    document = document.add(draw_frame(
        centre.x,
        centre.y,
        config.radii.r_i,
        stroke_color,
        stroke_width,
        stroke_opacity,
    ));
    document = document.add(draw_frame(
        centre.x,
        centre.y,
        config.radii.r_o,
        stroke_color,
        stroke_width,
        stroke_opacity,
    ));

    document
}

/// Returns the circle Greek Key pattern as an SVG string.
///
/// Available on all targets including WASM. For file output, use
/// [`generate_pattern_svg`] (requires the `native` feature).
pub fn generate_svg_string(
    config: &GreekKeyCircleConfig,
    stroke_color: &str,
    stroke_opacity: f32,
) -> String {
    build_document(config, stroke_color, stroke_opacity).to_string()
}

/// Generates a circle Greek Key pattern and writes `<filename>.svg` and `<filename>.png`.
///
/// Requires the `native` feature (enabled by default). For WASM targets, use
/// [`generate_svg_string`] instead.
#[cfg(feature = "native")]
pub fn generate_pattern_svg(
    config: &GreekKeyCircleConfig,
    stroke_color: &str,
    stroke_opacity: f32,
    filename: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    save_and_convert_svg(
        build_document(config, stroke_color, stroke_opacity),
        filename,
    )
}
