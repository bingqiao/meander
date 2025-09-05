use std::f64::consts::PI;

pub struct GreekKeyRectConfig {
    pub key_unit_length: i32,
    pub width_units: i32,
    pub height_units: i32,
    pub key_pattern_length: i32,
    pub border_margin: i32,
    pub stroke_width: f32,
}

impl GreekKeyRectConfig {
    pub fn new(
        key_unit_length: i32,
        width_units: i32,
        height_units: i32,
        border_margin: i32,
        stroke_width: f32,
    ) -> Self {
        Self {
            key_unit_length,
            width_units,
            height_units,
            key_pattern_length: key_unit_length * 5,
            border_margin: border_margin,
            stroke_width: stroke_width,
        }
    }

    pub fn get_canvas_size(&self) -> (f64, f64) {
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

    pub fn get_start_position(&self) -> (f64, f64) {
        let start_x = (self.border_margin + self.key_unit_length) as f64 + self.stroke_width as f64;
        let start_y = (self.key_pattern_length + self.border_margin + self.key_unit_length) as f64
            + self.stroke_width as f64;
        (start_x, start_y)
    }

    pub fn get_outer_frame_size(&self) -> (f64, f64, i32, i32) {
        let outer_x = self.border_margin as f64 + self.stroke_width as f64;
        let outer_y = self.border_margin as f64 + self.stroke_width as f64;
        let outer_width = self.width_units * self.key_pattern_length + 2 * self.key_unit_length;
        let outer_height = self.height_units * self.key_pattern_length + 2 * self.key_unit_length;
        (outer_x, outer_y, outer_width, outer_height)
    }

    pub fn get_inner_frame_size(&self) -> (f64, f64, i32, i32) {
        let inner_x =
            (6 * self.key_unit_length + self.border_margin) as f64 + self.stroke_width as f64;
        let inner_y = (6 * self.key_unit_length) as f64 + self.stroke_width as f64;
        let inner_width = (self.width_units - 2) * self.key_pattern_length;
        let inner_height = (self.height_units - 2) * self.key_pattern_length;
        (inner_x, inner_y, inner_width, inner_height)
    }
}

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
pub fn get_radii_for_outer_radius(r_o: f64, n: i32) -> Result<Radii, &'static str> {
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
pub fn calculate_circle_points(centre: Point, n: i32, p1: Point, r: f64) -> [Point; 6] {
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

// Struct to represent a 2D point
#[derive(Debug, Clone, Copy)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl GreekKeyCircleConfig {
    pub fn new(r_o: f64, pattern_count: i32, border_margin: i32, stroke_width: f32) -> Self {
        let radii = get_radii_for_outer_radius(r_o, PATTERN_UNIT_SIZE * pattern_count).unwrap();
        Self {
            r_o,
            pattern_count,
            border_margin,
            radii,
            stroke_width: stroke_width,
        }
    }

    pub fn get_canvas_size(&self) -> (f64, f64) {
        let offset =
            2. * self.r_o + (2 * self.border_margin) as f64 + (2.0 * self.stroke_width) as f64;
        (offset, offset)
    }

    pub fn get_centre(&self) -> Point {
        let offset = self.border_margin as f64 + self.r_o + (self.stroke_width as f64);
        Point {
            x: offset,
            y: offset,
        }
    }

    pub fn get_start_position(&self, r: f64) -> Point {
        let centre = self.get_centre();
        Point {
            x: centre.x,
            y: centre.y - r,
        }
    }

    pub fn get_coords_for_patterns(
        &self,
    ) -> ([Point; 6], [Point; 6], [Point; 6], [Point; 6], [Point; 6]) {
        let centre = self.get_centre();
        let mut p1 = self.get_start_position(self.radii.r_a);
        let points_a = calculate_circle_points(centre, self.pattern_count, p1, self.radii.r_a);
        p1 = self.get_start_position(self.radii.r_b);
        let points_b = calculate_circle_points(centre, self.pattern_count, p1, self.radii.r_b);

        p1 = self.get_start_position(self.radii.r_c);
        let points_c = calculate_circle_points(centre, self.pattern_count, p1, self.radii.r_c);

        p1 = self.get_start_position(self.radii.r_d);
        let points_d = calculate_circle_points(centre, self.pattern_count, p1, self.radii.r_d);

        p1 = self.get_start_position(self.radii.r_e);
        let points_e = calculate_circle_points(centre, self.pattern_count, p1, self.radii.r_e);

        (points_a, points_b, points_c, points_d, points_e)
    }

    pub fn get_coords_for_patterns_by_p0(
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
