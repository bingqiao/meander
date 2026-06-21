use std::f64::consts::PI;

use crate::common::Point;

/// Visual styling options for SVG pattern generation.
///
/// Usable on all targets including WASM; no native-only dependencies.
#[cfg_attr(feature = "native", derive(serde::Deserialize, serde::Serialize))]
#[derive(Debug, Clone)]
pub struct VisualOptions {
    pub stroke_color: String,
    pub stroke_opacity: f32,
    /// Fill color for the pattern interior. `None` leaves the interior transparent.
    pub fill_color: Option<String>,
    /// Background color for the SVG canvas. `None` omits the background element.
    pub background_color: Option<String>,
    /// SVG `stroke-dasharray` value (e.g. `"5,3"`). `None` produces solid strokes.
    pub stroke_dash: Option<String>,
}

impl VisualOptions {
    pub fn new(stroke_color: impl Into<String>, stroke_opacity: f32) -> Self {
        Self {
            stroke_color: stroke_color.into(),
            stroke_opacity,
            fill_color: None,
            background_color: None,
            stroke_dash: None,
        }
    }
}

impl Default for VisualOptions {
    fn default() -> Self {
        Self::new("#AB8E0E", 0.7)
    }
}

/// Configuration for a rectangle Greek Key border pattern.
#[derive(Debug)]
pub struct GreekKeyRectConfig {
    pub key_unit_length: i32,
    pub width_units: i32,
    pub height_units: i32,
    pub key_pattern_length: i32,
    pub border_margin: i32,
    pub stroke_width: f32,
}

impl GreekKeyRectConfig {
    /// Creates a new rect config.
    ///
    /// Returns an error if `key_unit_length` ≤ 0, `width` or `height` < 3, `border_margin` < 0,
    /// or `stroke_width` is not a positive finite number.
    pub fn new(
        key_unit_length: i32,
        width_units: i32,
        height_units: i32,
        border_margin: i32,
        stroke_width: f32,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if key_unit_length <= 0 {
            return Err("--size must be greater than 0".into());
        }
        if width_units < 3 {
            return Err("--width must be at least 3".into());
        }
        if height_units < 3 {
            return Err("--height must be at least 3".into());
        }
        if border_margin < 0 {
            return Err("--border-margin must be non-negative".into());
        }
        if stroke_width <= 0.0 || !stroke_width.is_finite() {
            return Err("--stroke-width must be a positive finite number".into());
        }
        Ok(Self {
            key_unit_length,
            width_units,
            height_units,
            key_pattern_length: key_unit_length * 5,
            border_margin,
            stroke_width,
        })
    }

    pub(crate) fn get_canvas_size(&self) -> (f64, f64) {
        let width = (self.width_units * self.key_pattern_length
            + 2 * self.border_margin
            + 2 * self.key_unit_length) as f64
            + (2.0 * self.stroke_width) as f64;
        let height = (self.height_units * self.key_pattern_length
            + 2 * self.border_margin
            + 2 * self.key_unit_length) as f64
            + (2.0 * self.stroke_width) as f64;
        (width, height)
    }

    pub(crate) fn get_start_position(&self) -> (f64, f64) {
        let start_x = (self.border_margin + self.key_unit_length) as f64 + self.stroke_width as f64;
        let start_y = (self.key_pattern_length + self.border_margin + self.key_unit_length) as f64
            + self.stroke_width as f64;
        (start_x, start_y)
    }

    pub(crate) fn get_outer_frame_size(&self) -> (f64, f64, i32, i32) {
        let outer_x = self.border_margin as f64 + self.stroke_width as f64;
        let outer_y = self.border_margin as f64 + self.stroke_width as f64;
        let outer_width = self.width_units * self.key_pattern_length + 2 * self.key_unit_length;
        let outer_height = self.height_units * self.key_pattern_length + 2 * self.key_unit_length;
        (outer_x, outer_y, outer_width, outer_height)
    }

    pub(crate) fn get_inner_frame_size(&self) -> (f64, f64, i32, i32) {
        let inner_x =
            (6 * self.key_unit_length + self.border_margin) as f64 + self.stroke_width as f64;
        let inner_y =
            (6 * self.key_unit_length + self.border_margin) as f64 + self.stroke_width as f64;
        let inner_width = (self.width_units - 2) * self.key_pattern_length;
        let inner_height = (self.height_units - 2) * self.key_pattern_length;
        (inner_x, inner_y, inner_width, inner_height)
    }
}

/// Configuration for a circle Greek Key border pattern.
#[derive(Debug)]
pub struct GreekKeyCircleConfig {
    pub r_o: f64,
    pub pattern_count: i32,
    pub border_margin: i32,
    pub radii: Radii,
    pub stroke_width: f32,
}

#[derive(Debug)]
pub struct Radii {
    pub r_a: f64,
    pub r_b: f64,
    pub r_c: f64,
    pub r_d: f64,
    pub r_e: f64,
    pub r_o: f64,
    pub r_i: f64,
}

static PATTERN_UNIT_SIZE: i32 = 5;

type CirclePointSet = [Point; 6];
type CirclePatternPoints = (
    CirclePointSet,
    CirclePointSet,
    CirclePointSet,
    CirclePointSet,
    CirclePointSet,
);

type EllipsePointSet = [Point; 6];
type EllipsePatternPoints = (
    EllipsePointSet,
    EllipsePointSet,
    EllipsePointSet,
    EllipsePointSet,
    EllipsePointSet,
);

// Function to compute radii based on outer radius r_o and n
pub(crate) fn get_radii_for_outer_radius(r_o: f64, n: i32) -> Result<Radii, &'static str> {
    // Check if n >= 19
    if n < 19 {
        return Err("n must be greater or equal to 19");
    }

    // Compute r_c = r_o / (6 * PI / n + 1)
    let n_f64 = n as f64;
    let r_c = r_o / (6.0 * PI / n_f64 + 1.0);

    // Compute other radii based on the formulas
    let r_a = (5.0 * r_c - 2.0 * r_o) / 3.0;
    let r_b = (4.0 * r_c - r_o) / 3.0;
    let r_d = (2.0 * r_c + r_o) / 3.0;
    let r_e = (r_c + 2.0 * r_o) / 3.0;
    let r_i = (6.0 * r_c - 3.0 * r_o) / 3.0;

    // Return the results in a Radii struct
    Ok(Radii {
        r_a,
        r_b,
        r_c,
        r_d,
        r_e,
        r_o,
        r_i,
    })
}

// Function to calculate 6 points on a circle centered at (x0, y0) with radius r
pub(crate) fn calculate_circle_points(centre: Point, n: i32, p1: Point, r: f64) -> CirclePointSet {
    // Calculate u = 2 * r * PI / (5 * n)
    let u = (2.0 * r * PI) / (5.0 * n as f64);

    // Calculate theta1 = atan2(p1_y - y0, p1_x - x0)
    let theta1 = (p1.y - centre.y).atan2(p1.x - centre.x);

    // Calculate theta = u / r
    let theta = u / r;

    // Initialize array with the first point (p1_x, p1_y)
    let mut points = [
        Point { x: p1.x, y: p1.y },
        Point { x: 0.0, y: 0.0 },
        Point { x: 0.0, y: 0.0 },
        Point { x: 0.0, y: 0.0 },
        Point { x: 0.0, y: 0.0 },
        Point { x: 0.0, y: 0.0 },
    ];

    // Calculate the remaining 5 points
    for (i, point) in points.iter_mut().enumerate().skip(1) {
        let angle = theta1 + (i as f64) * theta;
        let x = centre.x + r * angle.cos();
        let y = centre.y + r * angle.sin();
        *point = Point { x, y };
    }

    points
}

/// Ellipse semi-axis values for the five concentric rings used in the key pattern.
#[derive(Debug)]
pub struct EllipseRadii {
    pub rx_a: f64,
    pub ry_a: f64,
    pub rx_b: f64,
    pub ry_b: f64,
    pub rx_c: f64,
    pub ry_c: f64,
    pub rx_d: f64,
    pub ry_d: f64,
    pub rx_e: f64,
    pub ry_e: f64,
    pub rx_i: f64,
    pub ry_i: f64,
}

pub(crate) fn get_ellipse_radii(rx: f64, ry: f64, n: i32) -> Result<EllipseRadii, &'static str> {
    let r = get_radii_for_outer_radius(1.0, n)?;
    Ok(EllipseRadii {
        rx_a: rx * r.r_a,
        ry_a: ry * r.r_a,
        rx_b: rx * r.r_b,
        ry_b: ry * r.r_b,
        rx_c: rx * r.r_c,
        ry_c: ry * r.r_c,
        rx_d: rx * r.r_d,
        ry_d: ry * r.r_d,
        rx_e: rx * r.r_e,
        ry_e: ry * r.r_e,
        rx_i: rx * r.r_i,
        ry_i: ry * r.r_i,
    })
}

pub(crate) fn calculate_ellipse_points(
    centre: Point,
    n: i32,
    p1: Point,
    rx: f64,
    ry: f64,
) -> EllipsePointSet {
    let theta1 = ((p1.y - centre.y) / ry).atan2((p1.x - centre.x) / rx);
    let theta = (2.0 * PI) / (5.0 * n as f64);
    let mut points = [
        Point { x: p1.x, y: p1.y },
        Point { x: 0.0, y: 0.0 },
        Point { x: 0.0, y: 0.0 },
        Point { x: 0.0, y: 0.0 },
        Point { x: 0.0, y: 0.0 },
        Point { x: 0.0, y: 0.0 },
    ];
    for (i, point) in points.iter_mut().enumerate().skip(1) {
        let angle = theta1 + (i as f64) * theta;
        *point = Point {
            x: centre.x + rx * angle.cos(),
            y: centre.y + ry * angle.sin(),
        };
    }
    points
}

/// Configuration for an ellipse Greek Key border pattern.
#[derive(Debug)]
pub struct GreekKeyEllipseConfig {
    pub rx: f64,
    pub ry: f64,
    pub pattern_count: i32,
    pub border_margin: i32,
    pub ellipse_radii: EllipseRadii,
    pub stroke_width: f32,
}

impl GreekKeyEllipseConfig {
    /// Creates a new ellipse config.
    ///
    /// Returns an error if `rx` or `ry` is not a positive finite number, `pattern_count` < 4,
    /// `border_margin` < 0, or `stroke_width` is not a positive finite number.
    pub fn new(
        rx: f64,
        ry: f64,
        pattern_count: i32,
        border_margin: i32,
        stroke_width: f32,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if rx <= 0.0 || !rx.is_finite() {
            return Err("--rx must be a positive finite number".into());
        }
        if ry <= 0.0 || !ry.is_finite() {
            return Err("--ry must be a positive finite number".into());
        }
        if pattern_count < 4 {
            return Err("--pattern-count must be at least 4".into());
        }
        if border_margin < 0 {
            return Err("--border-margin must be non-negative".into());
        }
        if stroke_width <= 0.0 || !stroke_width.is_finite() {
            return Err("--stroke-width must be a positive finite number".into());
        }
        let ellipse_radii = get_ellipse_radii(rx, ry, PATTERN_UNIT_SIZE * pattern_count)?;
        Ok(Self {
            rx,
            ry,
            pattern_count,
            border_margin,
            ellipse_radii,
            stroke_width,
        })
    }

    pub(crate) fn get_canvas_size(&self) -> (f64, f64) {
        let width =
            2.0 * self.rx + (2 * self.border_margin) as f64 + (2.0 * self.stroke_width) as f64;
        let height =
            2.0 * self.ry + (2 * self.border_margin) as f64 + (2.0 * self.stroke_width) as f64;
        (width, height)
    }

    pub(crate) fn get_centre(&self) -> Point {
        Point {
            x: self.border_margin as f64 + self.rx + self.stroke_width as f64,
            y: self.border_margin as f64 + self.ry + self.stroke_width as f64,
        }
    }

    pub(crate) fn get_coords_for_patterns(&self) -> EllipsePatternPoints {
        let centre = self.get_centre();
        let n = self.pattern_count;
        let er = &self.ellipse_radii;
        let start = |_rx_r: f64, ry_r: f64| Point {
            x: centre.x,
            y: centre.y - ry_r,
        };
        let points_a =
            calculate_ellipse_points(centre, n, start(er.rx_a, er.ry_a), er.rx_a, er.ry_a);
        let points_b =
            calculate_ellipse_points(centre, n, start(er.rx_b, er.ry_b), er.rx_b, er.ry_b);
        let points_c =
            calculate_ellipse_points(centre, n, start(er.rx_c, er.ry_c), er.rx_c, er.ry_c);
        let points_d =
            calculate_ellipse_points(centre, n, start(er.rx_d, er.ry_d), er.rx_d, er.ry_d);
        let points_e =
            calculate_ellipse_points(centre, n, start(er.rx_e, er.ry_e), er.rx_e, er.ry_e);
        (points_a, points_b, points_c, points_d, points_e)
    }

    pub(crate) fn get_coords_for_patterns_by_p0(
        &self,
        p_a0: Point,
        p_b0: Point,
        p_c0: Point,
        p_d0: Point,
        p_e0: Point,
    ) -> EllipsePatternPoints {
        let centre = self.get_centre();
        let er = &self.ellipse_radii;
        let points_a = calculate_ellipse_points(centre, self.pattern_count, p_a0, er.rx_a, er.ry_a);
        let points_b = calculate_ellipse_points(centre, self.pattern_count, p_b0, er.rx_b, er.ry_b);
        let points_c = calculate_ellipse_points(centre, self.pattern_count, p_c0, er.rx_c, er.ry_c);
        let points_d = calculate_ellipse_points(centre, self.pattern_count, p_d0, er.rx_d, er.ry_d);
        let points_e = calculate_ellipse_points(centre, self.pattern_count, p_e0, er.rx_e, er.ry_e);
        (points_a, points_b, points_c, points_d, points_e)
    }
}

impl GreekKeyCircleConfig {
    /// Creates a new circle config.
    ///
    /// Returns an error if `r_o` is not a positive finite number, `pattern_count` < 4,
    /// `border_margin` < 0,
    /// or `stroke_width` is not a positive finite number.
    pub fn new(
        r_o: f64,
        pattern_count: i32,
        border_margin: i32,
        stroke_width: f32,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if r_o <= 0.0 || !r_o.is_finite() {
            return Err("--radius must be a positive finite number".into());
        }
        if pattern_count < 4 {
            return Err("--pattern-count must be at least 4".into());
        }
        if border_margin < 0 {
            return Err("--border-margin must be non-negative".into());
        }
        if stroke_width <= 0.0 || !stroke_width.is_finite() {
            return Err("--stroke-width must be a positive finite number".into());
        }
        let radii = get_radii_for_outer_radius(r_o, PATTERN_UNIT_SIZE * pattern_count)?;
        Ok(Self {
            r_o,
            pattern_count,
            border_margin,
            radii,
            stroke_width,
        })
    }

    pub(crate) fn get_canvas_size(&self) -> (f64, f64) {
        let offset =
            2. * self.r_o + (2 * self.border_margin) as f64 + (2.0 * self.stroke_width) as f64;
        (offset, offset)
    }

    pub(crate) fn get_centre(&self) -> Point {
        let offset = self.border_margin as f64 + self.r_o + (self.stroke_width as f64);
        Point {
            x: offset,
            y: offset,
        }
    }

    pub(crate) fn get_coords_for_patterns(&self) -> CirclePatternPoints {
        let centre = self.get_centre();
        let n = self.pattern_count;
        let start = |r: f64| Point {
            x: centre.x,
            y: centre.y - r,
        };
        let points_a = calculate_circle_points(centre, n, start(self.radii.r_a), self.radii.r_a);
        let points_b = calculate_circle_points(centre, n, start(self.radii.r_b), self.radii.r_b);
        let points_c = calculate_circle_points(centre, n, start(self.radii.r_c), self.radii.r_c);
        let points_d = calculate_circle_points(centre, n, start(self.radii.r_d), self.radii.r_d);
        let points_e = calculate_circle_points(centre, n, start(self.radii.r_e), self.radii.r_e);
        (points_a, points_b, points_c, points_d, points_e)
    }

    pub(crate) fn get_coords_for_patterns_by_p0(
        &self,
        p_a0: Point,
        p_b0: Point,
        p_c0: Point,
        p_d0: Point,
        p_e0: Point,
    ) -> CirclePatternPoints {
        let centre = self.get_centre();
        let points_a = calculate_circle_points(centre, self.pattern_count, p_a0, self.radii.r_a);
        let points_b = calculate_circle_points(centre, self.pattern_count, p_b0, self.radii.r_b);
        let points_c = calculate_circle_points(centre, self.pattern_count, p_c0, self.radii.r_c);
        let points_d = calculate_circle_points(centre, self.pattern_count, p_d0, self.radii.r_d);
        let points_e = calculate_circle_points(centre, self.pattern_count, p_e0, self.radii.r_e);
        (points_a, points_b, points_c, points_d, points_e)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // --- GreekKeyRectConfig validation ---

    #[test]
    fn rect_valid() {
        assert!(GreekKeyRectConfig::new(25, 16, 9, 10, 3.0).is_ok());
    }

    #[test]
    fn rect_minimum_valid() {
        assert!(GreekKeyRectConfig::new(1, 3, 3, 0, 1.0).is_ok());
    }

    #[test]
    fn rect_zero_size_fails() {
        let e = GreekKeyRectConfig::new(0, 16, 9, 10, 3.0).unwrap_err();
        assert!(e.to_string().contains("--size"));
    }

    #[test]
    fn rect_negative_size_fails() {
        let e = GreekKeyRectConfig::new(-1, 16, 9, 10, 3.0).unwrap_err();
        assert!(e.to_string().contains("--size"));
    }

    #[test]
    fn rect_width_below_minimum_fails() {
        let e = GreekKeyRectConfig::new(25, 2, 9, 10, 3.0).unwrap_err();
        assert!(e.to_string().contains("--width"));
    }

    #[test]
    fn rect_height_below_minimum_fails() {
        let e = GreekKeyRectConfig::new(25, 16, 1, 10, 3.0).unwrap_err();
        assert!(e.to_string().contains("--height"));
    }

    #[test]
    fn rect_negative_margin_fails() {
        let e = GreekKeyRectConfig::new(25, 16, 9, -1, 3.0).unwrap_err();
        assert!(e.to_string().contains("--border-margin"));
    }

    #[test]
    fn rect_zero_stroke_width_fails() {
        let e = GreekKeyRectConfig::new(25, 16, 9, 10, 0.0).unwrap_err();
        assert!(e.to_string().contains("--stroke-width"));
    }

    #[test]
    fn rect_negative_stroke_width_fails() {
        let e = GreekKeyRectConfig::new(25, 16, 9, 10, -1.0).unwrap_err();
        assert!(e.to_string().contains("--stroke-width"));
    }

    #[test]
    fn rect_nan_stroke_width_fails() {
        let e = GreekKeyRectConfig::new(25, 16, 9, 10, f32::NAN).unwrap_err();
        assert!(e.to_string().contains("--stroke-width"));
    }

    // --- GreekKeyCircleConfig validation ---

    #[test]
    fn circle_valid() {
        assert!(GreekKeyCircleConfig::new(300.0, 30, 10, 3.0).is_ok());
    }

    #[test]
    fn circle_minimum_pattern_count() {
        assert!(GreekKeyCircleConfig::new(300.0, 4, 10, 3.0).is_ok());
    }

    #[test]
    fn circle_pattern_count_below_minimum_fails() {
        let e = GreekKeyCircleConfig::new(300.0, 3, 10, 3.0).unwrap_err();
        assert!(e.to_string().contains("--pattern-count"));
    }

    #[test]
    fn circle_zero_pattern_count_fails() {
        let e = GreekKeyCircleConfig::new(300.0, 0, 10, 3.0).unwrap_err();
        assert!(e.to_string().contains("--pattern-count"));
    }

    #[test]
    fn circle_zero_radius_fails() {
        let e = GreekKeyCircleConfig::new(0.0, 30, 10, 3.0).unwrap_err();
        assert!(e.to_string().contains("--radius"));
    }

    #[test]
    fn circle_negative_radius_fails() {
        let e = GreekKeyCircleConfig::new(-50.0, 30, 10, 3.0).unwrap_err();
        assert!(e.to_string().contains("--radius"));
    }

    #[test]
    fn circle_nan_radius_fails() {
        let e = GreekKeyCircleConfig::new(f64::NAN, 30, 10, 3.0).unwrap_err();
        assert!(e.to_string().contains("--radius"));
    }

    #[test]
    fn circle_infinite_radius_fails() {
        let e = GreekKeyCircleConfig::new(f64::INFINITY, 30, 10, 3.0).unwrap_err();
        assert!(e.to_string().contains("--radius"));
    }

    #[test]
    fn circle_negative_margin_fails() {
        let e = GreekKeyCircleConfig::new(300.0, 30, -1, 3.0).unwrap_err();
        assert!(e.to_string().contains("--border-margin"));
    }

    #[test]
    fn circle_zero_stroke_width_fails() {
        let e = GreekKeyCircleConfig::new(300.0, 30, 10, 0.0).unwrap_err();
        assert!(e.to_string().contains("--stroke-width"));
    }

    #[test]
    fn circle_negative_stroke_width_fails() {
        let e = GreekKeyCircleConfig::new(300.0, 30, 10, -1.0).unwrap_err();
        assert!(e.to_string().contains("--stroke-width"));
    }

    #[test]
    fn circle_nan_stroke_width_fails() {
        let e = GreekKeyCircleConfig::new(300.0, 30, 10, f32::NAN).unwrap_err();
        assert!(e.to_string().contains("--stroke-width"));
    }

    // --- GreekKeyEllipseConfig validation ---

    #[test]
    fn ellipse_valid() {
        assert!(GreekKeyEllipseConfig::new(300.0, 200.0, 30, 10, 3.0).is_ok());
    }

    #[test]
    fn ellipse_equal_axes_valid() {
        assert!(GreekKeyEllipseConfig::new(200.0, 200.0, 20, 5, 2.0).is_ok());
    }

    #[test]
    fn ellipse_minimum_pattern_count() {
        assert!(GreekKeyEllipseConfig::new(300.0, 200.0, 4, 10, 3.0).is_ok());
    }

    #[test]
    fn ellipse_pattern_count_below_minimum_fails() {
        let e = GreekKeyEllipseConfig::new(300.0, 200.0, 3, 10, 3.0).unwrap_err();
        assert!(e.to_string().contains("--pattern-count"));
    }

    #[test]
    fn ellipse_zero_rx_fails() {
        let e = GreekKeyEllipseConfig::new(0.0, 200.0, 30, 10, 3.0).unwrap_err();
        assert!(e.to_string().contains("--rx"));
    }

    #[test]
    fn ellipse_negative_rx_fails() {
        let e = GreekKeyEllipseConfig::new(-50.0, 200.0, 30, 10, 3.0).unwrap_err();
        assert!(e.to_string().contains("--rx"));
    }

    #[test]
    fn ellipse_nan_rx_fails() {
        let e = GreekKeyEllipseConfig::new(f64::NAN, 200.0, 30, 10, 3.0).unwrap_err();
        assert!(e.to_string().contains("--rx"));
    }

    #[test]
    fn ellipse_zero_ry_fails() {
        let e = GreekKeyEllipseConfig::new(300.0, 0.0, 30, 10, 3.0).unwrap_err();
        assert!(e.to_string().contains("--ry"));
    }

    #[test]
    fn ellipse_negative_ry_fails() {
        let e = GreekKeyEllipseConfig::new(300.0, -50.0, 30, 10, 3.0).unwrap_err();
        assert!(e.to_string().contains("--ry"));
    }

    #[test]
    fn ellipse_infinite_ry_fails() {
        let e = GreekKeyEllipseConfig::new(300.0, f64::INFINITY, 30, 10, 3.0).unwrap_err();
        assert!(e.to_string().contains("--ry"));
    }

    #[test]
    fn ellipse_negative_margin_fails() {
        let e = GreekKeyEllipseConfig::new(300.0, 200.0, 30, -1, 3.0).unwrap_err();
        assert!(e.to_string().contains("--border-margin"));
    }

    #[test]
    fn ellipse_zero_stroke_width_fails() {
        let e = GreekKeyEllipseConfig::new(300.0, 200.0, 30, 10, 0.0).unwrap_err();
        assert!(e.to_string().contains("--stroke-width"));
    }

    #[test]
    fn ellipse_canvas_size() {
        let config = GreekKeyEllipseConfig::new(300.0, 200.0, 30, 10, 3.0).unwrap();
        let (w, h) = config.get_canvas_size();
        assert!((w - (2.0 * 300.0 + 20.0 + 6.0)).abs() < 1e-6);
        assert!((h - (2.0 * 200.0 + 20.0 + 6.0)).abs() < 1e-6);
    }

    #[test]
    fn ellipse_radii_positive() {
        let config = GreekKeyEllipseConfig::new(300.0, 200.0, 30, 10, 3.0).unwrap();
        let er = &config.ellipse_radii;
        assert!(er.rx_a > 0.0 && er.ry_a > 0.0);
        assert!(er.rx_i > 0.0 && er.ry_i > 0.0);
        assert!(er.rx_e < config.rx && er.ry_e < config.ry);
    }

    #[cfg(feature = "native")]
    #[test]
    fn visual_options_round_trip_through_toml() {
        let visual = VisualOptions {
            stroke_color: "#123456".to_string(),
            stroke_opacity: 0.5,
            fill_color: Some("#AABBCC".to_string()),
            background_color: Some("#001122".to_string()),
            stroke_dash: Some("4,2".to_string()),
        };

        let toml = toml::to_string(&visual).unwrap();
        let parsed: VisualOptions = toml::from_str(&toml).unwrap();

        assert_eq!(parsed.stroke_color, visual.stroke_color);
        assert_eq!(parsed.stroke_opacity, visual.stroke_opacity);
        assert_eq!(parsed.fill_color, visual.fill_color);
        assert_eq!(parsed.background_color, visual.background_color);
        assert_eq!(parsed.stroke_dash, visual.stroke_dash);
    }
}
