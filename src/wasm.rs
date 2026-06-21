use wasm_bindgen::prelude::*;

use crate::{
    circle,
    config::{GreekKeyCircleConfig, GreekKeyRectConfig, VisualOptions},
    rect,
};

fn validate_stroke_opacity(stroke_opacity: f32) -> Result<(), JsValue> {
    if !(0.0..=1.0).contains(&stroke_opacity) || !stroke_opacity.is_finite() {
        return Err(JsValue::from_str(
            "--stroke-opacity must be a finite number between 0.0 and 1.0",
        ));
    }
    Ok(())
}

/// Generate a rectangle Greek Key pattern and return SVG markup.
///
/// # Arguments
/// - `size` — key unit length (must be > 0)
/// - `width` — number of pattern units across (must be ≥ 3)
/// - `height` — number of pattern units down (must be ≥ 3)
/// - `border_margin` — padding outside the outer frame (must be ≥ 0)
/// - `stroke_width` — line width (must be a positive finite number)
/// - `stroke_color` — CSS color string, e.g. `"#AB8E0E"`
/// - `stroke_opacity` — 0.0–1.0
/// - `fill_color` — optional fill color for the pattern interior
/// - `background_color` — optional canvas background color
/// - `stroke_dash` — optional SVG `stroke-dasharray` value, e.g. `"5,3"`
///
/// Returns an SVG string, or throws a JS error string on invalid input.
#[allow(clippy::too_many_arguments)]
#[wasm_bindgen]
pub fn rect_generate_svg(
    size: i32,
    width: i32,
    height: i32,
    border_margin: i32,
    stroke_width: f32,
    stroke_color: &str,
    stroke_opacity: f32,
    fill_color: Option<String>,
    background_color: Option<String>,
    stroke_dash: Option<String>,
) -> Result<String, JsValue> {
    validate_stroke_opacity(stroke_opacity)?;
    let mut visual = VisualOptions::new(stroke_color, stroke_opacity);
    visual.fill_color = fill_color;
    visual.background_color = background_color;
    visual.stroke_dash = stroke_dash;
    GreekKeyRectConfig::new(size, width, height, border_margin, stroke_width)
        .map(|c| rect::generate_svg_string(&c, &visual))
        .map_err(|e| JsValue::from_str(&e.to_string()))
}

/// Generate a circle Greek Key pattern and return SVG markup.
///
/// # Arguments
/// - `radius` — outer radius in SVG units (must be > 0)
/// - `pattern_count` — number of key units around the ring (must be ≥ 4)
/// - `border_margin` — padding outside the outer circle (must be ≥ 0)
/// - `stroke_width` — line width (must be a positive finite number)
/// - `stroke_color` — CSS color string, e.g. `"#AB8E0E"`
/// - `stroke_opacity` — 0.0–1.0
/// - `fill_color` — optional fill color for the pattern interior
/// - `background_color` — optional canvas background color
/// - `stroke_dash` — optional SVG `stroke-dasharray` value, e.g. `"5,3"`
///
/// Returns an SVG string, or throws a JS error string on invalid input.
#[allow(clippy::too_many_arguments)]
#[wasm_bindgen]
pub fn circle_generate_svg(
    radius: f64,
    pattern_count: i32,
    border_margin: i32,
    stroke_width: f32,
    stroke_color: &str,
    stroke_opacity: f32,
    fill_color: Option<String>,
    background_color: Option<String>,
    stroke_dash: Option<String>,
) -> Result<String, JsValue> {
    validate_stroke_opacity(stroke_opacity)?;
    let mut visual = VisualOptions::new(stroke_color, stroke_opacity);
    visual.fill_color = fill_color;
    visual.background_color = background_color;
    visual.stroke_dash = stroke_dash;
    GreekKeyCircleConfig::new(radius, pattern_count, border_margin, stroke_width)
        .map(|c| circle::generate_svg_string(&c, &visual))
        .map_err(|e| JsValue::from_str(&e.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;

    #[wasm_bindgen_test]
    fn rect_svg_contains_svg_element() {
        let svg = rect_generate_svg(25, 16, 9, 10, 3.0, "#AB8E0E", 0.7, None, None, None).unwrap();
        assert!(svg.contains("<svg"));
        assert!(svg.contains("<path"));
    }

    #[wasm_bindgen_test]
    fn rect_svg_contains_color() {
        let svg = rect_generate_svg(25, 16, 9, 10, 3.0, "#AB8E0E", 0.7, None, None, None).unwrap();
        assert!(svg.contains("#AB8E0E"));
    }

    #[wasm_bindgen_test]
    fn rect_invalid_size_returns_error() {
        let err =
            rect_generate_svg(0, 16, 9, 10, 3.0, "#AB8E0E", 0.7, None, None, None).unwrap_err();
        assert!(err.as_string().unwrap().contains("--size"));
    }

    #[wasm_bindgen_test]
    fn rect_invalid_opacity_returns_error() {
        let err =
            rect_generate_svg(25, 16, 9, 10, 3.0, "#AB8E0E", 1.1, None, None, None).unwrap_err();
        assert!(err.as_string().unwrap().contains("--stroke-opacity"));
    }

    #[wasm_bindgen_test]
    fn circle_svg_contains_svg_element() {
        let svg =
            circle_generate_svg(300.0, 30, 10, 3.0, "#AB8E0E", 0.7, None, None, None).unwrap();
        assert!(svg.contains("<svg"));
        assert!(svg.contains("<path"));
    }

    #[wasm_bindgen_test]
    fn circle_svg_contains_color() {
        let svg =
            circle_generate_svg(300.0, 30, 10, 3.0, "#AB8E0E", 0.7, None, None, None).unwrap();
        assert!(svg.contains("#AB8E0E"));
    }

    #[wasm_bindgen_test]
    fn circle_invalid_radius_returns_error() {
        let err =
            circle_generate_svg(0.0, 30, 10, 3.0, "#AB8E0E", 0.7, None, None, None).unwrap_err();
        assert!(err.as_string().unwrap().contains("--radius"));
    }

    #[wasm_bindgen_test]
    fn circle_nan_radius_returns_error() {
        let err = circle_generate_svg(f64::NAN, 30, 10, 3.0, "#AB8E0E", 0.7, None, None, None)
            .unwrap_err();
        assert!(err.as_string().unwrap().contains("--radius"));
    }

    #[wasm_bindgen_test]
    fn circle_infinite_radius_returns_error() {
        let err = circle_generate_svg(f64::INFINITY, 30, 10, 3.0, "#AB8E0E", 0.7, None, None, None)
            .unwrap_err();
        assert!(err.as_string().unwrap().contains("--radius"));
    }

    #[wasm_bindgen_test]
    fn circle_nan_opacity_returns_error() {
        let err = circle_generate_svg(300.0, 30, 10, 3.0, "#AB8E0E", f32::NAN, None, None, None)
            .unwrap_err();
        assert!(err.as_string().unwrap().contains("--stroke-opacity"));
    }
}
