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
        if t <= t_range.0 || t >= t_range.1 {
            return RayHit::NoHit;
        }
        let point = ray.produce(t);
        let normal = (point - self.position).unit_vector();

        RayHit::Hit{
            t,
            point,
            face: if normal.dot(point) < 0.0 { Face::FrontFace } else { Face::BackFace },
            normal: if normal.dot(point) < 0.0 { normal } else { -normal }, // always points opposite to ray
            material: &self.material
        }
    }
}

pub struct Plane {
    pub mx: f64,
    pub my: f64,
    pub b: f64,
    pub normal: Point3  // which way the plane is facing
}

impl Plane {
    pub fn construct(mx: f64, my: f64, b: f64, internal_pt: Point3) -> Plane {
        let along_x = Point3 {x: 1.0, y: 0.0, z: mx};
        let along_y = Point3 {x: 0.0, y: 1.0, z: my};
        let normal = along_x.cross(along_y);
        
        // panic if internal_pt is on the plane
        let z = internal_pt.x * mx + internal_pt.y * my + b;
        if (internal_pt.z - z).abs() < 0.00001 {
            panic!("Point used to construct normal to plane cannot be on the plane!\nPoint {:#?}", internal_pt);
        }

        Plane {
            mx, my, b, 
            normal: if internal_pt.dot(normal) > 0.0 { -normal } else { normal } // TODO: check if normal is correct
        }
    }
    pub fn intersect(&self, ray: &Ray) -> Option<f64> {
        // Equation of plane: z = mx*x + my*y + b 
        // Also equation of plane: z = m . [x, y, 0] + b  where m = [mx, my, -1]
        // => Az*t + Bz = mx*(Ax*t+Bx) + my*(Ay*t+By) + b
        // solve for t.
        let to_mul: Point3 = Point3 {x: self.mx, y: self.my, z: -1.0};
        let numerator = ray.B.dot(to_mul) + self.b;
        let denominator = ray.A.dot(-to_mul);
        let t = numerator / denominator;

        if self.is_on_plane(ray.produce(t)) {
            Some(t)
        } else {
            None
        }
    }
    pub fn is_on_plane(&self, pt: Point3) -> bool {
        let z = pt.x * self.mx + pt.y * self.my + self.b;
        if (pt.z - z).abs() < 0.00001 {
            true
        } else {
            false
        }
    }
}

pub struct Triangle {
    // pub A: Point3,
    // pub B: Point3,
    // pub C: Point3,
    pub material: Box<dyn Material>,
    pub plane: Plane
}

impl Triangle {
    pub fn construct(A: Point3, B: Point3, C: Point3, material: Box<dyn Material>) -> Triangle {
        Triangle {
            material,
            plane: Plane::construct(0.0, 0.0, 0.0, Point3 {x: 0.0, y: 0.0, z: 0.0}) // TODO:
        }
    }
}

impl Object for Triangle {
    fn ray_hit(&self, ray: &Ray, t_range: (f64, f64)) -> RayHit<'_> {
        let Some(t) = self.plane.intersect(ray) else {return RayHit::NoHit};
        if t <= t_range.0 || t >= t_range.1 {
            return RayHit::NoHit;
        }
        let normal = self.plane.normal;
        let point = ray.produce(t);

        RayHit::Hit {
            t,
            point,
            normal: if normal.dot(point) < 0.0 { normal } else { -normal },
            face: if normal.dot(point) < 0.0 { Face::FrontFace } else { Face::BackFace },
            material: &self.material
        }
    }
}


