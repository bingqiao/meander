pub struct GreekKeyConfig {
    pub key_unit_length: i32,
    pub width_units: i32,
    pub height_units: i32,
    pub key_pattern_length: i32,
}

impl GreekKeyConfig {
    pub fn new(key_unit_length: i32, width_units: i32, height_units: i32) -> Self {
        Self {
            key_unit_length,
            width_units,
            height_units,
            key_pattern_length: key_unit_length * 5,
        }
    }

    pub fn get_canvas_size(&self) -> (i32, i32) {
        let width = self.width_units * self.key_pattern_length + 3 * self.key_unit_length;
        let height = self.height_units * self.key_pattern_length + 3 * self.key_unit_length;
        (width, height)
    }

    pub fn get_start_position(&self) -> (i32, i32) {
        let start_x = (1.5 * self.key_unit_length as f32) as i32;
        let start_y = (6.5 * self.key_unit_length as f32) as i32;
        (start_x, start_y)
    }

    pub fn get_outer_frame_size(&self) -> (i32, i32, i32, i32) {
        let outer_x = (0.5 * self.key_unit_length as f32) as i32;
        let outer_y = (0.5 * self.key_unit_length as f32) as i32;
        let outer_width = self.width_units * self.key_pattern_length + 2 * self.key_unit_length;
        let outer_height = self.height_units * self.key_pattern_length + 2 * self.key_unit_length;
        (outer_x, outer_y, outer_width, outer_height)
    }

    pub fn get_inner_frame_size(&self) -> (i32, i32, i32, i32) {
        let inner_x = (6.5 * self.key_unit_length as f32) as i32;
        let inner_y = (6.5 * self.key_unit_length as f32) as i32;
        let inner_width = (self.width_units - 2) * self.key_pattern_length;
        let inner_height = (self.height_units - 2) * self.key_pattern_length;
        (inner_x, inner_y, inner_width, inner_height)
    }
}
