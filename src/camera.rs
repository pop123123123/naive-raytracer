use cgmath::prelude::ElementWise;
use cgmath::{dot, Decomposed, Ortho, Point3, Vector2, Vector3};

use crate::light::{Color, Ray, WHITE};
use crate::plane::*;
use crate::screen::{Screen, HEIGHT, WIDTH};

const focal: f64 = 0.8;
const PINHOLE: Point3<f64> = Point3::new(0.5, 0.5, focal);

pub struct Camera {
  //transformation: Decomposed,
  //projection: Ortho,
  planes: [Plane; 2],
  hole: Point3<f64>,
  pinhole_plane: Plane,
  pub rays: Vec<Ray>,
}

impl Camera {
  pub fn new() -> Self {
    Camera {
      //transformation: Decomposed::new(),
      //projection: Ortho::new(),
      rays: (0..(HEIGHT * WIDTH))
        .map(|i| {
          let x = i % WIDTH;
          let y = i / WIDTH;
          let start = Point3::new(x as f64 / WIDTH as f64, y as f64 / HEIGHT as f64, 0.0);
          Ray {
            pos: start,
            direction: (PINHOLE - start),
            color: WHITE,
          }
        })
        .collect(),
      hole: PINHOLE,
      pinhole_plane: Plane::new(
        [
          Point3::new(1.0, 0.0, focal),
          Point3::new(0.0, 1.0, focal),
          Point3::new(0.0, 0.0, focal),
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
  pub fn intersect(&self, ray: &Ray) -> Option<Point3<f64>> {
    self.planes[0]
      .intersect(ray)
      .or_else(|| self.planes[1].intersect(ray))
  }
}
