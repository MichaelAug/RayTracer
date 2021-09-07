use std::ops;

pub type Point3 = Vec3;
pub type Colour = Vec3;

#[derive(Default, Debug, PartialEq, Clone, Copy)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { x, y, z }
    }

    pub fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn normalized(self) -> Self{
        self / self.length()
    }

    pub fn dot(u:&Self, v:&Self) -> f64{
        u.x * v.x + u.y * v.y + u.z * v.z 
    }
}

impl ops::AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl ops::MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl ops::DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
    }
}

impl ops::Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Vec3) -> Vec3 {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl ops::Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl ops::Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self {x: self.x * rhs, y: self.y * rhs, z: self.z * rhs}
    }
}

impl ops::Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        Self {x: self.x / rhs, y: self.y / rhs, z: self.z / rhs}
    }
}

impl ops::Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Self::Output {x: rhs.x * self, y: rhs.y * self, z: rhs.z * self}
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_assign() {
        let mut v1 = Vec3::new(1.0, 2.0, 3.0);
        let mut v2 = Vec3::new(-2.0, -0.0, 2.0);
        v1 += v2.clone();

        assert_eq!(Vec3::new(-1.0, 2.0, 5.0), v1);

        v2 += Vec3::new(-3.0, -6.0, -9.0);

        assert_eq!(Vec3::new(-5.0, -6.0, -7.0), v2);
    }

    #[test]
    fn mul_assign() {
        let mut v1 = Vec3::new(1.0, 2.0, 3.0);
        v1 *= 2.0;
        assert_eq!(Vec3::new(2.0, 4.0, 6.0), v1);
    }

    #[test]
    fn div_assign() {
        let mut v1 = Vec3::new(2.0, 4.0, 6.0);
        v1 /= 2.0;
        assert_eq!(Vec3::new(1.0, 2.0, 3.0), v1);
    }

    #[test]
    fn multiply_by_float() {
        let mut v1 = Vec3::new(1.0, 2.0, 3.0);
        v1 *= -3.0;

        let mut v2 = Vec3::new(-2.0, -0.0, 2.0);
        v2 *= 2.0;

        assert_eq!(Vec3::new(-3.0, -6.0, -9.0), v1);
        assert_eq!(Vec3::new(-4.0, 0.0, 4.0), v2);
    }

    #[test]
    fn divide_by_float() {
        let mut v1 = Vec3::new(3.0, 6.0, 9.0);
        v1 /= -3.0;

        let mut v2 = Vec3::new(-2.0, -0.0, 2.0);
        v2 /= 2.0;

        assert_eq!(Vec3::new(-1.0, -2.0, -3.0), v1);
        assert_eq!(Vec3::new(-1.0, 0.0, 1.0), v2);
    }

    #[test]
    fn add() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(1.0, 2.0, 3.0);

        assert_eq!(Vec3::new(2.0, 4.0, 6.0), v1 + v2);
    }

    #[test]
    fn negate() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(-2.0, -0.0, 2.0);

        assert_eq!(Vec3::new(-1.0, -2.0, -3.0), -v1);
        assert_eq!(Vec3::new(2.0, -0.0, -2.0), -v2);
    }

    #[test]
    fn multiply_f64() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        
        assert_eq!(Vec3::new(3.0, 6.0, 9.0), v1 * 3.0);
    }

    #[test]
    fn divide_f64() {
        let v1 = Vec3::new(3.0, 6.0, 9.0);
        
        assert_eq!(Vec3::new(1.0, 2.0, 3.0), v1 / 3.0);
    }

    //TODO:
    // f64 mul test
    // dot test
    // normalize test
}