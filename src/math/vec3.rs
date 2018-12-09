use std::ops::Add;
use std::ops::AddAssign;
use std::ops::Sub;
use std::ops::SubAssign;
use std::ops::Mul;
use std::ops::MulAssign;
use std::ops::Div;
use std::ops::DivAssign;
use std::ops::Neg;
use std::fmt;

#[derive(Copy, Clone)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    
    pub const ZERO : Vec3 = Vec3{x:0.0, y:0.0, z:0.0};
    pub const ONE : Vec3 = Vec3{x:1.0, y:1.0, z:1.0};

    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3{x, y, z}
    }

    pub fn normalize(self) -> Vec3 {        
        let k = 1.0 / self.length();
        Vec3 {
            x : self.x * k,
            y : self.y * k,
            z : self.z * k, 
        }
    }

    pub fn length(self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt() as f32
    }

    pub fn squared_length(self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn dot(self, rhs: Vec3) -> f32 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    pub fn cross(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            x : self.y * rhs.z - self.z * rhs.y,
            y : -(self.x * rhs.z - self.z * rhs.x),
            z : self.x * rhs.y - self.y * rhs.x, 
        }
    }

    pub fn reflect(self, rhs: Vec3) -> Vec3 {
        self - rhs * 2.0 * self.dot(rhs)
    }
}

impl Neg for Vec3 {
    type Output = Vec3;
    fn neg(self) -> Vec3 {
        Vec3{x: -self.x, y: -self.y, z: -self.z}
    }
}

impl Add for Vec3 {
    type Output = Vec3;
    fn add(self, rhs: Vec3) -> Vec3 {
        Vec3 { 
            x : self.x + rhs.x,
            y : self.y + rhs.y,
            z : self.z + rhs.z
        }
    }
}

impl Add<f32> for Vec3 {
    type Output = Vec3;
    fn add(self, rhs: f32) -> Vec3 {
        Vec3 { 
            x : self.x + rhs,
            y : self.y + rhs,
            z : self.z + rhs
        }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Vec3) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl Sub for Vec3 {
    type Output = Vec3;
    fn sub(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            x : self.x - rhs.x,
            y : self.y - rhs.y,
            z : self.z - rhs.z
        }
    }
}

impl Sub<f32> for Vec3 {
    type Output = Vec3;
    fn sub(self, rhs: f32) -> Vec3 {
        Vec3 {
            x : self.x - rhs,
            y : self.y - rhs,
            z : self.z - rhs
        }
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Vec3) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl Mul for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            x : self.x * rhs.x,
            y : self.y * rhs.y,
            z : self.z * rhs.z
        }
    }
}

impl Mul<f32> for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: f32) -> Vec3 {
        Vec3 {
            x : self.x * rhs,
            y : self.y * rhs,
            z : self.z * rhs
        }
    }
}

impl MulAssign for Vec3 {
    fn mul_assign(&mut self, rhs: Vec3) {
        self.x *= rhs.x;
        self.y *= rhs.y;
        self.z *= rhs.z;
    }
}

impl MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, rhs: f32) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl Div for Vec3 {
    type Output = Vec3;
    fn div(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            x : self.x / rhs.x,
            y : self.y / rhs.y,
            z : self.z / rhs.z
        }
    }
}

impl Div<f32> for Vec3 {
    type Output = Vec3;
    fn div(self, rhs: f32) -> Vec3 {
        Vec3 {
            x : self.x / rhs,
            y : self.y / rhs,
            z : self.z / rhs
        }
    }
}

impl DivAssign for Vec3 {
    fn div_assign(&mut self, rhs: Vec3) {
        self.x /= rhs.x;
        self.y /= rhs.y;
        self.z /= rhs.z;
    }
}

impl DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, rhs: f32) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
    }
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        write!(f, "{} {} {}", self.x, self.y, self.z)
    }
}
