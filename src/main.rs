use clap::Parser;

mod args;
mod cli_output;
mod file_config;
use args::{Args, Commands};
use cli_output::OutputOptions;

use greek_meander::{
    circle,
    config::{GreekKeyCircleConfig, GreekKeyRectConfig, VisualOptions},
    rect,
};

const DEFAULT_STROKE_WIDTH: f32 = 6.0;
const DEFAULT_STROKE_COLOR: &str = "#AB8E0E";
const DEFAULT_STROKE_OPACITY: f32 = 0.7;
const DEFAULT_BORDER_MARGIN: i32 = 1;
const DEFAULT_FILE: &str = "meander";
const DEFAULT_SCALE: f32 = 1.0;
const DEFAULT_RECT_SIZE: i32 = 25;
const DEFAULT_RECT_WIDTH: i32 = 16;
const DEFAULT_RECT_HEIGHT: i32 = 9;
const DEFAULT_CIRCLE_PATTERN_COUNT: i32 = 30;
const DEFAULT_CIRCLE_RADIUS: f64 = 300.0;

fn main() {
    let args = Args::parse();

    let file_cfg = match &args.config {
        Some(path) => match file_config::load(path) {
            Ok(cfg) => cfg,
            Err(e) => {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
        },
        None => file_config::FileConfig::default(),
    };

    // CLI flags win; file config fills gaps; hardcoded defaults are the last resort.
    let stroke_width = args
        .stroke_width
        .or(file_cfg.stroke_width)
        .unwrap_or(DEFAULT_STROKE_WIDTH);
    let stroke_color = args
        .stroke_color
        .or(file_cfg.stroke_color)
        .unwrap_or_else(|| DEFAULT_STROKE_COLOR.to_string());
    let stroke_opacity = args
        .stroke_opacity
        .or(file_cfg.stroke_opacity)
        .unwrap_or(DEFAULT_STROKE_OPACITY);
    let border_margin = args
        .border_margin
        .or(file_cfg.border_margin)
        .unwrap_or(DEFAULT_BORDER_MARGIN);
    let file = args
        .file
        .or(file_cfg.file)
        .unwrap_or_else(|| DEFAULT_FILE.to_string());
    let scale = args.scale.or(file_cfg.scale).unwrap_or(DEFAULT_SCALE);

    let mut visual = VisualOptions::new(stroke_color, stroke_opacity);
    visual.fill_color = args.fill_color.or(file_cfg.fill_color);
    visual.background_color = args.background_color.or(file_cfg.background_color);
    visual.stroke_dash = args.stroke_dash.or(file_cfg.stroke_dash);

    let output_options = match OutputOptions::new(!args.no_svg, !args.no_png, args.stdout, scale) {
        Ok(options) => options,
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    };

    let rect_cfg = file_cfg.rect.unwrap_or_default();
    let circle_cfg = file_cfg.circle.unwrap_or_default();

    let result = match args.command {
        Commands::Rect(rect_args) => {
            let size = rect_args
                .size
                .or(rect_cfg.size)
                .unwrap_or(DEFAULT_RECT_SIZE);
            let width = rect_args
                .width
                .or(rect_cfg.width)
                .unwrap_or(DEFAULT_RECT_WIDTH);
            let height = rect_args
                .height
                .or(rect_cfg.height)
                .unwrap_or(DEFAULT_RECT_HEIGHT);
            GreekKeyRectConfig::new(size, width, height, border_margin, stroke_width).and_then(
                |config| {
                    let svg = rect::generate_svg_string(&config, &visual);
                    cli_output::write_outputs(svg.as_bytes(), &file, &output_options)
                },
            )
        }
        Commands::Circle(circle_args) => {
            let radius = circle_args
                .radius
                .or(circle_cfg.radius)
                .unwrap_or(DEFAULT_CIRCLE_RADIUS);
            let pattern_count = circle_args
                .pattern_count
                .or(circle_cfg.pattern_count)
                .unwrap_or(DEFAULT_CIRCLE_PATTERN_COUNT);
            GreekKeyCircleConfig::new(radius, pattern_count, border_margin, stroke_width).and_then(
                |config| {
                    let svg = circle::generate_svg_string(&config, &visual);
                    cli_output::write_outputs(svg.as_bytes(), &file, &output_options)
                },
            )
        }
    };

    if let Err(e) = result {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
