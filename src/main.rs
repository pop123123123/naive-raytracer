extern crate cgmath;
extern crate image;
use cgmath::prelude::*;
use cgmath::{Point2, Point3, Vector3};

use crate::cgmath::num_traits::Pow;

mod camera;
mod light;
mod plane;
mod screen;
use camera::*;
use light::*;
use plane::*;
use screen::*;

fn main() {
  let mut screen = Screen::init(BLACK);

  let camera = Camera::new();

  let mut planes = Vec::<Plane>::new();
  planes.push(Plane::new(
    [
      Point3::new(0.0, 0.0, 5.0),
      Point3::new(0.8, 0.8, 5.0),
      Point3::new(0.8, 0.0, 5.0),
    ],
    WHITE,
  ));
  planes.push(Plane::new(
    [
      Point3::new(0.0, 0.0, 4.0),
      Point3::new(0.4, 0.8, 4.0),
      Point3::new(0.8, 0.0, 4.0),
    ],
    GREEN,
  ));

  let mut lights = Vec::<Light>::new();
  lights.push(Light {
    pos: Point3::new(10.5, 0.5, 1.0),
    color: GREEN,
    intensity: 2,
    reflet: false,
  });
  lights.push(Light {
    pos: Point3::new(10.5, 0.5, 1.0),
    color: RED,
    intensity: 2,
    reflet: false,
  });

  render(&camera, &mut screen, lights, planes, 0);
}

fn save_image(screen: Vec<Color>) {
  // Create a new ImgBuf with width: imgx and height: imgy
  let mut imgbuf = image::ImageBuffer::new(WIDTH as u32, HEIGHT as u32);

  // Iterate over the coordinates and pixels of the image
  for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
    let c = screen[x as usize + y as usize * WIDTH];
    *pixel = image::Rgb([c.get_r(), c.get_g(), c.get_b()]);
  }
  imgbuf.save("render.png").unwrap();
}

fn render(camera: &Camera, screen: &mut Screen, lights: Vec<Light>, planes: Vec<Plane>, depth: u8) {
  if depth > 1 {
    return;
  }

  let mut pixels = camera
    .rays
    .iter()
    .map(|ray| {
      let (closest_plane, closest_inter) = planes.iter().fold(
        (None, None),
        |(closest, closest_inter): (Option<&Plane>, Option<Point3<f64>>), p| {
          let inter = p.intersect(&ray);
          if inter.is_some()
            && (closest_inter.is_none()
              || (closest_inter.unwrap() - ray.pos).magnitude()
                > (inter.unwrap() - ray.pos).magnitude())
          {
            (Some(p), inter)
          } else {
            (closest, closest_inter)
          }
        },
      );

      if closest_plane.is_some() {
        let color = ray.color.mul_element_wise(closest_plane.unwrap().color);
        color.mul_element_wise(
          lights
            .iter()
            .fold(BLACK, |sum, light| sum.add_element_wise(light.color)),
        )
      } else {
        BLACK
      }
    })
    .collect::<Vec<Color>>();

  let max = pixels.iter().fold(0.0, |max, col| {
    f64::max(f64::max(col.x, col.y), col.z).max(max)
  });
  pixels.iter_mut().for_each(|mut col| {
    col.x /= max;
    col.y /= max;
    col.z /= max;
  });
  save_image(pixels);
}
