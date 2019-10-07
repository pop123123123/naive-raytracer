use cgmath::{EuclideanSpace, Point2, Vector2};
use itertools::Itertools;

use crate::light::{Color, CoolColor};

pub const WIDTH: usize = 128*4;
pub const HEIGHT: usize = 128*4;
pub const ITER_SQUARE: [Vector2<f64>; 4] = [
  Vector2::new(-0.5, -0.5),
  Vector2::new(0.5, -0.5),
  Vector2::new(-0.5, 0.5),
  Vector2::new(0.5, 0.5),
];

pub type Screen = [Color; WIDTH * HEIGHT];

pub trait CoolScreen {
  fn init(color: Color) -> Self;
  fn get(&self, x: usize, y: usize) -> Color;
  fn get_point(&self, point: Point2<f64>) -> Color;
  fn set(&mut self, x: usize, y: usize, color: Color);
  fn add(&mut self, x: usize, y: usize, color: Color);
  fn add_point(&mut self, point: Point2<f64>, color: Color);
  fn normalize(&mut self);
  fn to_string(self) -> String;
}

fn proportion_of_surface(x: usize, y: usize, x0: f64, y0: f64, size: f64) -> f64 {
  let half_size = size / 2.0;
  let dx = (x as f64 + 0.5).min(x0 + half_size) - (x as f64 - 0.5).max(x0 - half_size);
  let dy = (y as f64 + 0.5).min(y0 + half_size) - (y as f64 - 0.5).max(y0 - half_size);
  if dx > 0.0 && dy > 0.0 {
    dx * dy / (size * size)
  } else {
    0.0
  }
}

impl CoolScreen for Screen {
  fn init<'a>(color: Color) -> Self {
    [color; WIDTH * HEIGHT]
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
  fn add_point(&mut self, point: Point2<f64>, color: Color) {
    let x0 = point.x * WIDTH as f64;
    let y0 = point.y * HEIGHT as f64;
    ITER_SQUARE.iter().for_each(|modif| {
      let x = (modif.x + x0) as usize;
      let y = (modif.y + y0) as usize;
      if x < WIDTH &&  y < HEIGHT {
        self.add(x, y, color*proportion_of_surface(x, y, x0, y0, 1.0));
      }
    });
  }
  fn add(&mut self, x: usize, y: usize, color: Color) {
    self[x + WIDTH * y] += color.to_vec();
  }
  fn normalize(&mut self) {
    let max : f64 = self.iter().fold(0.0, |max, col| f64::max(f64::max(col.x,col.y),col.z).max(max));
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
