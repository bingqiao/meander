use clap::Parser;

mod args;
use args::{Args, Commands};

use greek_meander::{
    circle,
    config::{GreekKeyCircleConfig, GreekKeyRectConfig},
    rect,
};

fn main() {
    let args = Args::parse();

    let result = match args.command {
        Commands::Rect(rect_args) => GreekKeyRectConfig::new(
            rect_args.size,
            rect_args.width,
            rect_args.height,
            args.border_margin,
            args.stroke_width,
        )
        .and_then(|config| {
            rect::generate_pattern_svg(&config, &args.stroke_color, args.stroke_opacity, &args.file)
        }),
        Commands::Circle(circle_args) => GreekKeyCircleConfig::new(
            circle_args.radius,
            circle_args.pattern_count,
            args.border_margin,
            args.stroke_width,
        )
        .and_then(|config| {
            circle::generate_pattern_svg(
                &config,
                &args.stroke_color,
                args.stroke_opacity,
                &args.file,
            )
        }),
    };

    if let Err(e) = result {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
