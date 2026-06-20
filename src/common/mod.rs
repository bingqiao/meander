#[cfg(feature = "native")]
use resvg::render;
#[cfg(feature = "native")]
use resvg::usvg::Tree;
#[cfg(feature = "native")]
use svg::Document;

#[derive(Debug, Clone, Copy)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

#[cfg(feature = "native")]
pub(crate) fn save_and_convert_svg(
    document: Document,
    filename: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let svg_filename = format!("{}.svg", filename);
    let png_filename = format!("{}.png", filename);

    // Serialize once; write to .svg and reuse the bytes for rasterization.
    let mut svg_content = Vec::new();
    svg::write(&mut svg_content, &document)?;
    std::fs::write(&svg_filename, &svg_content)?;

    let tree = Tree::from_data(&svg_content, &resvg::usvg::Options::default())?;

    let pixmap_size = tree.size().to_int_size();
    let mut pixmap = resvg::tiny_skia::Pixmap::new(pixmap_size.width(), pixmap_size.height())
        .ok_or("canvas has zero dimensions")?;
    render(
        &tree,
        resvg::tiny_skia::Transform::identity(),
        &mut pixmap.as_mut(),
    );
    pixmap.save_png(png_filename)?;

    Ok(())
}
