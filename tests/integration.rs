use greek_meander::{
    circle, rect,
    config::{GreekKeyCircleConfig, GreekKeyRectConfig},
};
use std::path::PathBuf;

const PNG_MAGIC: [u8; 8] = [137, 80, 78, 71, 13, 10, 26, 10];

fn temp_path(name: &str) -> String {
    std::env::temp_dir()
        .join(name)
        .to_string_lossy()
        .into_owned()
}

struct TempFiles(Vec<String>);

impl TempFiles {
    fn for_base(base: &str) -> Self {
        Self(vec![format!("{}.svg", base), format!("{}.png", base)])
    }
}

impl Drop for TempFiles {
    fn drop(&mut self) {
        for path in &self.0 {
            let _ = std::fs::remove_file(path);
        }
    }
}

// --- rect::generate_pattern_svg ---

#[test]
fn rect_creates_svg_and_png() {
    let config = GreekKeyRectConfig::new(10, 4, 4, 5, 2.0).unwrap();
    let path = temp_path("gm_test_rect");
    let _guard = TempFiles::for_base(&path);
    rect::generate_pattern_svg(&config, "#000000", 1.0, &path).unwrap();
    assert!(PathBuf::from(format!("{}.svg", path)).exists());
    assert!(PathBuf::from(format!("{}.png", path)).exists());
}

#[test]
fn rect_svg_has_valid_content() {
    let config = GreekKeyRectConfig::new(10, 4, 4, 5, 2.0).unwrap();
    let path = temp_path("gm_test_rect_svg");
    let _guard = TempFiles::for_base(&path);
    rect::generate_pattern_svg(&config, "#AB8E0E", 0.7, &path).unwrap();
    let content = std::fs::read_to_string(format!("{}.svg", path)).unwrap();
    assert!(content.contains("<svg"), "SVG output should contain an <svg> element");
    assert!(content.contains("viewBox"), "SVG output should contain a viewBox attribute");
}

#[test]
fn rect_png_has_valid_magic_bytes() {
    let config = GreekKeyRectConfig::new(10, 4, 4, 5, 2.0).unwrap();
    let path = temp_path("gm_test_rect_png");
    let _guard = TempFiles::for_base(&path);
    rect::generate_pattern_svg(&config, "#AB8E0E", 0.7, &path).unwrap();
    let bytes = std::fs::read(format!("{}.png", path)).unwrap();
    assert!(
        bytes.starts_with(&PNG_MAGIC),
        "PNG output should start with the PNG magic bytes"
    );
}

// --- circle::generate_pattern_svg ---

#[test]
fn circle_creates_svg_and_png() {
    let config = GreekKeyCircleConfig::new(100.0, 10, 5, 2.0).unwrap();
    let path = temp_path("gm_test_circle");
    let _guard = TempFiles::for_base(&path);
    circle::generate_pattern_svg(&config, "#000000", 1.0, &path).unwrap();
    assert!(PathBuf::from(format!("{}.svg", path)).exists());
    assert!(PathBuf::from(format!("{}.png", path)).exists());
}

#[test]
fn circle_svg_has_valid_content() {
    let config = GreekKeyCircleConfig::new(100.0, 10, 5, 2.0).unwrap();
    let path = temp_path("gm_test_circle_svg");
    let _guard = TempFiles::for_base(&path);
    circle::generate_pattern_svg(&config, "#AB8E0E", 0.7, &path).unwrap();
    let content = std::fs::read_to_string(format!("{}.svg", path)).unwrap();
    assert!(content.contains("<svg"), "SVG output should contain an <svg> element");
    assert!(content.contains("viewBox"), "SVG output should contain a viewBox attribute");
}

#[test]
fn circle_png_has_valid_magic_bytes() {
    let config = GreekKeyCircleConfig::new(100.0, 10, 5, 2.0).unwrap();
    let path = temp_path("gm_test_circle_png");
    let _guard = TempFiles::for_base(&path);
    circle::generate_pattern_svg(&config, "#AB8E0E", 0.7, &path).unwrap();
    let bytes = std::fs::read(format!("{}.png", path)).unwrap();
    assert!(
        bytes.starts_with(&PNG_MAGIC),
        "PNG output should start with the PNG magic bytes"
    );
}

// --- public type surface ---

#[test]
fn point_fields_are_accessible() {
    let p = greek_meander::Point { x: 1.5, y: 2.5 };
    assert_eq!(p.x, 1.5);
    assert_eq!(p.y, 2.5);
}

#[test]
fn radii_fields_are_accessible() {
    let config = GreekKeyCircleConfig::new(300.0, 30, 10, 3.0).unwrap();
    // All seven Radii fields must be reachable from outside the crate
    let _ = config.radii.r_i;
    let _ = config.radii.r_a;
    let _ = config.radii.r_b;
    let _ = config.radii.r_c;
    let _ = config.radii.r_d;
    let _ = config.radii.r_e;
    // Outer radius must equal the value passed to new()
    assert_eq!(config.radii.r_o, 300.0);
}

#[test]
fn radii_type_is_nameable_at_crate_root() {
    // Radii must be re-exported so consumers can name it without the config:: prefix
    let config = GreekKeyCircleConfig::new(300.0, 30, 10, 3.0).unwrap();
    let _r: greek_meander::Radii = config.radii;
    assert_eq!(_r.r_o, 300.0);
}
