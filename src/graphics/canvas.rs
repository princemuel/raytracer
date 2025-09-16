use std::fmt::Write as FmtWrite;
use std::fs::File;
use std::io;
use std::io::Write as IoWrite;
use std::path::Path;

use crate::math::primitives::Color3;

/// Creates a color
#[inline]
pub fn canvas<T>(width: T, height: T) -> Canvas
where
    T: core::convert::Into<usize>,
{
    Canvas::new(width.into(), height.into())
}

/// A 2D canvas storing colors for ray tracing.
#[derive(Default, Debug, Clone)]
pub struct Canvas {
    width:  usize,
    height: usize,
    pixels: Vec<Color3>,
}

impl Canvas {
    /// Creates a new canvas filled with default colors (black).
    pub fn new(width: usize, height: usize) -> Self {
        #[rustfmt::skip]
        Self { width, height, pixels: vec![Color3::default(); width * height] }
    }

    /// Returns the canvas width.
    pub const fn width(&self) -> usize { self.width }

    /// Returns the canvas height.
    pub const fn height(&self) -> usize { self.height }

    /// Returns the canvas pixels.
    pub fn pixels(&self) -> &[Color3] { &self.pixels }

    /// Returns a mutable reference to the canvas' pixels.
    pub fn pixels_mut(&mut self) -> &mut [Color3] { &mut self.pixels }
}

impl Canvas {
    #[inline]
    pub const fn pixel_index(&self, x: usize, y: usize) -> usize { y * self.width + x }

    pub fn pixel_at(&self, x: usize, y: usize) -> Color3 { self.pixels[self.pixel_index(x, y)] }

    pub fn pixel_at_mut(&mut self, x: usize, y: usize) -> &mut Color3 {
        let idx = self.pixel_index(x, y);
        &mut self.pixels[idx]
    }

    pub fn write_pixel(&mut self, x: usize, y: usize, color: Color3) { *self.pixel_at_mut(x, y) = color; }

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

        for pixel_row in self.pixels().chunks_exact(self.width) {
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

    pub fn write_to_file(&self, path: impl AsRef<Path>) -> io::Result<()> {
        let content = self.to_ppm();

        let mut file = File::create(path)?;
        file.write_all(content.as_bytes())
    }
}
