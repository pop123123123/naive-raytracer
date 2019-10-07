extern crate cgmath;
extern crate image;
use cgmath::prelude::*;
use cgmath::{dot, Point2, Point3, Vector3};

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
    RED,
  ));
  planes.push(Plane::new(
    [
      Point3::new(0.8, 0.0, 4.9),
      Point3::new(0.8, 0.8, 5.0),
      Point3::new(0.8, 0.0, 5.0),
    ],
    GREEN,
  ));
  planes.push(Plane::new(
    [
      Point3::new(-100.0, -100.0, 10.0),
      Point3::new(50.0, 100.0, 10.0),
      Point3::new(100.0, -100.0, 10.0),
    ],
    WHITE,
  ));

  let mut lights = Vec::<Light>::new();
  lights.push(Light {
    pos: Point3::new(2.5, 0.5, 1.0),
    color: WHITE,
    intensity: 1.0,
    reflet: false,
  });/*
  lights.push(Light {
    pos: Point3::new(10.5, 0.5, 1.0),
    color: RED,
    intensity: 2,
    reflet: false,
  });*/

  render(&camera, &mut screen, &lights, &planes);
}

fn save_image(screen: Vec<Color>) {
  // Create a new ImgBuf with width: imgx and height: imgy
  let mut imgbuf = image::ImageBuffer::new(WIDTH as u32, HEIGHT as u32);

  // Iterate over the coordinates and pixels of the image
  for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
    let c = screen[(WIDTH - 1 - x as usize) + (/*HEIGHT - 1 - */y as usize) * WIDTH];
    *pixel = image::Rgb([c.get_r(), c.get_g(), c.get_b()]);
  }
  imgbuf.save("render.png").unwrap();
}

fn is_closer(start: Point3<f64>, closest: Point3<f64>, furthest: Point3<f64>) -> bool {
  (furthest - start).magnitude() > (closest - start).magnitude()
}

fn closest_plane<'a>(
  ray: &Ray,
  planes: &'a Vec<Plane>,
  current_plane: Option<&Plane>,
) -> (Option<&'a Plane>, Option<Point3<f64>>) {
  planes.iter().fold(
    (None, None),
    |(closest, closest_inter): (Option<&Plane>, Option<Point3<f64>>), p| {
      let inter = p.intersect(&ray);
      if inter.is_some()
        && (current_plane.is_none() || current_plane.unwrap() != p)
        && (closest_inter.is_none() || is_closer(ray.pos, inter.unwrap(), closest_inter.unwrap()))
      {
        (Some(p), inter)
      } else {
        (closest, closest_inter)
      }
    },
  )
}

fn ray_to_color(ray: &Ray, lights: &Vec<Light>, planes: &Vec<Plane>, depth: u8) -> Color {
  if depth > 2 {
    return BLACK;
  }

  let (c_plane, closest_inter) = closest_plane(ray, planes, None);

  if c_plane.is_some() {
    ray
      .color
      .mul_element_wise(c_plane.unwrap().color)
      .mul_element_wise(
        lights
          .iter()
          .map(|light| {
            let old_direction = ray.direction;
            let ray = Ray {
              pos: closest_inter.unwrap(),
              direction: light.pos - closest_inter.unwrap(),
              color: WHITE,
            };
            let (c_plane, intersection) = closest_plane(&ray, planes, c_plane);
            if c_plane.is_none() || is_closer(ray.pos, light.pos, intersection.unwrap()) {
              light.color
            } else {
              ray_to_color(&ray, lights, planes, depth + 1)
            }//.mul_element_wise(dot(old_direction, ray.direction))
          })
          .fold(BLACK, |sum, col| sum.add_element_wise(col)),
      )
  } else {
    BLACK
  }
}

fn render(camera: &Camera, screen: &mut Screen, lights: &Vec<Light>, planes: &Vec<Plane>) {
  let mut pixels = camera
    .rays
    .iter()
    .map(|ray| ray_to_color(ray, lights, planes, 0))
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
