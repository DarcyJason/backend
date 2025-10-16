use colorgrad::{Color, Gradient, GradientBuilder, LinearGradient};
use owo_colors::OwoColorize;

pub fn gradient_text(text: &str) -> Result<(), Box<dyn std::error::Error>> {
    let grad = GradientBuilder::new()
        .colors(&[
            Color::from_rgba8(255, 0, 128, 255),
            Color::from_rgba8(0, 128, 255, 255),
        ])
        .build::<LinearGradient>()?;
    let chars: Vec<char> = text.chars().collect();
    let n = chars.len().max(1);
    for (ch, color) in chars.iter().zip(grad.colors_iter(n)) {
        let [r, g, b, _a] = color.to_rgba8();
        print!("{}", ch.truecolor(r, g, b));
    }
    Ok(())
}
