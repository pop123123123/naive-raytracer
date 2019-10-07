use cgmath::prelude::ElementWise;
use cgmath::{dot, Decomposed, Ortho, Point3, Vector3};

use crate::light::{Color, Ray, WHITE};
use crate::plane::*;

pub struct Camera {
  //transformation: Decomposed,
  //projection: Ortho,
  planes: [Plane; 2],
  hole: Point3<f64>,
  radius: f64,
      pinhole_plane: Plane,
}

impl Camera {
  pub fn new() -> Self {
    Camera {
      //transformation: Decomposed::new(),
      //projection: Ortho::new(),
      hole: Point3::new(0.5, 0.5, 0.5),
      radius: 0.01,
      pinhole_plane: Plane::new(
          [
            Point3::new(1.0, 0.0, 0.5),
            Point3::new(0.0, 1.0, 0.5),
            Point3::new(0.0, 0.0, 0.5),
          ],
          WHITE,
        ),
      planes: [
        Plane::new(
          [
            Point3::new(1.0, 0.0, 0.0),
            Point3::new(0.0, 1.0, 0.0),
            Point3::new(0.0, 0.0, 0.0),
          ],
          WHITE,
        ),
        Plane::new(
          [
            Point3::new(1.0, 0.0, 0.0),
            Point3::new(0.0, 1.0, 0.0),
            Point3::new(1.0, 1.0, 0.0),
          ],
          WHITE,
        ),
      ],
    }
  }
  pub fn normal(&self) -> Vector3<f64> {
    self.planes[0].normal
  }
  pub fn intersect_pinhole(&self, ray: &Ray) -> bool {
    let col = dot(self.pinhole_plane.normal, ray.direction);
    if col != 0.0 {

    let d = dot(self.pinhole_plane.vertices[0] - ray.pos, self.pinhole_plane.normal) / col;
    let p = ray.pos + d * ray.direction;
        let v = p - self.hole; 
        let d2 = dot(v, v); 
        return d2.sqrt() <= self.radius; 
        // or you can use the following optimisation (and precompute radius^2)
        // return d2 <= radius2; // where radius2 = radius * radius
     } 
 
     return false; 
  }
  pub fn intersect(&self, ray: &Ray) -> Option<Point3<f64>> {
    self.planes[0]
      .intersect(ray)
      .or_else(|| self.planes[1].intersect(ray))
  }
}
