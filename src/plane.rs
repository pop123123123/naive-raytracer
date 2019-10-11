use cgmath::{dot, Vector3, Point3, Array, EuclideanSpace, InnerSpace};
use cgmath::prelude::ElementWise;
use rand::Rng;

use crate::light::{Color, Ray, Light};

pub type Triangle = [Point3<f64>; 3];

pub struct Plane {
  pub id: f64,
  pub vertices: Triangle,
  pub color: Color,
  pub normal: Vector3<f64>,
}

fn is_point_in_triangle(vertices: Triangle, point: &Point3<f64>) -> bool {
  // Compute vectors
  let v0 = vertices[2] - vertices[0];
  let v1 = vertices[1] - vertices[0];
  let v2 = *point - vertices[0];

  // Compute dot products
  let dot00 = dot(v0, v0);
  let dot01 = dot(v0, v1);
  let dot02 = dot(v0, v2);
  let dot11 = dot(v1, v1);
  let dot12 = dot(v1, v2);

  // Compute barycentric coordinates
  let inv_denom = 1.0 / (dot00 * dot11 - dot01 * dot01);
  let u = (dot11 * dot02 - dot01 * dot12) * inv_denom;
  let v = (dot00 * dot12 - dot01 * dot02) * inv_denom;

  // Check if point is in triangle
  (u >= 0.0) && (v >= 0.0) && (u + v < 1.0)
}

fn normal(vertices: &Triangle) -> Vector3<f64> {
  let v0 = vertices[2] - vertices[0];
  let v1 = vertices[1] - vertices[0];

  v0.cross(v1).normalize()
}

impl Plane {
  pub fn new(vertices: Triangle, color: Color) -> Self {
    Plane {
      id: vertices.iter().fold(0.0, |sum, p| p.sum() + sum as f64) + color.sum(),
      vertices: vertices,
      color: color,
      normal: normal(&vertices),
    }
  }
  pub fn intersect(&self, ray: &Ray) -> Option<Point3<f64>> {
    let n = self.normal;
    let col = dot(n, ray.direction);
    if col == 0.0 {
      None
    } else {
      let d = dot(self.vertices[0] - ray.pos, n) / col;
      if d < 0.0 {
        return None;
      }
      let point = ray.pos + d * ray.direction;
      if is_point_in_triangle(self.vertices, &point) {
        Some(point)
      } else {
        None
      }
    }
  }
  pub fn random_point(&self) -> Point3<f64> {
    let mut rng = rand::thread_rng();
    let (r0, r1) : (f64, f64) = (rng.gen(), rng.gen());
    ((1.0 - r0.sqrt()) * self.vertices[0])
    .add_element_wise(r0.sqrt() * (1.0 - r1) * self.vertices[1])
    .add_element_wise((r0.sqrt() * r1) * self.vertices[2])
  }
  pub fn are_on_same_side(&self, p0: Point3<f64>, p1: Point3<f64>) -> bool {
    dot(self.normal, self.vertices[0]-p0).signum() == dot(self.normal, self.vertices[0]-p1).signum()
  }
  pub fn light_from_ray(&self, ray: &Ray) -> Light {
    Light {
      pos: self.intersect(ray).unwrap(),
      color: ray.color.mul_element_wise(self.color),
      intensity: 1.0,
      reflet: true
    }
  }
  pub fn reflect(&self, ray: &Ray) -> Ray {
    let pos = self.intersect(ray).unwrap();
    let direction = ray.direction - 2.0 * dot(self.normal, ray.direction) * self.normal;
    Ray {
      pos : pos,
      direction : if self.are_on_same_side(ray.pos, pos + direction) {direction} else {-direction},
      color : ray.color.mul_element_wise(self.color),
    }
  }
}

impl PartialEq<Plane> for Plane
{
    fn eq(&self, other: &Plane) -> bool {
        self.id.eq(&other.id)
    }
}
