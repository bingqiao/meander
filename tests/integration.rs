use greek_meander::{
    circle,
    config::{GreekKeyCircleConfig, GreekKeyRectConfig},
    rect,
};
#[cfg(feature = "native")]
use std::path::PathBuf;
#[cfg(feature = "native")]
use std::process::Command;

#[cfg(feature = "native")]
const PNG_MAGIC: [u8; 8] = [137, 80, 78, 71, 13, 10, 26, 10];

#[cfg(feature = "native")]
fn temp_path(name: &str) -> String {
    std::env::temp_dir()
        .join(name)
        .to_string_lossy()
        .into_owned()
}

#[cfg(feature = "native")]
struct TempFiles(Vec<String>);

#[cfg(feature = "native")]
impl TempFiles {
    fn for_base(base: &str) -> Self {
        Self(vec![format!("{}.svg", base), format!("{}.png", base)])
    }
}

#[cfg(feature = "native")]
impl Drop for TempFiles {
    fn drop(&mut self) {
        for path in &self.0 {
            let _ = std::fs::remove_file(path);
        }
    }
}

#[cfg(feature = "native")]
fn png_dimensions(path: &str) -> (u32, u32) {
    let bytes = std::fs::read(path).unwrap();
    assert!(bytes.starts_with(&PNG_MAGIC));
    let width = u32::from_be_bytes(bytes[16..20].try_into().unwrap());
    let height = u32::from_be_bytes(bytes[20..24].try_into().unwrap());
    (width, height)
}

// --- rect::generate_pattern_svg (native only) ---

#[cfg(feature = "native")]
#[test]
fn rect_creates_svg_and_png() {
    let config = GreekKeyRectConfig::new(10, 4, 4, 5, 2.0).unwrap();
    let path = temp_path("gm_test_rect");
    let _guard = TempFiles::for_base(&path);
    rect::generate_pattern_svg(&config, "#000000", 1.0, &path).unwrap();
    assert!(PathBuf::from(format!("{}.svg", path)).exists());
    assert!(PathBuf::from(format!("{}.png", path)).exists());
}

#[cfg(feature = "native")]
#[test]
fn rect_svg_has_valid_content() {
    let config = GreekKeyRectConfig::new(10, 4, 4, 5, 2.0).unwrap();
    let path = temp_path("gm_test_rect_svg");
    let _guard = TempFiles::for_base(&path);
    rect::generate_pattern_svg(&config, "#AB8E0E", 0.7, &path).unwrap();
    let content = std::fs::read_to_string(format!("{}.svg", path)).unwrap();
    assert!(
        content.contains("<svg"),
        "SVG output should contain an <svg> element"
    );
    assert!(
        content.contains("viewBox"),
        "SVG output should contain a viewBox attribute"
    );
}

#[cfg(feature = "native")]
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

#[cfg(feature = "native")]
#[test]
fn cli_can_skip_png() {
    let path = temp_path("gm_test_skip_png");
    let _guard = TempFiles::for_base(&path);

    let output = Command::new(env!("CARGO_BIN_EXE_greek-meander"))
        .args([
            "--no-png", "--file", &path, "rect", "--size", "10", "--width", "4", "--height", "4",
        ])
        .output()
        .unwrap();

    assert!(output.status.success());
    assert!(PathBuf::from(format!("{}.svg", path)).exists());
    assert!(!PathBuf::from(format!("{}.png", path)).exists());
}

#[cfg(feature = "native")]
#[test]
fn cli_can_skip_svg() {
    let path = temp_path("gm_test_skip_svg");
    let _guard = TempFiles::for_base(&path);

    let output = Command::new(env!("CARGO_BIN_EXE_greek-meander"))
        .args([
            "--no-svg", "--file", &path, "rect", "--size", "10", "--width", "4", "--height", "4",
        ])
        .output()
        .unwrap();

    assert!(output.status.success());
    assert!(!PathBuf::from(format!("{}.svg", path)).exists());
    assert!(PathBuf::from(format!("{}.png", path)).exists());
}

#[cfg(feature = "native")]
#[test]
fn cli_scale_changes_png_dimensions() {
    let normal_path = temp_path("gm_test_scale_normal");
    let scaled_path = temp_path("gm_test_scale_scaled");
    let _normal_guard = TempFiles::for_base(&normal_path);
    let _scaled_guard = TempFiles::for_base(&scaled_path);

    let normal_output = Command::new(env!("CARGO_BIN_EXE_greek-meander"))
        .args([
            "--no-svg",
            "--file",
            &normal_path,
            "rect",
            "--size",
            "10",
            "--width",
            "4",
            "--height",
            "4",
        ])
        .output()
        .unwrap();
    let scaled_output = Command::new(env!("CARGO_BIN_EXE_greek-meander"))
        .args([
            "--no-svg",
            "--scale",
            "2",
            "--file",
            &scaled_path,
            "rect",
            "--size",
            "10",
            "--width",
            "4",
            "--height",
            "4",
        ])
        .output()
        .unwrap();

    assert!(normal_output.status.success());
    assert!(scaled_output.status.success());

    let normal = png_dimensions(&format!("{}.png", normal_path));
    let scaled = png_dimensions(&format!("{}.png", scaled_path));
    assert_eq!(scaled, (normal.0 * 2, normal.1 * 2));
}

#[cfg(feature = "native")]
#[test]
fn cli_stdout_only_writes_svg_to_stdout_without_files() {
    let path = temp_path("gm_test_stdout_only");
    let _guard = TempFiles::for_base(&path);

    let output = Command::new(env!("CARGO_BIN_EXE_greek-meander"))
        .args([
            "--stdout", "--no-svg", "--no-png", "--file", &path, "rect", "--size", "10", "--width",
            "4", "--height", "4",
        ])
        .output()
        .unwrap();

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("<svg"));
    assert!(stdout.contains("<path"));
    assert!(output.stderr.is_empty());
    assert!(!PathBuf::from(format!("{}.svg", path)).exists());
    assert!(!PathBuf::from(format!("{}.png", path)).exists());
}

#[cfg(feature = "native")]
#[test]
fn cli_rejects_no_output() {
    let output = Command::new(env!("CARGO_BIN_EXE_greek-meander"))
        .args(["--no-svg", "--no-png", "rect"])
        .output()
        .unwrap();

    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(stderr.contains("at least one output is required"));
}

// --- circle::generate_pattern_svg (native only) ---

#[cfg(feature = "native")]
#[test]
fn circle_creates_svg_and_png() {
    let config = GreekKeyCircleConfig::new(100.0, 10, 5, 2.0).unwrap();
    let path = temp_path("gm_test_circle");
    let _guard = TempFiles::for_base(&path);
    circle::generate_pattern_svg(&config, "#000000", 1.0, &path).unwrap();
    assert!(PathBuf::from(format!("{}.svg", path)).exists());
    assert!(PathBuf::from(format!("{}.png", path)).exists());
}

#[cfg(feature = "native")]
#[test]
fn circle_svg_has_valid_content() {
    let config = GreekKeyCircleConfig::new(100.0, 10, 5, 2.0).unwrap();
    let path = temp_path("gm_test_circle_svg");
    let _guard = TempFiles::for_base(&path);
    circle::generate_pattern_svg(&config, "#AB8E0E", 0.7, &path).unwrap();
    let content = std::fs::read_to_string(format!("{}.svg", path)).unwrap();
    assert!(
        content.contains("<svg"),
        "SVG output should contain an <svg> element"
    );
    assert!(
        content.contains("viewBox"),
        "SVG output should contain a viewBox attribute"
    );
}

#[cfg(feature = "native")]
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

// --- generate_svg_string (always available, WASM-safe) ---

#[test]
fn rect_svg_string_is_valid_svg() {
    let config = GreekKeyRectConfig::new(10, 4, 4, 5, 2.0).unwrap();
    let svg = rect::generate_svg_string(&config, "#AB8E0E", 0.7);
    assert!(
        svg.contains("<svg"),
        "svg string should contain an <svg> element"
    );
    assert!(
        svg.contains("viewBox"),
        "svg string should contain a viewBox attribute"
    );
    assert!(svg.contains("<path"), "svg string should contain path data");
}

#[test]
fn circle_svg_string_is_valid_svg() {
    let config = GreekKeyCircleConfig::new(100.0, 10, 5, 2.0).unwrap();
    let svg = circle::generate_svg_string(&config, "#AB8E0E", 0.7);
    assert!(
        svg.contains("<svg"),
        "svg string should contain an <svg> element"
    );
    assert!(
        svg.contains("viewBox"),
        "svg string should contain a viewBox attribute"
    );
    assert!(svg.contains("<path"), "svg string should contain path data");
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
    let _ = config.radii.r_i;
    let _ = config.radii.r_a;
    let _ = config.radii.r_b;
    let _ = config.radii.r_c;
    let _ = config.radii.r_d;
    let _ = config.radii.r_e;
    assert_eq!(config.radii.r_o, 300.0);
}

#[test]
fn radii_type_is_nameable_at_crate_root() {
    let config = GreekKeyCircleConfig::new(300.0, 30, 10, 3.0).unwrap();
    let _r: greek_meander::Radii = config.radii;
    assert_eq!(_r.r_o, 300.0);
}
