use svg::Document;
use svg::node::element::path::Data;
use svg::node::element::{Ellipse as SvgEllipse, Path as SvgPath, Rectangle};

#[cfg(feature = "native")]
use crate::common::save_and_convert_svg;
use crate::config::{GreekKeyEllipseConfig, VisualOptions};

fn draw_greek_key_patterns(config: &GreekKeyEllipseConfig) -> Data {
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
    cx: f64,
    cy: f64,
    rx: f64,
    ry: f64,
    stroke_color: &str,
    stroke_width: f32,
    stroke_opacity: f32,
) -> SvgEllipse {
    SvgEllipse::new()
        .set("cx", cx)
        .set("cy", cy)
        .set("rx", rx)
        .set("ry", ry)
        .set("fill", "none")
        .set("stroke", stroke_color.to_string())
        .set("stroke-width", stroke_width)
        .set("stroke-opacity", stroke_opacity)
}

fn apply_dash(ellipse: SvgEllipse, dash: Option<&str>) -> SvgEllipse {
    match dash {
        Some(d) => ellipse.set("stroke-dasharray", d),
        None => ellipse,
    }
}

fn build_document(config: &GreekKeyEllipseConfig, visual: &VisualOptions) -> Document {
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
    let path = SvgPath::new()
        .set("fill", visual.fill_color.as_deref().unwrap_or("none"))
        .set("stroke", stroke_color)
        .set("stroke-width", stroke_width)
        .set("stroke-opacity", stroke_opacity)
        .set("d", path_data);
    let path = match dash {
        Some(d) => path.set("stroke-dasharray", d),
        None => path,
    };
    document = document.add(path);

    let centre = config.get_centre();
    let er = &config.ellipse_radii;
    document = document.add(apply_dash(
        draw_frame(
            centre.x,
            centre.y,
            er.rx_i,
            er.ry_i,
            stroke_color,
            stroke_width,
            stroke_opacity,
        ),
        dash,
    ));
    document = document.add(apply_dash(
        draw_frame(
            centre.x,
            centre.y,
            config.rx,
            config.ry,
            stroke_color,
            stroke_width,
            stroke_opacity,
        ),
        dash,
    ));

    document
}

/// Returns the ellipse Greek Key pattern as an SVG string.
///
/// Available on all targets including WASM. For file output, use
/// [`generate_pattern_svg`] (requires the `native` feature).
pub fn generate_svg_string(config: &GreekKeyEllipseConfig, visual: &VisualOptions) -> String {
    build_document(config, visual).to_string()
}

/// Generates an ellipse Greek Key pattern and writes `<filename>.svg` and `<filename>.png`.
///
/// Requires the `native` feature (enabled by default). For WASM targets, use
/// [`generate_svg_string`] instead.
#[cfg(feature = "native")]
pub fn generate_pattern_svg(
    config: &GreekKeyEllipseConfig,
    visual: &VisualOptions,
    filename: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    save_and_convert_svg(build_document(config, visual), filename)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ellipse_svg_contains_svg_element() {
        let config = GreekKeyEllipseConfig::new(300.0, 200.0, 30, 10, 3.0).unwrap();
        let svg = generate_svg_string(&config, &VisualOptions::default());
        assert!(svg.contains("<svg"));
        assert!(svg.contains("<path"));
    }

    #[test]
    fn ellipse_svg_contains_ellipse_frames() {
        let config = GreekKeyEllipseConfig::new(300.0, 200.0, 30, 10, 3.0).unwrap();
        let svg = generate_svg_string(&config, &VisualOptions::default());
        assert!(svg.contains("<ellipse"));
    }

    #[test]
    fn ellipse_svg_contains_stroke_color() {
        let config = GreekKeyEllipseConfig::new(300.0, 200.0, 30, 10, 3.0).unwrap();
        let svg = generate_svg_string(&config, &VisualOptions::default());
        assert!(svg.contains("#AB8E0E"));
    }

    #[test]
    fn ellipse_svg_contains_fill_color() {
        let config = GreekKeyEllipseConfig::new(300.0, 200.0, 30, 10, 3.0).unwrap();
        let visual = VisualOptions {
            fill_color: Some("#FF0000".to_string()),
            ..VisualOptions::default()
        };
        let svg = generate_svg_string(&config, &visual);
        assert!(svg.contains("#FF0000"));
    }

    #[test]
    fn ellipse_svg_contains_background() {
        let config = GreekKeyEllipseConfig::new(300.0, 200.0, 30, 10, 3.0).unwrap();
        let visual = VisualOptions {
            background_color: Some("#001122".to_string()),
            ..VisualOptions::default()
        };
        let svg = generate_svg_string(&config, &visual);
        assert!(svg.contains("#001122"));
        assert!(svg.contains("<rect"));
    }

    #[test]
    fn ellipse_svg_contains_dash() {
        let config = GreekKeyEllipseConfig::new(300.0, 200.0, 30, 10, 3.0).unwrap();
        let visual = VisualOptions {
            stroke_dash: Some("5,3".to_string()),
            ..VisualOptions::default()
        };
        let svg = generate_svg_string(&config, &visual);
        assert!(svg.contains("stroke-dasharray"));
    }

    #[test]
    fn ellipse_equal_axes_produces_valid_svg() {
        let config = GreekKeyEllipseConfig::new(200.0, 200.0, 20, 5, 2.0).unwrap();
        let svg = generate_svg_string(&config, &VisualOptions::default());
        assert!(svg.contains("<svg"));
    }
}
