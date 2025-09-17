use core::ops::{Index, IndexMut};

use crate::primitives::Color3;

/// A 2D canvas storing colors for ray tracing.
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Canvas {
    width:  usize,
    height: usize,
    pixels: Vec<Color3>,
}

impl Canvas {
    /// Creates a new canvas filled with default colors (black).
    pub fn new(width: usize, height: usize) -> Self {
        #[rustfmt::skip]
        Self { width, height, pixels: vec![Color3::black(); width * height] }
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
    pub fn write_pixel(&mut self, x: usize, y: usize, color: Color3) { self[y][x] = color; }

    pub fn to_ppm(&self) -> String {
        let mut ppm = self.ppm_header();
        ppm.push_str(&self.ppm_content());
        ppm
    }

    // fn scale_color(value: f64) -> u8 { (value * 256.0).clamp(0.0, 255.0) as u8 }

    /// Generates the PPM header.
    fn ppm_header(&self) -> String { format!("P3\n{} {}\n255\n", self.width, self.height) }

    fn ppm_content(&self) -> String {
        use ::std::fmt::Write as _;

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

    pub fn export(&self, path: impl AsRef<::std::path::Path>) -> ::std::io::Result<()> {
        use ::std::io::Write as _;
        let content = self.to_ppm();

        let mut file = ::std::fs::File::create(path)?;
        file.write_all(content.as_bytes())
    }
}

impl Index<usize> for Canvas {
    type Output = [Color3];

    fn index(&self, index: usize) -> &Self::Output {
        let start = self.width * index;
        &self.pixels[start..start + self.width]
    }
}

impl IndexMut<usize> for Canvas {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        let start = self.width * index;
        &mut self.pixels[start..start + self.width]
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::prelude::color;

    #[test]
    fn test_canvas_constructor_sets_height_and_width() {
        let canvas1 = Canvas::new(10, 20);
        assert_eq!(canvas1.width, 10);
        assert_eq!(canvas1.height, 20);
    }

    #[test]
    fn test_canvas_constructor_sets_all_pixels_to_black() {
        let canvas1 = Canvas::new(4, 6);

        for x in 0..3 {
            for y in 0..5 {
                assert_eq!(canvas1[y][x], color(0.0, 0.0, 0.0));
            }
        }
    }

    #[test]
    fn test_pixels_can_be_written_to_a_canvas() {
        let mut canvas1 = Canvas::new(10, 20);
        let color1 = color(1.0, 0.0, 0.0);

        canvas1.write_pixel(2, 3, color1);

        let actual = canvas1[3][2];
        assert_eq!(actual, color(1.0, 0.0, 0.0),);
    }
}
