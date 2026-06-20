use std::error::Error;

use resvg::render;
use resvg::usvg::Tree;

#[derive(Debug, Clone, Copy)]
pub(crate) struct OutputOptions {
    pub(crate) write_svg: bool,
    pub(crate) write_png: bool,
    pub(crate) write_stdout: bool,
    pub(crate) png_scale: f32,
}

impl OutputOptions {
    pub(crate) fn new(
        write_svg: bool,
        write_png: bool,
        write_stdout: bool,
        png_scale: f32,
    ) -> Result<Self, Box<dyn Error>> {
        let options = Self {
            write_svg,
            write_png,
            write_stdout,
            png_scale,
        };
        options.validate()?;
        Ok(options)
    }

    fn validate(&self) -> Result<(), Box<dyn Error>> {
        if !self.write_svg && !self.write_png && !self.write_stdout {
            return Err(
                "at least one output is required; remove --no-svg or --no-png, or add --stdout"
                    .into(),
            );
        }
        if self.png_scale <= 0.0 || !self.png_scale.is_finite() {
            return Err("--scale must be a positive finite number".into());
        }
        Ok(())
    }
}

pub(crate) fn write_outputs(
    svg_content: &[u8],
    filename: &str,
    options: &OutputOptions,
) -> Result<(), Box<dyn Error>> {
    options.validate()?;

    if options.write_stdout {
        use std::io::Write;
        let stdout = std::io::stdout();
        let mut handle = stdout.lock();
        handle.write_all(svg_content)?;
        handle.write_all(b"\n")?;
    }

    if options.write_svg {
        std::fs::write(format!("{}.svg", filename), svg_content)?;
    }

    if options.write_png {
        write_png(svg_content, filename, options.png_scale)?;
    }

    Ok(())
}

fn write_png(svg_content: &[u8], filename: &str, scale: f32) -> Result<(), Box<dyn Error>> {
    let tree = Tree::from_data(svg_content, &resvg::usvg::Options::default())?;
    let pixmap_size = tree
        .size()
        .to_int_size()
        .scale_by(scale)
        .ok_or("scaled canvas has zero or invalid dimensions")?;
    let mut pixmap = resvg::tiny_skia::Pixmap::new(pixmap_size.width(), pixmap_size.height())
        .ok_or("canvas has zero dimensions")?;
    render(
        &tree,
        resvg::tiny_skia::Transform::from_scale(scale, scale),
        &mut pixmap.as_mut(),
    );
    pixmap.save_png(format!("{}.png", filename))?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::OutputOptions;

    #[test]
    fn stdout_only_output_is_valid() {
        let options = OutputOptions::new(false, false, true, 1.0).unwrap();
        assert!(options.write_stdout);
        assert!(!options.write_svg);
        assert!(!options.write_png);
    }

    #[test]
    fn no_output_is_invalid() {
        let err = OutputOptions::new(false, false, false, 1.0).unwrap_err();
        assert!(err.to_string().contains("at least one output is required"));
    }

    #[test]
    fn png_scale_must_be_positive_and_finite() {
        for scale in [0.0, -1.0, f32::NAN, f32::INFINITY] {
            let err = OutputOptions::new(true, true, false, scale).unwrap_err();
            assert!(err.to_string().contains("--scale"));
        }
    }
}
