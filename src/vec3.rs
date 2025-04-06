use std::fmt;
use std::ops::{Add, Sub, Mul, Div, Neg};

#[derive(Copy, Clone)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { x, y, z }
    }

    pub fn squared_length(self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn length(self) -> f64 {
        self.squared_length().sqrt()
    }

    pub fn unit_vector(self) -> Vec3 {
        self / self.length()
    }

    pub fn dot(self, v: Vec3) -> f64 {
        self.x * v.x + self.y * v.y + self.z * v.z
    }

    pub fn cross(self, v: Vec3) -> Vec3 {
        Vec3::new(
            self.y * v.z - self.z * v.y,
            self.z * v.x - self.x * v.z,
            self.x * v.y - self.y * v.x,
        )
    }
}

// Add
impl Add<Vec3> for Vec3 {
    type Output = Vec3;
    fn add(self, v: Vec3) -> Vec3 {
        Vec3::new(self.x + v.x, self.y + v.y, self.z + v.z)
    }
}

impl Add<f64> for Vec3 {
    type Output = Vec3;
    fn add(self, f: f64) -> Vec3 {
        Vec3::new(self.x + f, self.y + f, self.z + f)
    }
}

// Subtract
impl Sub<Vec3> for Vec3 {
    type Output = Vec3;
    fn sub(self, v: Vec3) -> Vec3 {
        Vec3::new(self.x - v.x, self.y - v.y, self.z - v.z)
    }
}

impl Sub<f64> for Vec3 {
    type Output = Vec3;
    fn sub(self, f: f64) -> Vec3 {
        Vec3::new(self.x - f, self.y - f, self.z - f)
    }
}

// Multiply
impl Mul<Vec3> for Vec3 {
    type Output = Vec3;
    fn mul(self, v: Vec3) -> Vec3 {
        Vec3::new(self.x * v.x, self.y * v.y, self.z * v.z)
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;
    fn mul(self, f: f64) -> Vec3 {
        Vec3::new(self.x * f, self.y * f, self.z * f)
    }
}

// Divide
impl Div<Vec3> for Vec3 {
    type Output = Vec3;
    fn div(self, v: Vec3) -> Vec3 {
        Vec3::new(self.x / v.x, self.y / v.y, self.z / v.z)
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;
    fn div(self, f: f64) -> Vec3 {
        Vec3::new(self.x / f, self.y / f, self.z / f)
    }
}

// Negate
impl Neg for Vec3 {
    type Output = Vec3;
    fn neg(self) -> Vec3 {
        Vec3::new(-self.x, -self.y, -self.z)
    }
}

// Display
impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {}", self.x, self.y, self.z)
    }
}

pub fn test() {
    let u = Vec3::new(1.0, 2.0, 3.0);
    let v = Vec3::new(-2.0, 1.0, 2.0);

    eprintln!("{:>20}  =  {}", "u", u);
    eprintln!("{:>20}  =  {}", "v", v);
    eprintln!("{:>20}  =  {}", "-u", -u);
    eprintln!("{:>20}  =  {}", "u + 1.0", u + 1.0);
    eprintln!("{:>20}  =  {}", "u - 1.0", u - 1.0);
    eprintln!("{:>20}  =  {}", "u * -2.0", u * -2.0);
    eprintln!("{:>20}  =  {}", "u / 0.5", u / 0.5);
    eprintln!("{:>20}  =  {}", "u + v", u + v);
    eprintln!("{:>20}  =  {}", "u - v", u - v);
    eprintln!("{:>20}  =  {}", "u * v", u * v);
    eprintln!("{:>20}  =  {}", "u / v", u / v);
    eprintln!("{:>20}  =  {}", "u.squared_length()", u.squared_length());
    eprintln!("{:>20}  =  {}", "u.length()", u.length());
    eprintln!("{:>20}  =  {}", "u.unit_vector()", u.unit_vector());
    eprintln!("{:>20}  =  {}", "u.dot(v)", u.dot(v));
    eprintln!("{:>20}  =  {}", "v.dot(u)", v.dot(u));
    eprintln!("{:>20}  =  {}", "u.cross(v)", u.cross(v));
    eprintln!("{:>20}  =  {}", "v.cross(u)", v.cross(u));
}
