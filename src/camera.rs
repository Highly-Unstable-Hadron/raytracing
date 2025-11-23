use crate::utils::{Point3, Color3, Ray, UNIT_X, UNIT_Z, RayHit, MINIMUM, INFINITY};
use crate::object::{World};
use rand::prelude::*;

#[derive(Debug, Clone)]
struct Viewport {
    // viewport = (positon vector to top left corner, position vector to bottom right corner)
    position: (Point3, Point3),
    dx: Point3, 
    dy: Point3,
    first_pixel_center: Point3,
    rng: ThreadRng
}

impl Viewport {
    pub fn construct(position: (Point3, Point3), dx: Point3, dy: Point3, first_pixel_center: Point3) -> Viewport {
        let v = Viewport {
            position, dx, dy, first_pixel_center, rng: rand::rng()
        };
        v
    }
    pub fn produce_ray(&mut self, i: i32, j: i32, orig: Point3) -> Ray {
        let offset_x: f64 = self.rng.random_range(-0.5..0.5);
        let offset_y: f64 = self.rng.random_range(-0.5..0.5);
        Ray::construct(
            (self.first_pixel_center + 
            self.dx*(i as f64 + offset_x) + 
            self.dy*(j as f64 + offset_y) - orig).unit_vector(), 
            orig
        )
    }
}

// #[derive(Debug)]
pub struct Camera<'a> {
    pub image_height: i32,
    pub image_width: i32,
    pub camera: Point3,
    viewport: Viewport,
    pub world: &'a World,
    pub pixel_samples: i32,
    pub scatter_depth: i32
}

impl Camera<'_> {
    fn header_ascii_ppm(&self) {
        println!("P3\n{} {}", self.image_width, self.image_height);
        println!("255");
    }
    pub fn construct<'a>(world: &'a World, width: i32, height: i32, camera: Point3, viewport_center: Point3, viewport_diagonal: Point3, pixel_samples: i32, scatter_depth: i32) -> Camera<'a> {
        let position = (viewport_center - viewport_diagonal / 2, 
                        viewport_center + viewport_diagonal / 2);
        let dx = viewport_diagonal*UNIT_X / width;
        let dy = viewport_diagonal*UNIT_Z / height;
        Camera {
            image_width: width,
            image_height: height,
            camera,
            viewport: Viewport::construct(position, dx, dy, position.0 + (dx + dy)/2),
            world, pixel_samples, scatter_depth
        }
    }
    pub fn render_ascii_ppm(&mut self) {
        self.header_ascii_ppm();
        eprintln!();
        for j in 0..self.image_height {
            for i in 0..self.image_width {
                let mut pixel = Point3 {x: 0.0, y: 0.0, z: 0.0};
                for _ in 0..self.pixel_samples {  // anti-aliasing
                    let ray = self.viewport.produce_ray(i, j, self.camera);
                    pixel += self.ray_color(ray).to_point3();
                }
                pixel /= self.pixel_samples as f64;
                Color3::construct(pixel.x, pixel.y, pixel.z).print_out();
            }
            eprint!("\r{}% done: {} rows of {} total", 100.0*j as f64/self.image_height as f64, j, self.image_height);
        }
        eprintln!();
        eprintln!("Done");
    }
    pub fn ray_color(&self, ray: Ray) -> Color3 {
        let t = 0.5*(ray.A.z + 1.0);
        let init = Point3 {x: 0.4, y: 0.6, z: 1.0};
        let end = Point3 {x: 1.0, y: 1.0, z: 1.0};
        let lerp = end * (1.0 - t) + init * t;
        let background = Color3::construct(lerp.x, lerp.y, lerp.z);

        let scattered_ray = ray.clone();
        let mut color = background;
        let mut hit = self.world.hit(&scattered_ray, (MINIMUM, INFINITY));

        for _ in 0..self.scatter_depth {
            let RayHit::Hit {material, ..} = hit else { break; };
            color = material.attenuate(color);
            let Some(scattered_ray) = material.scatter(&scattered_ray, &hit) else { break; };
            hit = self.world.hit(&scattered_ray, (MINIMUM, INFINITY));
        }

        color
    }
}