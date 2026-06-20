use std::f64::consts::PI;

use crate::common::Point;

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
pub(crate) fn calculate_circle_points(centre: Point, n: i32, p1: Point, r: f64) -> [Point; 6] {
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
    for i in 1..6 {
        let angle = theta1 + (i as f64) * theta;
        let x = centre.x + r * angle.cos();
        let y = centre.y + r * angle.sin();
        points[i] = Point { x, y };
    }

    points
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

    pub(crate) fn get_coords_for_patterns(
        &self,
    ) -> ([Point; 6], [Point; 6], [Point; 6], [Point; 6], [Point; 6]) {
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
    ) -> ([Point; 6], [Point; 6], [Point; 6], [Point; 6], [Point; 6]) {
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
}
