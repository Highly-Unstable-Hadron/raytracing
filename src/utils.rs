use std::ops;

#[derive(Copy, Clone, Debug)]
pub struct Point3 {
    pub x: f64,
    pub y: f64,
    pub z: f64
}

pub const UNIT_X: Point3 = Point3 {x: 1.0, y: 0.0, z: 0.0};
pub const UNIT_Y: Point3 = Point3 {x: 0.0, y: 1.0, z: 0.0};
pub const UNIT_Z: Point3 = Point3 {x: 0.0, y: 0.0, z: 1.0};
pub const ORIGIN: Point3 = Point3 {x: 0.0, y: 0.0, z: 0.0};

pub const MINIMUM: f64 = 0.000001;
pub const INFINITY: f64 = f64::INFINITY;

impl Point3 {
    pub fn construct(x: f64, y: f64, z: f64) -> Point3 {
        Point3 {x, y, z}
    }
    fn sum(self) -> f64 {
        self.x + self.y + self.z
    }
    pub fn norm_square(self) -> f64 {
        (self * self).sum()
    }
    pub fn norm(self) -> f64 {
        self.norm_square().powf(0.5)
    }
    pub fn dot(self, rhs: Point3) -> f64 {
        (self * rhs).sum()
    }
    pub fn unit_vector(self) -> Point3 {
        self / self.norm()
    }
    pub fn cross(self, rhs: Point3) -> Point3 {
        Point3 {
            x: self.y*rhs.z - self.z*rhs.y,
            y: self.z*rhs.x - self.x*rhs.z,
            z: self.x*rhs.y - self.y*rhs.x
        }
    }
}

impl ops::Neg for Point3 {
    type Output = Point3;
    fn neg(self) -> Point3 {
        Point3 {x: -self.x, y: -self.y, z: -self.z}
    }
}

impl ops::Add<Point3> for Point3 {
    type Output = Point3;
    fn add(self, rhs: Point3) -> Point3 {
        Point3 {x: self.x + rhs.x, y: self.y + rhs.y, z: self.z + rhs.z}
    }
}

impl ops::AddAssign<f64> for Point3 {
    fn add_assign(&mut self, rhs: f64) {
        self.x += rhs;
        self.y += rhs;
        self.z += rhs;
    }
}

impl ops::AddAssign<Point3> for Point3 {
    fn add_assign(&mut self, rhs: Point3) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl ops::Sub<Point3> for Point3 {
    type Output = Point3;
    fn sub(self, rhs: Point3) -> Point3 {
        Point3 {x: self.x - rhs.x, y: self.y - rhs.y, z: self.z - rhs.z}
    }
}

impl ops::SubAssign<f64> for Point3 {
    fn sub_assign(&mut self, rhs: f64) {
        self.x -= rhs;
        self.y -= rhs;
        self.z -= rhs;
    }
}

impl ops::SubAssign<Point3> for Point3 {
    fn sub_assign(&mut self, rhs: Point3) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl ops::Mul<Point3> for Point3 {
    type Output = Point3;
    fn mul(self, rhs: Point3) -> Point3 {
        Point3 {x: self.x * rhs.x, y: self.y * rhs.y, z: self.z * rhs.z}
    }
}

impl ops::Mul<f64> for Point3 {
    type Output = Point3;
    fn mul(self, rhs: f64) -> Point3 {
        Point3 {x: self.x * rhs, y: self.y * rhs, z: self.z * rhs}
    }
}

impl ops::Mul<i32> for Point3 {
    type Output = Point3;
    fn mul(self, rhs_: i32) -> Point3 {
        let rhs = rhs_ as f64;
        Point3 {x: self.x * rhs, y: self.y * rhs, z: self.z * rhs}
    }
}

impl ops::MulAssign<f64> for Point3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl ops::Div<f64> for Point3 {
    type Output = Point3;
    fn div(self, rhs_: f64) -> Point3 {
        let rhs = 1.0/rhs_;
        Point3 {x: self.x * rhs, y: self.y * rhs, z: self.z * rhs}
    }
}

impl ops::Div<i32> for Point3 {
    type Output = Point3;
    fn div(self, rhs_: i32) -> Point3 {
        let rhs = 1.0 / rhs_ as f64;
        Point3 {x: self.x * rhs, y: self.y * rhs, z: self.z * rhs}
    }
}

impl ops::DivAssign<f64> for Point3 {
    fn div_assign(&mut self, rhs: f64) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Color3 {
    pub r: i32,
    pub g: i32,
    pub b: i32
}

impl Color3 {
    pub fn construct(r_: f64, g_: f64, b_: f64) -> Color3 {
        for c in vec![r_, g_, b_] {
            if c < 0.0 || c > 1.0 {
                panic!("Invalid colouring! r={} g={} b={}", r_, g_, b_);
            }
        }
        Color3 {r: (255.99*r_) as i32, g: (255.99*g_) as i32, b: (255.99*b_) as i32}
    }
    pub fn from_point3(point: Point3) -> Color3 {
        Self::construct(point.x, point.y, point.z)
    }
    pub fn print_out(&self) {
        let mut gamma_corrected = self.to_point3();
        // gamma_corrected.x = gamma_corrected.x.powf(0.5);
        // gamma_corrected.y = gamma_corrected.y.powf(0.5);
        // gamma_corrected.z = gamma_corrected.z.powf(0.5);
        let c = Color3::from_point3(gamma_corrected);
        println!("{} {} {}", c.r, c.g, c.b);
    }
    pub fn to_point3(&self) -> Point3 {
        Point3 {x: self.r as f64/255.99, y: self.g as f64/255.99, z: self.b as f64/255.99}
    }
}

#[derive(Debug, Clone)]
pub struct Ray {
    pub A: Point3,
    pub B: Point3
    // Ray(t: f64) = A*t + B
}

impl Ray {
    pub fn produce(&self, t: f64) -> Point3 {
        self.A * t + self.B
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Face { // face to camera
    FrontFace,
    BackFace
}

pub trait Material {
    fn scatter(&self, ray: &Ray, hit: &RayHit) -> Ray;
    fn attenuate(&self, color: Color3) -> Color3;
}

#[derive(Clone)]
pub enum RayHit<'a> {
    Hit {
        t: f64,
        point: Point3,
        real_normal: Point3,
        fake_normal: Point3,
        face: Face,
        material: &'a Box<dyn Material>
    },
    NoHit
}