use greek_meander::{GreekKeyCircleConfig, circle};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = GreekKeyCircleConfig::new(300.0, 30, 10, 3.0)?;
    circle::generate_pattern_svg(&config, "#AB8E0E", 0.7, "meander_circle")?;
    println!("Generated meander_circle.svg and meander_circle.png");
    Ok(())
}
