use std::fmt::Write as FmtWrite;
use std::fs::File;
use std::io::Write as IoWrite;
use std::path::Path;

use crate::prelude::Color;

/// A 2D canvas storing colors for ray tracing.
#[derive(Default, Debug, Clone)]
pub struct Canvas {
    width:  usize,
    height: usize,
    pixels: Vec<Color>,
}

impl Canvas {
    /// Creates a new canvas filled with default colors (black).
    pub fn new(width: usize, height: usize) -> Self {
        #[rustfmt::skip]
        Self { width, height, pixels: vec![Color::default(); width * height] }
    }

    /// Returns the canvas width.
    pub const fn width(&self) -> usize { self.width }

    /// Returns the canvas height.
    pub const fn height(&self) -> usize { self.height }

    pub const fn index(&self, x: usize, y: usize) -> usize { y * self.width + x }
}

impl Canvas {
    pub fn pixel_at(&self, x: usize, y: usize) -> Color { self.pixels[self.index(x, y)] }

    pub fn write_pixel(mut self, x: usize, y: usize, color: Color) -> Self {
        let idx = self.index(x, y);
        self.pixels[idx] = color;
        self
    }

    pub fn to_ppm(&self) -> String {
        let mut ppm = self.ppm_header();
        ppm.push_str(&self.ppm_content());
        ppm
    }

    // fn scale_color(value: f64) -> u8 { (value * 256.0).clamp(0.0, 255.0) as u8 }

    /// Generates the PPM header.
    fn ppm_header(&self) -> String { format!("P3\n{} {}\n255\n", self.width, self.height) }

    fn ppm_content(&self) -> String {
        let mut output = String::with_capacity(self.width * self.height * 12);

        for pixel_row in self.pixels.chunks_exact(self.width) {
            let mut current_line_len = 0;

            for rgb_values in pixel_row.iter().map(|&color| <[u8; 3]>::from(color)) {
                for component_str in rgb_values.into_iter().map(|component| component.to_string()) {
                    let separator = if current_line_len == 0 { "" } else { " " };

                    // Enforce 70-char line limit (PPM spec requirement)
                    if current_line_len + separator.len() + component_str.len() > 70 {
                        output.push('\n');
                        current_line_len = 0;
                    }

                    write!(output, "{}{}", separator, component_str).unwrap();
                    current_line_len += separator.len() + component_str.len();
                }
            }

            // After each row, finish the line
            output.push('\n');
        }

        output
    }

    pub fn write_to_file(&self, path: impl AsRef<Path>) -> std::io::Result<()> {
        let content = self.to_ppm();

        let mut file = File::create(path)?;
        file.write_all(content.as_bytes())
    }
}
