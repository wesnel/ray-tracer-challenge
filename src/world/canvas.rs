use std::{
    fmt::{self, Display, Formatter},
    ops::{Index, IndexMut},
    vec::Vec,
};

use super::color::{Color, MAX_COLOR};

#[derive(Debug)]
pub struct Canvas {
    pub width: usize,
    pub height: usize,
    vals: Vec<Color>,
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Canvas {
        Canvas::from_fn(width, height, |_, _| Color::new(0.0, 0.0, 0.0))
    }

    pub fn from_fn<F: FnMut(usize, usize) -> Color>(
        width: usize,
        height: usize,
        mut f: F,
    ) -> Canvas {
        Canvas {
            width,
            height,
            vals: (0..(height * width))
                .map(|i| f(i % width, i / width))
                .collect(),
        }
    }

    pub fn to_ppm(&self) -> String {
        format!(
            "P3\n{} {}\n{}\n{}",
            self.width, self.height, MAX_COLOR as i64, self
        )
    }
}

impl Index<(usize, usize)> for Canvas {
    type Output = Color;

    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        self.vals.get(x + y * self.width).unwrap()
    }
}

impl IndexMut<(usize, usize)> for Canvas {
    fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut Color {
        self.vals.get_mut(x + y * self.width).unwrap()
    }
}

impl Display for Canvas {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut result = Ok(());

        for elem in &self.vals {
            if let Err(e) = writeln!(f, "{}", elem) {
                result = Err(e);
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_canvas() {
        let c = Canvas::new(10, 20);
        assert_eq!(c.width, 10);
        assert_eq!(c.height, 20);

        for y in 0..20 {
            for x in 0..10 {
                assert_eq!(c[(x, y)], Color::black());
            }
        }
    }

    #[test]
    fn write_pixel() {
        let mut c = Canvas::new(10, 20);
        let red = Color::new(1.0, 0.0, 0.0);
        c[(2, 3)] = red;
        assert_eq!(c[(2, 3)], red);
    }

    #[test]
    fn ppm_header() {
        let c = Canvas::new(5, 3);
        let ppm = c.to_ppm();
        let lines: Vec<&str> = ppm.split("\n").collect();
        assert_eq!(lines[..3], ["P3", "5 3", "255"]);
    }

    #[test]
    fn ppm_pixel_data() {
        let mut c = Canvas::new(5, 3);
        let c1 = Color::new(1.5, 0.0, 0.0);
        let c2 = Color::new(0.0, 0.5, 0.0);
        let c3 = Color::new(-0.5, 0.0, 1.0);

        c[(0, 0)] = c1;
        c[(2, 1)] = c2;
        c[(4, 2)] = c3;

        let ppm = c.to_ppm();
        let lines: Vec<&str> = ppm.split("\n").collect();
        assert_eq!(
            lines[3..18],
            [
                "255 0 0", "0 0 0", "0 0 0", "0 0 0", "0 0 0", "0 0 0", "0 0 0", "0 128 0",
                "0 0 0", "0 0 0", "0 0 0", "0 0 0", "0 0 0", "0 0 0", "0 0 255",
            ]
        );
    }

    #[test]
    fn ppm_dense_colors() {
        let c = Canvas::from_fn(10, 2, |_, _| Color::new(1.0, 0.8, 0.6));
        let ppm = c.to_ppm();
        let lines: Vec<&str> = ppm.split("\n").collect();

        assert_eq!(
            lines[3..23],
            [
                "255 204 153",
                "255 204 153",
                "255 204 153",
                "255 204 153",
                "255 204 153",
                "255 204 153",
                "255 204 153",
                "255 204 153",
                "255 204 153",
                "255 204 153",
                "255 204 153",
                "255 204 153",
                "255 204 153",
                "255 204 153",
                "255 204 153",
                "255 204 153",
                "255 204 153",
                "255 204 153",
                "255 204 153",
                "255 204 153",
            ]
        );
    }

    #[test]
    fn ppm_ends_with_newline() {
        let c = Canvas::new(5, 3);
        let ppm = c.to_ppm();
        let mut chars = ppm.chars();
        assert_eq!(chars.next_back().unwrap(), '\n');
    }
}
