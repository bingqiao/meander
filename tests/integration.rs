use greek_meander::{
    circle,
    config::{GreekKeyCircleConfig, GreekKeyRectConfig, VisualOptions},
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
    rect::generate_pattern_svg(&config, &VisualOptions::new("#000000", 1.0), &path).unwrap();
    assert!(PathBuf::from(format!("{}.svg", path)).exists());
    assert!(PathBuf::from(format!("{}.png", path)).exists());
}

#[cfg(feature = "native")]
#[test]
fn rect_svg_has_valid_content() {
    let config = GreekKeyRectConfig::new(10, 4, 4, 5, 2.0).unwrap();
    let path = temp_path("gm_test_rect_svg");
    let _guard = TempFiles::for_base(&path);
    rect::generate_pattern_svg(&config, &VisualOptions::default(), &path).unwrap();
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
    rect::generate_pattern_svg(&config, &VisualOptions::default(), &path).unwrap();
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
    circle::generate_pattern_svg(&config, &VisualOptions::new("#000000", 1.0), &path).unwrap();
    assert!(PathBuf::from(format!("{}.svg", path)).exists());
    assert!(PathBuf::from(format!("{}.png", path)).exists());
}

#[cfg(feature = "native")]
#[test]
fn circle_svg_has_valid_content() {
    let config = GreekKeyCircleConfig::new(100.0, 10, 5, 2.0).unwrap();
    let path = temp_path("gm_test_circle_svg");
    let _guard = TempFiles::for_base(&path);
    circle::generate_pattern_svg(&config, &VisualOptions::default(), &path).unwrap();
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
    circle::generate_pattern_svg(&config, &VisualOptions::default(), &path).unwrap();
    let bytes = std::fs::read(format!("{}.png", path)).unwrap();
    assert!(
        bytes.starts_with(&PNG_MAGIC),
        "PNG output should start with the PNG magic bytes"
    );
}

// --- --config file input ---

#[cfg(feature = "native")]
fn write_temp_config(name: &str, content: &str) -> std::path::PathBuf {
    let path = std::env::temp_dir().join(format!(
        "{}_{}_{}.toml",
        name,
        std::process::id(),
        std::thread::current().name().unwrap_or("test")
    ));
    std::fs::write(&path, content).unwrap();
    path
}

#[cfg(feature = "native")]
#[test]
fn config_file_sets_rect_params() {
    let out = temp_path("gm_test_cfg_rect");
    let _guard = TempFiles::for_base(&out);
    let cfg = write_temp_config(
        "gm_test_cfg_rect",
        &format!(
            r#"
file = "{out}"
stroke_width = 3.0
[rect]
size = 10
width = 4
height = 4
"#
        ),
    );

    let output = Command::new(env!("CARGO_BIN_EXE_greek-meander"))
        .args(["--config", cfg.to_str().unwrap(), "--no-png", "rect"])
        .output()
        .unwrap();

    let _ = std::fs::remove_file(&cfg);
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let svg = std::fs::read_to_string(format!("{out}.svg")).unwrap();
    assert!(
        svg.contains(r#"viewBox="0 0 228 228""#),
        "rect config dimensions should come from the TOML file"
    );
    assert!(
        svg.contains(r#"stroke-width="3""#),
        "shared config fields should apply to generated SVG"
    );
}

#[cfg(feature = "native")]
#[test]
fn config_file_sets_circle_params() {
    let out = temp_path("gm_test_cfg_circle");
    let _guard = TempFiles::for_base(&out);
    let cfg = write_temp_config(
        "gm_test_cfg_circle",
        &format!(
            r#"
file = "{out}"
[circle]
pattern_count = 10
radius = 100.0
"#
        ),
    );

    let output = Command::new(env!("CARGO_BIN_EXE_greek-meander"))
        .args(["--config", cfg.to_str().unwrap(), "--no-png", "circle"])
        .output()
        .unwrap();

    let _ = std::fs::remove_file(&cfg);
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let svg = std::fs::read_to_string(format!("{out}.svg")).unwrap();
    assert!(
        svg.contains(r#"viewBox="0 0 214 214""#),
        "circle radius should come from the TOML file"
    );
    assert!(
        svg.contains(r#"r="100""#),
        "outer circle radius should come from the TOML file"
    );
}

#[cfg(feature = "native")]
#[test]
fn config_file_scale_changes_png_dimensions() {
    let normal_path = temp_path("gm_test_cfg_scale_normal");
    let scaled_path = temp_path("gm_test_cfg_scale_scaled");
    let _normal_guard = TempFiles::for_base(&normal_path);
    let _scaled_guard = TempFiles::for_base(&scaled_path);
    let cfg = write_temp_config(
        "gm_test_cfg_scale",
        &format!(
            r#"
file = "{scaled_path}"
scale = 2.0
[rect]
size = 10
width = 4
height = 4
"#
        ),
    );

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
        .args(["--config", cfg.to_str().unwrap(), "--no-svg", "rect"])
        .output()
        .unwrap();

    let _ = std::fs::remove_file(&cfg);
    assert!(
        normal_output.status.success(),
        "{}",
        String::from_utf8_lossy(&normal_output.stderr)
    );
    assert!(
        scaled_output.status.success(),
        "{}",
        String::from_utf8_lossy(&scaled_output.stderr)
    );

    let normal = png_dimensions(&format!("{}.png", normal_path));
    let scaled = png_dimensions(&format!("{}.png", scaled_path));
    assert_eq!(scaled, (normal.0 * 2, normal.1 * 2));
}

#[cfg(feature = "native")]
#[test]
fn cli_flag_overrides_config_file() {
    let cfg_out = temp_path("gm_test_override_cfg");
    let cli_out = temp_path("gm_test_override_cli");
    let _cfg_guard = TempFiles::for_base(&cfg_out);
    let _cli_guard = TempFiles::for_base(&cli_out);

    let cfg = write_temp_config(
        "gm_test_override",
        &format!(
            r#"
file = "{cfg_out}"
stroke_width = 2.0
[rect]
size = 10
width = 4
height = 4
"#
        ),
    );

    // CLI --file overrides config file's file setting
    let output = Command::new(env!("CARGO_BIN_EXE_greek-meander"))
        .args([
            "--config",
            cfg.to_str().unwrap(),
            "--file",
            &cli_out,
            "--stroke-width",
            "4",
            "--no-png",
            "rect",
            "--size",
            "20",
        ])
        .output()
        .unwrap();

    let _ = std::fs::remove_file(&cfg);
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(
        !PathBuf::from(format!("{cfg_out}.svg")).exists(),
        "config file path should not be written"
    );
    let svg = std::fs::read_to_string(format!("{cli_out}.svg")).unwrap();
    assert!(
        svg.contains(r#"viewBox="0 0 450 450""#),
        "CLI shape flags should override shape values from the config file"
    );
    assert!(
        svg.contains(r#"stroke-width="4""#),
        "CLI shared flags should override shared values from the config file"
    );
}

#[cfg(feature = "native")]
#[test]
fn invalid_config_value_gives_validation_error() {
    let cfg = write_temp_config(
        "gm_test_invalid_value_cfg",
        r#"
[rect]
width = 2
"#,
    );
    let output = Command::new(env!("CARGO_BIN_EXE_greek-meander"))
        .args(["--config", cfg.to_str().unwrap(), "--no-png", "rect"])
        .output()
        .unwrap();

    let _ = std::fs::remove_file(&cfg);
    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(
        stderr.contains("--width"),
        "invalid config values should use the normal validation errors"
    );
}

#[cfg(feature = "native")]
#[test]
fn config_file_missing_gives_error() {
    let path = std::env::temp_dir().join(format!(
        "nonexistent_gm_config_{}_{}.toml",
        std::process::id(),
        "missing"
    ));
    let output = Command::new(env!("CARGO_BIN_EXE_greek-meander"))
        .args(["--config", path.to_str().unwrap(), "rect"])
        .output()
        .unwrap();

    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(stderr.contains("Error"), "should report an error");
}

#[cfg(feature = "native")]
#[test]
fn config_file_with_unknown_field_is_accepted() {
    // Unknown fields are silently ignored for forward compatibility: a config
    // written for a newer binary must not break on an older one.
    let out = temp_path("gm_test_unknown_cfg");
    let _guard = TempFiles::for_base(&out);
    let cfg = write_temp_config(
        "gm_test_bad_cfg",
        &format!("unknown_key = 42\nfile = \"{out}\"\n"),
    );
    let output = Command::new(env!("CARGO_BIN_EXE_greek-meander"))
        .args(["--config", cfg.to_str().unwrap(), "--no-png", "rect"])
        .output()
        .unwrap();

    let _ = std::fs::remove_file(&cfg);
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(PathBuf::from(format!("{out}.svg")).exists());
}

#[cfg(feature = "native")]
#[test]
fn malformed_config_file_gives_error() {
    let cfg = write_temp_config("gm_test_malformed_cfg", "stroke_width = [");
    let output = Command::new(env!("CARGO_BIN_EXE_greek-meander"))
        .args(["--config", cfg.to_str().unwrap(), "rect"])
        .output()
        .unwrap();

    let _ = std::fs::remove_file(&cfg);
    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(
        stderr.contains("invalid config file"),
        "malformed TOML should be reported as a config parse error"
    );
}

// --- generate_svg_string (always available, WASM-safe) ---

#[test]
fn rect_svg_string_is_valid_svg() {
    let config = GreekKeyRectConfig::new(10, 4, 4, 5, 2.0).unwrap();
    let svg = rect::generate_svg_string(&config, &VisualOptions::default());
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
    let svg = circle::generate_svg_string(&config, &VisualOptions::default());
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

// --- VisualOptions: SVG structure ---

#[test]
fn default_output_has_no_fill_no_background_no_dash() {
    let config = GreekKeyRectConfig::new(10, 4, 4, 5, 2.0).unwrap();
    let svg = rect::generate_svg_string(&config, &VisualOptions::default());
    assert!(
        svg.contains(r#"fill="none""#),
        "default fill should be none"
    );
    assert!(!svg.contains("stroke-dasharray"), "no dasharray by default");
    assert!(!svg.contains("<rect"), "no background rect by default");
}

#[test]
fn fill_color_appears_on_pattern_path() {
    let config = GreekKeyRectConfig::new(10, 4, 4, 5, 2.0).unwrap();
    let visual = VisualOptions {
        fill_color: Some("#FF0000".to_string()),
        ..Default::default()
    };
    let svg = rect::generate_svg_string(&config, &visual);
    assert!(
        svg.contains("fill=\"#FF0000\""),
        "fill color should appear on the path element"
    );
}

#[test]
fn background_color_adds_rect_element() {
    let config = GreekKeyRectConfig::new(10, 4, 4, 5, 2.0).unwrap();
    let visual = VisualOptions {
        background_color: Some("#000000".to_string()),
        ..Default::default()
    };
    let svg = rect::generate_svg_string(&config, &visual);
    assert!(
        svg.contains("<rect"),
        "background color should add a rect element"
    );
    assert!(
        svg.contains("fill=\"#000000\""),
        "background rect should have the specified fill color"
    );
}

#[test]
fn stroke_dash_appears_on_path_and_frames() {
    let config = GreekKeyRectConfig::new(10, 4, 4, 5, 2.0).unwrap();
    let visual = VisualOptions {
        stroke_dash: Some("5,3".to_string()),
        ..Default::default()
    };
    let svg = rect::generate_svg_string(&config, &visual);
    let count = svg.matches("stroke-dasharray").count();
    assert!(
        count >= 3,
        "stroke-dasharray should appear on pattern path and both frames, got {count}"
    );
}

#[test]
fn circle_fill_color_appears_on_pattern_path() {
    let config = GreekKeyCircleConfig::new(100.0, 10, 5, 2.0).unwrap();
    let visual = VisualOptions {
        fill_color: Some("#00FF00".to_string()),
        ..Default::default()
    };
    let svg = circle::generate_svg_string(&config, &visual);
    assert!(svg.contains("fill=\"#00FF00\""));
}

#[test]
fn circle_background_color_adds_rect_element() {
    let config = GreekKeyCircleConfig::new(100.0, 10, 5, 2.0).unwrap();
    let visual = VisualOptions {
        background_color: Some("#111111".to_string()),
        ..Default::default()
    };
    let svg = circle::generate_svg_string(&config, &visual);
    assert!(svg.contains("<rect"));
    assert!(svg.contains("fill=\"#111111\""));
}

#[test]
fn circle_stroke_dash_appears_on_path_and_frames() {
    let config = GreekKeyCircleConfig::new(100.0, 10, 5, 2.0).unwrap();
    let visual = VisualOptions {
        stroke_dash: Some("8,4".to_string()),
        ..Default::default()
    };
    let svg = circle::generate_svg_string(&config, &visual);
    let count = svg.matches("stroke-dasharray").count();
    assert!(
        count >= 3,
        "stroke-dasharray should appear on pattern path and both circle frames, got {count}"
    );
}

#[cfg(feature = "native")]
#[test]
fn cli_fill_color_appears_in_svg() {
    let path = temp_path("gm_test_fill_color");
    let _guard = TempFiles::for_base(&path);

    let output = Command::new(env!("CARGO_BIN_EXE_greek-meander"))
        .args([
            "--fill-color",
            "#FF0000",
            "--no-png",
            "--file",
            &path,
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

    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let svg = std::fs::read_to_string(format!("{path}.svg")).unwrap();
    assert!(svg.contains("fill=\"#FF0000\""));
}

#[cfg(feature = "native")]
#[test]
fn cli_background_color_adds_rect_to_svg() {
    let path = temp_path("gm_test_background_color");
    let _guard = TempFiles::for_base(&path);

    let output = Command::new(env!("CARGO_BIN_EXE_greek-meander"))
        .args([
            "--background-color",
            "#000000",
            "--no-png",
            "--file",
            &path,
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

    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let svg = std::fs::read_to_string(format!("{path}.svg")).unwrap();
    assert!(svg.contains("<rect"));
    assert!(svg.contains("fill=\"#000000\""));
}

#[cfg(feature = "native")]
#[test]
fn cli_stroke_dash_appears_in_svg() {
    let path = temp_path("gm_test_stroke_dash");
    let _guard = TempFiles::for_base(&path);

    let output = Command::new(env!("CARGO_BIN_EXE_greek-meander"))
        .args([
            "--stroke-dash",
            "5,3",
            "--no-png",
            "--file",
            &path,
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

    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let svg = std::fs::read_to_string(format!("{path}.svg")).unwrap();
    assert!(svg.contains("stroke-dasharray"));
}

#[cfg(feature = "native")]
#[test]
fn config_file_visual_options_apply() {
    let out = temp_path("gm_test_cfg_visual");
    let _guard = TempFiles::for_base(&out);
    let cfg = write_temp_config(
        "gm_test_cfg_visual",
        &format!(
            r##"
file = "{out}"
fill_color = "#AABBCC"
background_color = "#112233"
stroke_dash = "4,2"
[rect]
size = 10
width = 4
height = 4
"##
        ),
    );

    let output = Command::new(env!("CARGO_BIN_EXE_greek-meander"))
        .args(["--config", cfg.to_str().unwrap(), "--no-png", "rect"])
        .output()
        .unwrap();

    let _ = std::fs::remove_file(&cfg);
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let svg = std::fs::read_to_string(format!("{out}.svg")).unwrap();
    assert!(
        svg.contains("fill=\"#AABBCC\""),
        "fill_color from config should appear in svg"
    );
    assert!(
        svg.contains("<rect"),
        "background_color from config should add a rect element"
    );
    assert!(
        svg.contains("stroke-dasharray"),
        "stroke_dash from config should appear in svg"
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
    let radii = config.radii;
    assert!(radii.r_i < radii.r_a);
    assert!(radii.r_a < radii.r_b);
    assert!(radii.r_b < radii.r_c);
    assert!(radii.r_c < radii.r_d);
    assert!(radii.r_d < radii.r_e);
    assert!(radii.r_e < radii.r_o);
    assert_eq!(radii.r_o, 300.0);
}

#[test]
fn radii_type_is_nameable_at_crate_root() {
    let config = GreekKeyCircleConfig::new(300.0, 30, 10, 3.0).unwrap();
    let _r: greek_meander::Radii = config.radii;
    assert_eq!(_r.r_o, 300.0);
}
