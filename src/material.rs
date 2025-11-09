use crate::utils::{Point3, Ray, Color3, RayHit, Face, Material, MINIMUM, INFINITY};

pub struct Metallic {
    albedo: Point3,
    fuzz: f64
}

impl Metallic {
    pub fn construct(r_: f64, g_: f64, b_: f64, fuzz: f64) -> Metallic {
        for c in vec![r_, g_, b_, fuzz] {
            if c < 0.0 || c > 1.0 {
                panic!("Invalid albedo (for metal)! r={} g={} b={} fuzz={}", r_, g_, b_, fuzz);
            }
        }
        Metallic {albedo: Point3 {x: r_, y: g_, z: b_}, fuzz}
    }
}

impl Material for Metallic {
    fn scatter(&self, ray: &Ray, hit: &RayHit) -> Option<Ray> {
        let RayHit::Hit {fake_normal, point, ..} = *hit else { panic!("metallic scatter attempted on a NoHit") };
        let mut reflected_vec = ray.A - fake_normal*fake_normal.dot(ray.A)*2.0;
        if reflected_vec.norm() < MINIMUM {
            reflected_vec = fake_normal;
        }
        let random_vec = Point3 {x: rand::random(), y: rand::random(), z: rand::random()}; // TODO: INEFFICIENT
        reflected_vec = reflected_vec.unit_vector() + random_vec.unit_vector() * self.fuzz;

        Some(Ray::construct(reflected_vec, point))
    }

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
                panic!("Invalid albedo (for diffuse material)! r={} g={} b={}", r_, g_, b_);
            }
        }
        Diffuse {albedo: Point3 {x: r_, y: g_, z: b_}}
    }
}

impl Material for Diffuse {
    fn scatter(&self, _ray: &Ray, hit: &RayHit) -> Option<Ray> {
        let RayHit::Hit {fake_normal, point, ..} = *hit else { panic!("diffuse scatter attempted on a NoHit") };
        let mut random_vec = Point3 {x: rand::random(), y: rand::random(), z: rand::random()}; // TODO: INEFFICIENT
        random_vec = random_vec.unit_vector() + fake_normal;
        
        if random_vec.norm() < MINIMUM && random_vec.norm() >= INFINITY {
            random_vec = fake_normal;
        }

        Some(Ray::construct(random_vec, point))
    }

    fn attenuate(&self, color: Color3) -> Color3 {
        Color3::from_point3(self.albedo * color.to_point3())
    }
}

pub struct Dielectric {
    pub refractive_index: f64
}

impl Material for Dielectric {
    fn scatter(&self, ray: &Ray, hit: &RayHit) -> Option<Ray> {
        let RayHit::Hit {fake_normal, point, face, ..} = *hit else { panic!("diffuse scatter attempted on a NoHit") };
        let refractive_ratio = match face {
            Face::FrontFace => 1.0/self.refractive_index,
            Face::BackFace => self.refractive_index
        };
        let refracted_y = fake_normal.cross(ray.A.cross(fake_normal) * refractive_ratio);
        let refracted_x = -fake_normal * (1.0 - refracted_y.norm_square()).powf(0.5);

        let ratio_of_sines = (refracted_x + refracted_y).cross(fake_normal).norm() / ray.A.cross(fake_normal).norm();
        if ratio_of_sines > refractive_ratio + 0.0001 || ratio_of_sines < refractive_ratio - 0.0001 {
            panic!("sine ratio {:?} and refractive ratio {:?}", ratio_of_sines, refractive_ratio);
        }
        if (refracted_x + refracted_y).dot(ray.A) <= 0.0 {
            panic!("refracted vec {:#?}, ray {:#?}, dot product {:#?}", (refracted_x + refracted_y), ray.A, (refracted_x + refracted_y).dot(ray.A))
        }
        Some(Ray::construct(refracted_x + refracted_y, point))
    }

    fn attenuate(&self, color: Color3) -> Color3 {
        color
    }
}

pub struct LightSource {
    color: Color3
}

impl LightSource {
    pub fn construct() -> LightSource {
        LightSource {color: Color3::construct(1.0, 1.0, 1.0)}
    }
}

impl Material for LightSource {
    fn scatter(&self, ray: &Ray, hit: &RayHit) -> Option<Ray> {
        None
    }

    fn attenuate(&self, color: Color3) -> Color3 {
        self.color
    }
}