use cgmath::{Angle, Point3, Rad, Vector3};
use std::f64::consts::PI;

pub const RESOLUTION: usize = 64*16;

pub const WHITE: Color = Point3::new(1.0, 1.0, 1.0);
pub const BLACK: Color = Point3::new(0.0, 0.0, 0.0);

pub const RED: Color = Point3::new(1.0, 0.0, 0.0);
pub const GREEN: Color = Point3::new(0.0, 1.0, 0.0);
pub const BLUE: Color = Point3::new(0.0, 0.0, 1.0);

pub type Color = Point3<f64>;

pub trait CoolColor {
  fn to_string(self) -> String;
  fn get_r(self) -> u8;
  fn get_g(self) -> u8;
  fn get_b(self) -> u8;
}

impl CoolColor for Color {
  fn get_r(self) -> u8 {
    (self.x * 255.0) as u8
  }
  fn get_g(self) -> u8 {
    (self.y * 255.0) as u8
  }
  fn get_b(self) -> u8 {
    (self.z * 255.0) as u8
  }
  fn to_string(self) -> String {
    format!("{:0x}", (self.x * 15.0) as u8)
      + &format!("{:0x}", (self.y * 15.0) as u8).to_owned()
      + &format!("{:0x}", (self.z * 15.0) as u8).to_owned()
  }
}
pub struct Ray {
  pub pos: Point3<f64>,
  pub direction: Vector3<f64>,
  pub color: Color,
}

#[derive(Clone, Copy)]
pub struct Light {
  pub pos: Point3<f64>,
  pub color: Color,
  pub intensity: f64,
  pub reflet: bool
}

impl Light {
}

#[derive(Clone, Copy)]
pub struct LightIterator {
  light: Light,
  x: usize,
  y: usize,
}

impl IntoIterator for Light {
  type Item = Ray;
  type IntoIter = LightIterator;

  fn into_iter(self) -> Self::IntoIter {
    LightIterator {
      light: self,
      x: 0,
      y: 0,
    }
  }
}

fn sphere_to_cartesian(x: Rad<f64>, y: Rad<f64>) -> Vector3<f64> {
  Vector3::new(x.cos() * y.cos(), y.sin(), y.cos()*x.sin())
}

impl Iterator for LightIterator {
  type Item = Ray;
  fn next(&mut self) -> Option<Ray> {
    let ray = Ray {
      pos: self.light.pos,
      direction: sphere_to_cartesian(
        Rad((self.x as f64) * 2.0 * PI / (RESOLUTION as f64)),
        Rad((self.y as f64) * 2.0 * PI / (RESOLUTION as f64)),
      ),
      color: self.light.color,
    };
    self.x = (self.x + 1) % RESOLUTION;
    if self.x == 0 {
      self.y += 1;
      if !self.light.reflet {
      println!("{}/{}", self.y, RESOLUTION);
      }
    }
    if self.y == RESOLUTION {
      None
    } else {
      Some(ray)
    }
  }
}
