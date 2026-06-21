use greek_meander::{GreekKeyRectConfig, VisualOptions, rect};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = GreekKeyRectConfig::new(14, 16, 9, 8, 3.0)?;
    let mut visual = VisualOptions::new("#1F5B73", 0.9);
    visual.fill_color = Some("#DCEFF4".to_string());
    visual.background_color = Some("#182026".to_string());
    visual.stroke_dash = Some("10,5".to_string());

    rect::generate_pattern_svg(&config, &visual, "meander_rect")?;
    println!("Generated meander_rect.svg and meander_rect.png");
    Ok(())
}
