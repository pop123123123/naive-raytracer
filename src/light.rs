use cgmath::{Angle, ElementWise, InnerSpace, Point3, Rad, Vector3};

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
}

impl Light {
  pub fn get_intense_color_from(&self, p: Point3<f64>) -> Color {
    self.color * (self.intensity / (self.pos - p).magnitude().powf(2.0))
  }
}
