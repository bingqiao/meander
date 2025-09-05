use resvg::render;
use resvg::usvg::Tree;

use svg::Document;
use svg::node::element::Path as SvgPath;
use svg::node::element::path::Data;

use crate::config::GreekKeyRectConfig;

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

pub fn generate_pattern_svg(
    config: &GreekKeyRectConfig,
    stroke_width: f32,
    stroke_color: &str,
    stroke_opacity: f32,
    filename: &str,
) {
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

    let (outer_x, outer_y, outer_width, outer_height) = config.get_outer_frame_size();
    let outer_frame = draw_frame(
        outer_x,
        outer_y,
        outer_width,
        outer_height,
        stroke_color,
        stroke_width,
        stroke_opacity,
    );
    document = document.add(outer_frame);

    let (inner_x, inner_y, inner_width, inner_height) = config.get_inner_frame_size();
    let inner_frame = draw_frame(
        inner_x,
        inner_y,
        inner_width,
        inner_height,
        stroke_color,
        stroke_width,
        stroke_opacity,
    );
    document = document.add(inner_frame);

    let svg_filename = format!("{}.svg", filename);
    svg::save(&svg_filename, &document).unwrap();

    let png_filename = format!("{}.png", filename);
    let mut fontdb = resvg::usvg::fontdb::Database::new();
    fontdb.load_system_fonts();

    let tree = Tree::from_data(
        &std::fs::read(&svg_filename).unwrap(),
        &resvg::usvg::Options::default(),
    )
    .unwrap();

    let pixmap_size = tree.size().to_int_size();
    let mut pixmap =
        resvg::tiny_skia::Pixmap::new(pixmap_size.width(), pixmap_size.height()).unwrap();
    render(
        &tree,
        resvg::tiny_skia::Transform::identity(),
        &mut pixmap.as_mut(),
    );
    pixmap.save_png(png_filename).unwrap();
}
