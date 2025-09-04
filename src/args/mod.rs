use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(long, default_value_t = 6.0)]
    pub stroke_width: f32,
    #[arg(long, default_value_t = String::from("#AB8E0E"))]
    pub stroke_color: String,
    #[arg(long, default_value_t = 0.7)]
    pub stroke_opacity: f32,
    #[arg(long, default_value_t = 25)]
    pub size: i32,
    #[arg(long, default_value_t = 16)]
    pub width: i32,
    #[arg(long, default_value_t = 9)]
    pub height: i32,
    #[arg(long, default_value_t = String::from("meander"))]
    pub file: String,
}
