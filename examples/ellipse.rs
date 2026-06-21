use greek_meander::{GreekKeyEllipseConfig, VisualOptions, ellipse};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = GreekKeyEllipseConfig::new(200.0, 120.0, 28, 8, 3.0)?;
    let mut visual = VisualOptions::new("#7C3B2E", 0.85);
    visual.fill_color = Some("#F2DED1".to_string());
    visual.background_color = Some("#1E251D".to_string());

    ellipse::generate_pattern_svg(&config, &visual, "meander_ellipse")?;
    println!("Generated meander_ellipse.svg and meander_ellipse.png");
    Ok(())
}
