mod utils;
mod camera;
mod object;
mod material;
use utils::{Point3, UNIT_X, UNIT_Y, UNIT_Z, ORIGIN};
use camera::Camera;
use object::{World, Sphere};
use material::{Metallic, Diffuse, Dielectric, LightSource};

fn main() {
    let image_width = 512;
    let image_height = image_width*9/16;
    
    let world = World {
        objects: vec![
            // ground
            Box::new(Sphere {
                position: Point3::construct(0.0, 1.0, -100.5),
                radius: 100.0,
                material: Box::new(Diffuse::construct(0.8, 0.8, 0.0))
            }),
            // middle
            Box::new(Sphere {
                position: Point3::construct(0.0, 1.2, 0.0),
                radius: 0.5,
                material: Box::new(Diffuse::construct(0.1, 0.2, 0.5))
            }),
            // left
            Box::new(Sphere {
                position: Point3::construct(-1.0, 1.0, 0.0),
                radius: 0.5,
                material: 
                // Box::new(LightSource::construct())
                Box::new(Dielectric {refractive_index: 1.50})
                // Box::new(Metallic::construct(0.8, 0.8, 0.8, 0.3))
            }),
            // right
            Box::new(Sphere {
                position: Point3::construct(1.0, 1.0, 0.0),
                radius: 0.5,
                material: Box::new(Metallic::construct(0.8, 0.6, 0.2, 1.0))
            })
        ]
    };

    let mut camera = Camera::construct(&world, image_width, image_height, 
        ORIGIN, // camera position
        UNIT_Y, // viewport (center) position
        Point3 {x: 2.0*(image_width as f64 / image_height as f64), z: -2.0, y: 0.0} // viewport diagonal
    );
    camera.render_ascii_ppm();
}
