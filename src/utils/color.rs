use colorgrad::{Color, Gradient, GradientBuilder, LinearGradient};
use owo_colors::OwoColorize;

use crate::custom::{errors::external::ExternalError, result::AppResult};

pub fn gradient_text(text: &str) -> AppResult<()> {
    let grad = GradientBuilder::new()
        .colors(&[
            Color::from_rgba8(255, 0, 128, 255),
            Color::from_rgba8(0, 128, 255, 255),
        ])
        .build::<LinearGradient>()
        .map_err(ExternalError::from)?;
    let chars: Vec<char> = text.chars().collect();
    let n = chars.len().max(1);
    for (ch, color) in chars.iter().zip(grad.colors_iter(n)) {
        let [r, g, b, _a] = color.to_rgba8();
        print!("{}", ch.truecolor(r, g, b));
    }
    Ok(())
}
