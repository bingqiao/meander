use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(long)]
    pub config: Option<PathBuf>,
    #[arg(long, help = "Stroke width [default: 6]")]
    pub stroke_width: Option<f32>,
    #[arg(long, help = "Stroke color [default: #AB8E0E]")]
    pub stroke_color: Option<String>,
    #[arg(long, help = "Stroke opacity [default: 0.7]")]
    pub stroke_opacity: Option<f32>,
    #[arg(long, help = "Fill color for pattern interior [default: none]")]
    pub fill_color: Option<String>,
    #[arg(long, help = "Background color for the SVG canvas [default: none]")]
    pub background_color: Option<String>,
    #[arg(
        long,
        help = "SVG stroke-dasharray value, e.g. \"5,3\" [default: solid]"
    )]
    pub stroke_dash: Option<String>,
    #[arg(long, help = "Border margin in pixels [default: 1]")]
    pub border_margin: Option<i32>,
    #[arg(long, help = "Base name of the output file [default: meander]")]
    pub file: Option<String>,
    #[arg(long)]
    pub stdout: bool,
    #[arg(long)]
    pub no_svg: bool,
    #[arg(long)]
    pub no_png: bool,
    #[arg(long, help = "PNG scale factor [default: 1.0]")]
    pub scale: Option<f32>,
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Rect(RectArgs),
    Circle(CircleArgs),
    Ellipse(EllipseArgs),
}

#[derive(Parser, Debug)]
pub struct RectArgs {
    #[arg(long, help = "Key unit length in pixels [default: 25]")]
    pub size: Option<i32>,
    #[arg(long, help = "Width in pattern units [default: 16]")]
    pub width: Option<i32>,
    #[arg(long, help = "Height in pattern units [default: 9]")]
    pub height: Option<i32>,
}

#[derive(Parser, Debug)]
pub struct CircleArgs {
    #[arg(long, help = "Number of patterns around the circle [default: 30]")]
    pub pattern_count: Option<i32>,
    #[arg(long, help = "Outer radius in pixels [default: 300]")]
    pub radius: Option<f64>,
}

#[derive(Parser, Debug)]
pub struct EllipseArgs {
    #[arg(long, help = "Number of patterns around the ellipse [default: 30]")]
    pub pattern_count: Option<i32>,
    #[arg(long, help = "Horizontal outer semi-axis in pixels [default: 300]")]
    pub rx: Option<f64>,
    #[arg(long, help = "Vertical outer semi-axis in pixels [default: 200]")]
    pub ry: Option<f64>,
}
