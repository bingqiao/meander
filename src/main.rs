use clap::Parser;

mod args;
use args::Args;

mod config;
use config::GreekKeyConfig;

mod drawing;
use drawing::generate_pattern_svg;

fn main() {
    let args = Args::parse();

    let config = GreekKeyConfig::new(args.size, args.width, args.height);

    generate_pattern_svg(
        &config,
        args.stroke_width,
        &args.stroke_color,
        args.stroke_opacity,
        &args.file,
    );
}