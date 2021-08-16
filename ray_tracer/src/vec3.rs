use std::ops;

#[derive(Default, Debug, PartialEq)]
pub struct Vec3 {
    x: f64,
    y: f64,
    z: f64
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
}

impl ops::Add for Vec3 {

    type Output = Self;

    fn add(self, rhs: Vec3) -> Vec3{
        Vec3 {x: self.x + rhs.x, y: self.y + rhs.y, z: self.z + rhs.z}
    }
}

impl ops::AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.x;
        self.z += rhs.x;
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vec3_add() {
        let v1 = Vec3::new(1.0,2.0,3.0);
        let v2 = Vec3::new(1.0,2.0,3.0);

        assert_eq!(Vec3::new(2.0,4.0,6.0), v1 + v2);
    }
}