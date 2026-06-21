use serde::{Deserialize, Serialize};

/// Serializable model for a TOML config file.
///
/// All fields are optional so CLI flags can override any subset.
/// Output routing flags (`--stdout`, `--no-svg`, `--no-png`) are intentionally
/// omitted: they are session-level choices that cannot be reliably overridden
/// from the CLI when set in a file, and do not belong in a reusable design config.
#[derive(Deserialize, Serialize, Default, Debug)]
pub struct FileConfig {
    pub stroke_width: Option<f32>,
    pub stroke_color: Option<String>,
    pub stroke_opacity: Option<f32>,
    pub fill_color: Option<String>,
    pub background_color: Option<String>,
    pub stroke_dash: Option<String>,
    pub border_margin: Option<i32>,
    pub file: Option<String>,
    pub scale: Option<f32>,
    pub rect: Option<RectFileConfig>,
    pub circle: Option<CircleFileConfig>,
}

#[derive(Deserialize, Serialize, Default, Debug)]
pub struct RectFileConfig {
    pub size: Option<i32>,
    pub width: Option<i32>,
    pub height: Option<i32>,
}

#[derive(Deserialize, Serialize, Default, Debug)]
pub struct CircleFileConfig {
    pub pattern_count: Option<i32>,
    pub radius: Option<f64>,
}

pub fn load(path: &std::path::Path) -> Result<FileConfig, Box<dyn std::error::Error>> {
    let content = std::fs::read_to_string(path)
        .map_err(|e| format!("could not read config file '{}': {}", path.display(), e))?;
    toml::from_str(&content)
        .map_err(|e| format!("invalid config file '{}': {}", path.display(), e).into())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse(s: &str) -> FileConfig {
        toml::from_str(s).unwrap()
    }

    #[test]
    fn empty_config_gives_all_none() {
        let cfg = parse("");
        assert!(cfg.stroke_width.is_none());
        assert!(cfg.rect.is_none());
        assert!(cfg.circle.is_none());
    }

    #[test]
    fn common_fields_parse() {
        let cfg = parse(
            r##"
            stroke_width = 4.0
            stroke_color = "#FF0000"
            stroke_opacity = 0.5
            border_margin = 10
            file = "out"
            scale = 2.0
            "##,
        );
        assert_eq!(cfg.stroke_width, Some(4.0));
        assert_eq!(cfg.stroke_color.as_deref(), Some("#FF0000"));
        assert_eq!(cfg.stroke_opacity, Some(0.5));
        assert_eq!(cfg.border_margin, Some(10));
        assert_eq!(cfg.file.as_deref(), Some("out"));
        assert_eq!(cfg.scale, Some(2.0));
    }

    #[test]
    fn rect_section_parses() {
        let cfg = parse(
            r#"
            [rect]
            size = 20
            width = 8
            height = 6
            "#,
        );
        let rect = cfg.rect.unwrap();
        assert_eq!(rect.size, Some(20));
        assert_eq!(rect.width, Some(8));
        assert_eq!(rect.height, Some(6));
    }

    #[test]
    fn circle_section_parses() {
        let cfg = parse(
            r#"
            [circle]
            pattern_count = 20
            radius = 150.0
            "#,
        );
        let circle = cfg.circle.unwrap();
        assert_eq!(circle.pattern_count, Some(20));
        assert_eq!(circle.radius, Some(150.0));
    }

    #[test]
    fn output_routing_flags_are_ignored() {
        // stdout/no_svg/no_png are not config file fields; they must be silently
        // ignored so configs written for future versions don't break old binaries.
        for key in ["stdout", "no_svg", "no_png"] {
            let cfg: FileConfig =
                toml::from_str(&format!("{key} = true\nstroke_width = 4.0")).unwrap();
            assert_eq!(
                cfg.stroke_width,
                Some(4.0),
                "'{key}' should be ignored without dropping supported fields"
            );
        }
    }

    #[test]
    fn unknown_field_is_ignored() {
        // Unknown fields are silently ignored for forward compatibility.
        let cfg: FileConfig = toml::from_str("unknown_key = 1\nscale = 2.0").unwrap();
        assert_eq!(cfg.scale, Some(2.0));
    }
}
