use greek_meander::{rect, GreekKeyRectConfig};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = GreekKeyRectConfig::new(25, 16, 9, 10, 3.0)?;
    rect::generate_pattern_svg(&config, "#AB8E0E", 0.7, "meander_rect")?;
    println!("Generated meander_rect.svg and meander_rect.png");
    Ok(())
}
