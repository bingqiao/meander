use clap::Parser;

mod args;
use args::{Args, Commands};

mod config;
use config::{GreekKeyCircleConfig, GreekKeyRectConfig};

mod rect;

mod circle;

fn main() {
    let args = Args::parse();

    match args.command {
        Commands::Rect(rect_args) => {
            let config: GreekKeyRectConfig = GreekKeyRectConfig::new(
                rect_args.size,
                rect_args.width,
                rect_args.height,
                args.border_margin,
                args.stroke_width,
            );

            rect::generate_pattern_svg(
                &config,
                args.stroke_width,
                &args.stroke_color,
                args.stroke_opacity,
                &args.file,
            );
        }
        Commands::Circle(circle_args) => {
            let config: GreekKeyCircleConfig = GreekKeyCircleConfig::new(
                circle_args.radius,
                circle_args.pattern_count,
                args.border_margin,
                args.stroke_width,
            );

            circle::generate_pattern_svg(
                &config,
                args.stroke_width,
                &args.stroke_color,
                args.stroke_opacity,
                &args.file,
            );
        }
    }
}
