use greek_meander::{GreekKeyCircleConfig, VisualOptions, circle};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = GreekKeyCircleConfig::new(155.0, 28, 8, 3.0)?;
    let mut visual = VisualOptions::new("#7C3B2E", 0.85);
    visual.fill_color = Some("#F2DED1".to_string());
    visual.background_color = Some("#1E251D".to_string());
    visual.stroke_dash = Some("7,3".to_string());

    circle::generate_pattern_svg(&config, &visual, "meander_circle")?;
    println!("Generated meander_circle.svg and meander_circle.png");
    Ok(())
}
