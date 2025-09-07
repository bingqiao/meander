use resvg::render;
use resvg::usvg::Tree;
use svg::Document;

// Struct to represent a 2D point
#[derive(Debug, Clone, Copy)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

pub fn save_and_convert_svg(
    document: Document,
    filename: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let svg_filename = format!("{}.svg", filename);
    svg::save(&svg_filename, &document)?;

    let png_filename = format!("{}.png", filename);
    let mut fontdb = resvg::usvg::fontdb::Database::new();
    fontdb.load_system_fonts();

    let tree = Tree::from_data(
        &std::fs::read(&svg_filename)?,
        &resvg::usvg::Options::default(),
    )?;

    let pixmap_size = tree.size().to_int_size();
    let mut pixmap =
        resvg::tiny_skia::Pixmap::new(pixmap_size.width(), pixmap_size.height()).unwrap();
    render(
        &tree,
        resvg::tiny_skia::Transform::identity(),
        &mut pixmap.as_mut(),
    );
    pixmap.save_png(png_filename)?;

    Ok(())
}
