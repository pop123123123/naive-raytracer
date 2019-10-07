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
  println!("Hello, world!");
  let v = Vector3::new(1.0, 2.0, 3.0);
  let mut screen = Screen::init(BLACK);

  let p = Plane::new(
    [
      Point3::new(0.0, 0.0, 5.0),
      Point3::new(0.8, 0.8, 5.0),
      Point3::new(0.8, 0.0, 5.0),
    ],
    WHITE,
  );

  let camera = Camera::new();

  let light = Light {
    pos: Point3::new(10.5, 0.5, 1.0),
    color: GREEN,
    intensity: 2,
    reflet: false,
  };

  compute_light(&camera, &mut screen, &light, &p, 0);
  screen.normalize();

  save_image(&screen);
}

fn save_image(screen : &Screen) {
  // Create a new ImgBuf with width: imgx and height: imgy
  let mut imgbuf = image::ImageBuffer::new(WIDTH as u32, HEIGHT as u32);

  // Iterate over the coordinates and pixels of the image
  for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
    let c = screen.get(x as usize, y as usize);
    *pixel = image::Rgb([c.get_r(), c.get_g(), c.get_b()]);
  }
  imgbuf.save("render.png").unwrap();
}

fn compute_light(camera: &Camera, screen: &mut Screen, light: &Light, p: &Plane, depth: u8) {
  if depth > 1 {
    return;
  }

  light.into_iter().for_each(|ray| {
    //println!("jaj {:?}", ray.direction.magnitude());
    let plan = p.intersect(&ray);
    let mut cam = camera.intersect(&ray);

    if !light.reflet
      && plan.is_some()
      && (cam.is_none()
        || (cam.unwrap() - ray.pos).magnitude() > (plan.unwrap() - ray.pos).magnitude())
    {
      let light = p.light_from_ray(&ray);
      compute_light(&camera, screen, &light, &p, depth + 1);
    } else if camera.intersect_pinhole(&ray) && cam.is_some() {
      screen.add_point(
        Point2::from_vec(cam.unwrap().to_vec().truncate()),
        (ray.color * Vector3::dot(ray.direction, camera.normal())).map(|n| n.max(0.0)),
      )
    }
  });
}
