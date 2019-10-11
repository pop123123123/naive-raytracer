use cgmath::{EuclideanSpace, Point2, Vector2};
use itertools::Itertools;

use crate::light::{Color, CoolColor};

pub const WIDTH: usize = 128 * 4;
pub const HEIGHT: usize = 128 * 4;
pub const ITER_SQUARE: [Vector2<f64>; 4] = [
  Vector2::new(-0.5, -0.5),
  Vector2::new(0.5, -0.5),
  Vector2::new(-0.5, 0.5),
  Vector2::new(0.5, 0.5),
];

pub type Screen = [Color; WIDTH * HEIGHT];

pub trait CoolScreen {
  fn init(color: Color) -> Box<Self>;
  fn get(&self, x: usize, y: usize) -> Color;
  fn get_point(&self, point: Point2<f64>) -> Color;
  fn set(&mut self, x: usize, y: usize, color: Color);
  fn add(&mut self, x: usize, y: usize, color: Color);
  fn normalize(&mut self);
  fn to_string(self) -> String;
}

impl CoolScreen for Screen {
  fn init(color: Color) -> Box<Self> {
    Box::new([color; WIDTH * HEIGHT])
  }
  fn get_point(&self, point: Point2<f64>) -> Color {
    let x = (point.x * WIDTH as f64) as usize;
    let y = (point.x * HEIGHT as f64) as usize;
    self.get(x, y)
  }
  fn get(&self, x: usize, y: usize) -> Color {
    self[x + WIDTH * y]
  }
  fn set(&mut self, x: usize, y: usize, color: Color) {
    self[x + WIDTH * y] = color;
  }
  fn add(&mut self, x: usize, y: usize, color: Color) {
    self[x + WIDTH * y] += color.to_vec();
  }
  fn normalize(&mut self) {
    let max: f64 = self.iter().fold(0.0, |max, col| {
      f64::max(f64::max(col.x, col.y), col.z).max(max)
    });
    self.iter_mut().for_each(|mut col| {
      col.x /= max;
      col.y /= max;
      col.z /= max;
    });
  }
  fn to_string(self) -> String {
    self
      .iter()
      .chunks(WIDTH)
      .into_iter()
      .map(|row| row.map(|i| i.to_string() + " ").collect::<String>())
      .collect::<Vec<String>>()
      .join("\n")
  }
}
