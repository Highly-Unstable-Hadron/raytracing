use crate::utils::{Point3, Ray, Face, RayHit, Material};

pub struct World {
    pub objects: Vec<Box<dyn Object>>
}

impl World {
    pub fn hit(&self, ray: &Ray, t_range: (f64, f64)) -> RayHit<'_> {
        let mut closest: RayHit = RayHit::NoHit;
        let mut min_t: f64 = f64::INFINITY;

        for object in &self.objects {
            let hit: RayHit = object.ray_hit(ray, t_range);
            closest = match hit {
                RayHit::Hit {t, ..} => if t < min_t { min_t = t; hit } else { closest },
                RayHit::NoHit => closest
            }
        }
        closest
    }
}

pub trait Object {
    fn ray_hit(&self, ray: &Ray, t_range: (f64, f64)) -> RayHit<'_>;
}

pub struct Sphere {
    pub position: Point3,
    pub radius: f64,
    pub material: Box<dyn Material>
}

impl Object for Sphere {
    fn ray_hit(&self, ray: &Ray, t_range: (f64, f64)) -> RayHit<'_> {
        let b = ray.A.dot(self.position - ray.B) * (-2.0);
        let a = ray.A.norm_square();
        let c = (self.position - ray.B).norm_square() - self.radius*self.radius;
        let discriminant = b*b - 4.0*a*c;

        if discriminant < 0.0 {
            return RayHit::NoHit;
        }
        let t = (-b - discriminant.powf(0.5)) / (2.0*a);
        if t < t_range.0 || t > t_range.1 {
            return RayHit::NoHit;
        }
        let point = ray.produce(t);
        let normal = (point - self.position).unit_vector();

        RayHit::Hit{
            t,
            point,
            real_normal: normal,
            face: if normal.dot(point) < 0.0 { Face::FrontFace } else { Face::BackFace },
            fake_normal: if normal.dot(point) < 0.0 { normal } else { -normal }, // always points opposite to ray
            material: &self.material
        }
    }
}