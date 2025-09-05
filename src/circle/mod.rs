use resvg::render;
use resvg::usvg::Tree;

use svg::Document;
use svg::node::element::path::Data;
use svg::node::element::{Circle, Path as SvgPath};

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

pub fn generate_pattern_svg(
    config: &GreekKeyCircleConfig,
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
