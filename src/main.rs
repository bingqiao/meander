use clap::Parser;

mod args;
mod cli_output;
use args::{Args, Commands};
use cli_output::OutputOptions;

use greek_meander::{
    circle,
    config::{GreekKeyCircleConfig, GreekKeyRectConfig},
    rect,
};

fn main() {
    let args = Args::parse();

    let output_options =
        match OutputOptions::new(!args.no_svg, !args.no_png, args.stdout, args.scale) {
            Ok(options) => options,
            Err(e) => {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
        };

    let result = match args.command {
        Commands::Rect(rect_args) => GreekKeyRectConfig::new(
            rect_args.size,
            rect_args.width,
            rect_args.height,
            args.border_margin,
            args.stroke_width,
        )
        .and_then(|config| {
            let svg = rect::generate_svg_string(&config, &args.stroke_color, args.stroke_opacity);
            cli_output::write_outputs(svg.as_bytes(), &args.file, &output_options)
        }),
        Commands::Circle(circle_args) => GreekKeyCircleConfig::new(
            circle_args.radius,
            circle_args.pattern_count,
            args.border_margin,
            args.stroke_width,
        )
        .and_then(|config| {
            let svg = circle::generate_svg_string(&config, &args.stroke_color, args.stroke_opacity);
            cli_output::write_outputs(svg.as_bytes(), &args.file, &output_options)
        }),
    };

    if let Err(e) = result {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
