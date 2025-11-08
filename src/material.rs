use crate::utils::{Point3, Ray, Color3, RayHit, Material, MINIMUM, INFINITY};

pub struct Metallic {
    albedo: Point3
}

impl Metallic {
    pub fn construct(r_: f64, g_: f64, b_: f64) -> Metallic {
        for c in vec![r_, g_, b_] {
            if c < 0.0 || c > 1.0 {
                panic!("Invalid colouring! r={} g={} b={}", r_, g_, b_);
            }
        }
        Metallic {albedo: Point3 {x: r_, y: g_, z: b_}}
    }
}

impl Material for Metallic {
    fn scatter(&self, ray: &Ray, hit: &RayHit) -> Ray {
        let RayHit::Hit {fake_normal, point, ..} = hit else { panic!("metallic scatter attempted on a NoHit") };
        let mut reflected_vec = ray.A - (*fake_normal)*fake_normal.dot(ray.A)*2.0;
        if reflected_vec.norm() < MINIMUM {
            reflected_vec = *fake_normal;
        }
        Ray {A: reflected_vec, B: *point}
    }

    #[inline]
    fn attenuate(&self, color: Color3) -> Color3 {
        Color3::from_point3(self.albedo * color.to_point3())
    }
}

pub struct Diffuse {
    albedo: Point3
}

impl Diffuse {
    pub fn construct(r_: f64, g_: f64, b_: f64) -> Diffuse {
        for c in vec![r_, g_, b_] {
            if c < 0.0 || c > 1.0 {
                panic!("Invalid colouring! r={} g={} b={}", r_, g_, b_);
            }
        }
        Diffuse {albedo: Point3 {x: r_, y: g_, z: b_}}
    }
}

impl Material for Diffuse {
    fn scatter(&self, ray: &Ray, hit: &RayHit) -> Ray {
        let RayHit::Hit {fake_normal, point, ..} = hit else { panic!("diffuse scatter attempted on a NoHit") };
        let mut random_vec = Point3 {x: rand::random(), y: rand::random(), z: rand::random()}; // TODO: INEFFICIENT
        random_vec = random_vec.unit_vector() + *fake_normal;
        
        if random_vec.norm() < MINIMUM && random_vec.norm() >= INFINITY {
            random_vec = *fake_normal;
        }

        Ray {A: random_vec, B: *point}
    }

    #[inline]
    fn attenuate(&self, color: Color3) -> Color3 {
        Color3::from_point3(self.albedo * color.to_point3())
    }
}