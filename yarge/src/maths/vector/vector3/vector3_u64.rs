use std::simd::prelude::*;

/// A structure to represent a 3 dimensional u64 vector
#[derive(Clone, Copy)]
#[repr(C)]
pub struct Vector3u64 {
    data: u64x4, // can't do simd on line of 3
}

/// A structure to be able to create `.x`, `.y`, and `.z` getters and setters
mod private {
    #[repr(C)]
    pub struct CoordsVector3 {
        pub x: u64,
        pub y: u64,
        pub z: u64,
        _pad: u64, // can't do simd on line of 3
    }
}

/// Implements `Deref` to allow accessing `.x`, `.y`, and `.z`
impl std::ops::Deref for Vector3u64 {
    type Target = private::CoordsVector3;

    fn deref(&self) -> &Self::Target {
        let value: *const Vector3u64 = self;
        unsafe { &*(value as *const private::CoordsVector3) }
    }
}

/// Implements `DerefMut` to allow modifying `.x`, `.y`, and `.z`
impl std::ops::DerefMut for Vector3u64 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        let value: *mut Vector3u64 = self;
        unsafe { &mut *(value as *mut private::CoordsVector3) }
    }
}

/// Overrides the debug trait
impl std::fmt::Debug for Vector3u64 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Vector3u64")
            .field("x", &self.x)
            .field("y", &self.y)
            .field("z", &self.z)
            .finish()
    }
}

/// Overrides the display trait
impl std::fmt::Display for Vector3u64 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

/// Sets a 3 dimensional u64 vector to `[0, 0, 0]` as default
impl Default for Vector3u64 {
    fn default() -> Self {
        Self::ZEROS
    }
}

/// Creates a 3 dimensional u64 vector
pub fn vec3u64(x: u64, y: u64, z: u64) -> Vector3u64 {
    Vector3u64::new(x, y, z)
}

/// A union to cast simd to array and allow const construction
union UnionCast {
    array: [u64; 4],
    simd: Vector3u64,
}

impl Vector3u64 {
    //////////////////////////////////////////////////////////
    /////////////      vector creation       /////////////////
    //////////////////////////////////////////////////////////

    /// Creates a new vector given its coordinates
    pub const fn new(x: u64, y: u64, z: u64) -> Self {
        unsafe {
            UnionCast {
                array: [x, y, z, 1],
            }
            .simd
        }
    }

    /// Creates a new vector with all coordinates set to `value`
    const fn splat(value: u64) -> Self {
        Self::new(value, value, value)
    }

    /// Creates a new vector filled with `values`
    pub const fn filled(value: u64) -> Self {
        Self::splat(value)
    }

    /// Creates a new vector filled with ones
    pub const ONES: Self = Self::splat(1);

    /// Creates a new vector filled with zeros
    pub const ZEROS: Self = Self::splat(0);

    /// Creates a new vector filled with `u64::MIN``
    pub const MIN: Self = Self::splat(u64::MIN);

    /// Creates a new vector filled with `u64::MAX`
    pub const MAX: Self = Self::splat(u64::MAX);

    /// Creates a new vector pointing along the positive X axis
    pub const X: Self = Self::new(1, 0, 0);

    /// Creates a new vector pointing along the positive Y axis
    pub const Y: Self = Self::new(0, 1, 0);

    /// Creates a new vector pointing along the positive Z axis
    pub const Z: Self = Self::new(0, 0, 1);

    //////////////////////////////////////////////////////////
    /////////////     vector operations      /////////////////
    //////////////////////////////////////////////////////////

    /// Sums up the element of the vector
    pub fn prefix_sum(self) -> u64 {
        self.x + self.y + self.z
    }

    /// Dot product between two vectors
    pub fn dot(v1: &Vector3u64, v2: &Vector3u64) -> u64 {
        (v1 * v2).prefix_sum()
    }

    /// Cross product between two vectors
    pub fn cross(v1: &Vector3u64, v2: &Vector3u64) -> Vector3u64 {
        Self::new(
            v1.y * v2.z - v1.z * v2.y,
            v1.z * v2.x - v1.x * v2.z,
            v1.x * v2.y - v1.y * v2.x,
        )
    }

    /// Returns the length of the vector
    pub fn length(&self) -> f32 {
        (Self::dot(self, self) as f32).sqrt()
    }
}

impl PartialEq for Vector3u64 {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}

//////////////////////////////////////////////////////////
//////////////     vector additions     //////////////////
//////////////////////////////////////////////////////////

/// Components wise addition
/// v1 + v2 = [v1.x + v2.x, v1.y + v2.y, v1.z + v2.z]
impl std::ops::Add<Vector3u64> for Vector3u64 {
    type Output = Vector3u64;

    fn add(self, rhs: Vector3u64) -> Self::Output {
        Vector3u64 {
            data: self.data + rhs.data,
        }
    }
}

/// Components wise addition
/// v1 + v2 = [v1.x + v2.x, v1.y + v2.y, v1.z + v2.z]
impl std::ops::Add<&Vector3u64> for Vector3u64 {
    type Output = Vector3u64;

    fn add(self, rhs: &Vector3u64) -> Self::Output {
        self.add(*rhs)
    }
}

/// Components wise addition
/// v1 + v2 = [v1.x + v2.x, v1.y + v2.y, v1.z + v2.z]
impl std::ops::Add<Vector3u64> for &Vector3u64 {
    type Output = Vector3u64;

    fn add(self, rhs: Vector3u64) -> Self::Output {
        (*self).add(rhs)
    }
}

/// Components wise addition
/// v1 + v2 = [v1.x + v2.x, v1.y + v2.y, v1.z + v2.z]
impl std::ops::Add<&Vector3u64> for &Vector3u64 {
    type Output = Vector3u64;

    fn add(self, rhs: &Vector3u64) -> Self::Output {
        (*self).add(*rhs)
    }
}

/// Adds `rhs` to all components of the vector
/// v + f = [v.x + f, v.y + f, v.z + f]
impl std::ops::Add<u64> for Vector3u64 {
    type Output = Vector3u64;

    fn add(self, rhs: u64) -> Self::Output {
        Vector3u64 {
            data: self.data + u64x4::from_array([rhs, rhs, rhs, 0]),
        }
    }
}

/// Adds `rhs` to all components of the vector
/// v + f = [v.x + f, v.y + f, v.z + f]
impl std::ops::Add<u64> for &Vector3u64 {
    type Output = Vector3u64;

    fn add(self, rhs: u64) -> Self::Output {
        (*self).add(rhs)
    }
}

/// Adds `rhs` to all components of the vector
/// v + f = [v.x + f, v.y + f, v.z + f]
impl std::ops::Add<&u64> for Vector3u64 {
    type Output = Vector3u64;

    fn add(self, rhs: &u64) -> Self::Output {
        self.add(*rhs)
    }
}

/// Adds `rhs` to all components of the vector
/// v + f = [v.x + f, v.y + f, v.z + f]
impl std::ops::Add<&u64> for &Vector3u64 {
    type Output = Vector3u64;

    fn add(self, rhs: &u64) -> Self::Output {
        (*self).add(*rhs)
    }
}

//////////////////////////////////////////////////////////
//////////////   vector substractions    /////////////////
//////////////////////////////////////////////////////////

/// Components wise substraction
/// v1 - v2 = [v1.x - v2.x, v1.y - v2.y, v1.z - v2.z]
impl std::ops::Sub<Vector3u64> for Vector3u64 {
    type Output = Vector3u64;

    fn sub(self, rhs: Vector3u64) -> Self::Output {
        Vector3u64 {
            data: self.data - rhs.data,
        }
    }
}

/// Components wise substraction
/// v1 - v2 = [v1.x - v2.x, v1.y - v2.y, v1.z - v2.z]
impl std::ops::Sub<&Vector3u64> for Vector3u64 {
    type Output = Vector3u64;

    fn sub(self, rhs: &Vector3u64) -> Self::Output {
        self.sub(*rhs)
    }
}

/// Components wise substraction
/// v1 - v2 = [v1.x - v2.x, v1.y - v2.y, v1.z - v2.z]
impl std::ops::Sub<Vector3u64> for &Vector3u64 {
    type Output = Vector3u64;

    fn sub(self, rhs: Vector3u64) -> Self::Output {
        (*self).sub(rhs)
    }
}

/// Components wise substraction
/// v1 - v2 = [v1.x - v2.x, v1.y - v2.y, v1.z - v2.z]
impl std::ops::Sub<&Vector3u64> for &Vector3u64 {
    type Output = Vector3u64;

    fn sub(self, rhs: &Vector3u64) -> Self::Output {
        (*self).sub(*rhs)
    }
}

/// Substract `rhs` to all components of the vector
/// v - f = [v.x - f, v.y - f, v.z - f]
impl std::ops::Sub<u64> for Vector3u64 {
    type Output = Vector3u64;

    fn sub(self, rhs: u64) -> Self::Output {
        Vector3u64 {
            data: self.data - u64x4::from_array([rhs, rhs, rhs, 0]),
        }
    }
}

/// Substracts `rhs` to all components of the vector
/// v - f = [v.x - f, v.y - f, v.z - f]
impl std::ops::Sub<u64> for &Vector3u64 {
    type Output = Vector3u64;

    fn sub(self, rhs: u64) -> Self::Output {
        (*self).sub(rhs)
    }
}

/// Substracts `rhs` to all components of the vector
/// v - f = [v.x - f, v.y - f, v.z - f]
impl std::ops::Sub<&u64> for Vector3u64 {
    type Output = Vector3u64;

    fn sub(self, rhs: &u64) -> Self::Output {
        self.sub(*rhs)
    }
}

/// Substracts `rhs` to all components of the vector
/// v - f = [v.x - f, v.y - f, v.z - f]
impl std::ops::Sub<&u64> for &Vector3u64 {
    type Output = Vector3u64;

    fn sub(self, rhs: &u64) -> Self::Output {
        (*self).sub(*rhs)
    }
}

//////////////////////////////////////////////////////////
//////////////   vector multiplications   ////////////////
//////////////////////////////////////////////////////////

/// Components wise multiplication
/// v1 * v2 = [v1.x * v2.x, v1.y * v2.y, v1.z * v2.z]
impl std::ops::Mul<Vector3u64> for Vector3u64 {
    type Output = Vector3u64;

    fn mul(self, rhs: Vector3u64) -> Self::Output {
        Vector3u64 {
            data: self.data * rhs.data,
        }
    }
}

/// Components wise multiplication
/// v1 * v2 = [v1.x * v2.x, v1.y * v2.y, v1.z * v2.z]
impl std::ops::Mul<&Vector3u64> for Vector3u64 {
    type Output = Vector3u64;

    fn mul(self, rhs: &Vector3u64) -> Self::Output {
        self.mul(*rhs)
    }
}

/// Components wise multiplication
/// v1 * v2 = [v1.x * v2.x, v1.y * v2.y, v1.z * v2.z]
impl std::ops::Mul<Vector3u64> for &Vector3u64 {
    type Output = Vector3u64;

    fn mul(self, rhs: Vector3u64) -> Self::Output {
        (*self).mul(rhs)
    }
}

/// Components wise multiplication
/// v1 * v2 = [v1.x * v2.x, v1.y * v2.y, v1.z * v2.z]
impl std::ops::Mul<&Vector3u64> for &Vector3u64 {
    type Output = Vector3u64;

    fn mul(self, rhs: &Vector3u64) -> Self::Output {
        (*self).mul(*rhs)
    }
}

/// Multiplies all components of the vector by `rhs`
/// v * f = [v.x * f, v.y * f, v.z * f]
impl std::ops::Mul<u64> for Vector3u64 {
    type Output = Vector3u64;

    fn mul(self, rhs: u64) -> Self::Output {
        Vector3u64 {
            data: self.data * u64x4::from_array([rhs, rhs, rhs, 1]),
        }
    }
}

/// Multiplies all components of the vector by `rhs`
/// v * f = [v.x * f, v.y * f, v.z * f]
impl std::ops::Mul<u64> for &Vector3u64 {
    type Output = Vector3u64;

    fn mul(self, rhs: u64) -> Self::Output {
        (*self).mul(rhs)
    }
}

/// Multiplies all components of the vector by `rhs`
/// v * f = [v.x * f, v.y * f, v.z * f]
impl std::ops::Mul<&u64> for Vector3u64 {
    type Output = Vector3u64;

    fn mul(self, rhs: &u64) -> Self::Output {
        self.mul(*rhs)
    }
}

/// Multiplies all components of the vector by `rhs`
/// v * f = [v.x * f, v.y * f, v.z * f]
impl std::ops::Mul<&u64> for &Vector3u64 {
    type Output = Vector3u64;

    fn mul(self, rhs: &u64) -> Self::Output {
        (*self).mul(*rhs)
    }
}

//////////////////////////////////////////////////////////
//////////////     vector divisions     //////////////////
//////////////////////////////////////////////////////////

/// Components wise division
/// v1 / v2 = [v1.x / v2.x, v1.y / v2.y, v1.z / v2.z]
impl std::ops::Div<Vector3u64> for Vector3u64 {
    type Output = Vector3u64;

    fn div(self, rhs: Vector3u64) -> Self::Output {
        Vector3u64 {
            data: self.data / rhs.data,
        }
    }
}

/// Components wise division
/// v1 / v2 = [v1.x / v2.x, v1.y / v2.y, v1.z / v2.z]
impl std::ops::Div<&Vector3u64> for Vector3u64 {
    type Output = Vector3u64;

    fn div(self, rhs: &Vector3u64) -> Self::Output {
        self.div(*rhs)
    }
}

/// Components wise division
/// v1 / v2 = [v1.x / v2.x, v1.y / v2.y, v1.z / v2.z]
impl std::ops::Div<Vector3u64> for &Vector3u64 {
    type Output = Vector3u64;

    fn div(self, rhs: Vector3u64) -> Self::Output {
        (*self).div(rhs)
    }
}

/// Components wise division
/// v1 / v2 = [v1.x / v2.x, v1.y / v2.y, v1.z / v2.z]
impl std::ops::Div<&Vector3u64> for &Vector3u64 {
    type Output = Vector3u64;

    fn div(self, rhs: &Vector3u64) -> Self::Output {
        (*self).div(*rhs)
    }
}

/// Divides all components of the vector by `rhs`
/// v / f = [v.x / f, v.y / f, v.z / f]
impl std::ops::Div<u64> for Vector3u64 {
    type Output = Vector3u64;

    fn div(self, rhs: u64) -> Self::Output {
        Vector3u64 {
            data: self.data / u64x4::from_array([rhs, rhs, rhs, 1]),
        }
    }
}

/// Divides all components of the vector by `rhs`
/// v / f = [v.x / f, v.y / f, v.z / f]
impl std::ops::Div<u64> for &Vector3u64 {
    type Output = Vector3u64;

    fn div(self, rhs: u64) -> Self::Output {
        (*self).div(rhs)
    }
}

/// Divides all components of the vector by `rhs`
/// v / f = [v.x / f, v.y / f, v.z / f]
impl std::ops::Div<&u64> for Vector3u64 {
    type Output = Vector3u64;

    fn div(self, rhs: &u64) -> Self::Output {
        self.div(*rhs)
    }
}

/// Divides all components of the vector by `rhs`
/// v / f = [v.x / f, v.y / f, v.z / f]
impl std::ops::Div<&u64> for &Vector3u64 {
    type Output = Vector3u64;

    fn div(self, rhs: &u64) -> Self::Output {
        (*self).div(*rhs)
    }
}

//////////////////////////////////////////////////////////
//////////////      vector assign       //////////////////
//////////////////////////////////////////////////////////

/// Components wise addition
/// v1 += v2 <=> v1 = [v1.x + v2.x, v1.y + v2.y, v1.z + v2.z]
impl std::ops::AddAssign<Vector3u64> for Vector3u64 {
    fn add_assign(&mut self, rhs: Vector3u64) {
        self.data = self.data + rhs.data;
    }
}

/// Components wise addition
/// v1 += v2 <=> v1 = [v1.x + v2.x, v1.y + v2.y, v1.z + v2.z]
impl std::ops::AddAssign<&Vector3u64> for Vector3u64 {
    fn add_assign(&mut self, rhs: &Vector3u64) {
        self.add_assign(*rhs);
    }
}

/// Adds `rhs` to all components of the vector
/// v += f <=> v = [v.x + f, v.y + f]
impl std::ops::AddAssign<u64> for Vector3u64 {
    fn add_assign(&mut self, rhs: u64) {
        self.data = self.data + u64x4::from_array([rhs, rhs, rhs, 0]);
    }
}

/// Adds `rhs` to all components of the vector
/// v += f <=> v = [v.x + f, v.y + f]
impl std::ops::AddAssign<&u64> for Vector3u64 {
    fn add_assign(&mut self, rhs: &u64) {
        self.add_assign(*rhs);
    }
}

/// Components wise substraction
/// v1 -= v2 <=> v1 = [v1.x - v2.x, v1.y - v2.y, v1.z - v2.z]
impl std::ops::SubAssign<Vector3u64> for Vector3u64 {
    fn sub_assign(&mut self, rhs: Vector3u64) {
        self.data = self.data - rhs.data;
    }
}

/// Components wise substraction
/// v1 -= v2 <=> v1 = [v1.x - v2.x, v1.y - v2.y, v1.z - v2.z]
impl std::ops::SubAssign<&Vector3u64> for Vector3u64 {
    fn sub_assign(&mut self, rhs: &Vector3u64) {
        self.sub_assign(*rhs);
    }
}

/// Substracts `rhs` to all components of the vector
/// v -= f <=> v = [v.x - f, v.y - f, v.z - f]
impl std::ops::SubAssign<u64> for Vector3u64 {
    fn sub_assign(&mut self, rhs: u64) {
        self.data = self.data - u64x4::from_array([rhs, rhs, rhs, 0]);
    }
}

/// Substracts `rhs` to all components of the vector
/// v -= f <=> v = [v.x - f, v.y - f, v.z - f]
impl std::ops::SubAssign<&u64> for Vector3u64 {
    fn sub_assign(&mut self, rhs: &u64) {
        self.sub_assign(*rhs);
    }
}

/// Components wise multiplication
/// v1 *= v2 <=> v1 = [v1.x * v2.x, v1.y * v2.y, v1.z * v2.z]
impl std::ops::MulAssign<Vector3u64> for Vector3u64 {
    fn mul_assign(&mut self, rhs: Vector3u64) {
        self.data = self.data * rhs.data;
    }
}

/// Components wise multiplication
/// v1 *= v2 <=> v1 = [v1.x * v2.x, v1.y * v2.y, v1.z * v2.z]
impl std::ops::MulAssign<&Vector3u64> for Vector3u64 {
    fn mul_assign(&mut self, rhs: &Vector3u64) {
        self.mul_assign(*rhs);
    }
}

/// Multiplies all components of the vector by `rhs`
/// v *= f <=> v = [v.x * f, v.y * f, v.z * f]
impl std::ops::MulAssign<u64> for Vector3u64 {
    fn mul_assign(&mut self, rhs: u64) {
        self.data = self.data * u64x4::from_array([rhs, rhs, rhs, 1]);
    }
}

/// Multiplies all components of the vector by `rhs`
/// v *= f <=> v = [v.x * f, v.y * f, v.z * f]
impl std::ops::MulAssign<&u64> for Vector3u64 {
    fn mul_assign(&mut self, rhs: &u64) {
        self.mul_assign(*rhs);
    }
}

/// Components wise division
/// v1 /= v2 <=> v1 = [v1.x / v2.x, v1.y / v2.y, v1.z / v2.z]
impl std::ops::DivAssign<Vector3u64> for Vector3u64 {
    fn div_assign(&mut self, rhs: Vector3u64) {
        self.data = self.data / rhs.data;
    }
}

/// Components wise division
/// v1 /= v2 <=> v1 = [v1.x / v2.x, v1.y / v2.y, v1.z / v2.z]
impl std::ops::DivAssign<&Vector3u64> for Vector3u64 {
    fn div_assign(&mut self, rhs: &Vector3u64) {
        self.div_assign(*rhs);
    }
}

/// Divides all components of the vector by `rhs`
/// v /= f <=> v1 = [v.x / f, v.y / f, v.z / f]
impl std::ops::DivAssign<u64> for Vector3u64 {
    fn div_assign(&mut self, rhs: u64) {
        self.data = self.data / u64x4::from_array([rhs, rhs, rhs, 1]);
    }
}

/// Divides all components of the vector by `rhs`
/// v /= f <=> v = [v.x / f, v.y / f, v.z / f]
impl std::ops::DivAssign<&u64> for Vector3u64 {
    fn div_assign(&mut self, rhs: &u64) {
        self.div_assign(*rhs);
    }
}

//////////////////////////////////////////////////////////
///////////////     vector tests      ////////////////////
//////////////////////////////////////////////////////////
#[cfg(test)]
mod tests {
    use super::*;

    /// Tests 3 dimensional vector initialization
    #[test]
    fn initialization() {
        let v1 = vec3u64(1, 1, 1);
        let v2 = Vector3u64::ONES;
        assert_eq!(v1, v2);
    }

    /// Tests 3 dimensional vector operators
    #[test]
    fn operators() {
        let v1 = vec3u64(2, 3, 1);
        let v2 = vec3u64(4, 5, 6);
        assert_eq!(v1 + v2, vec3u64(6, 8, 7));
        let mut v3 = v1;
        assert_eq!(v1, v3);
        v3 += v1;
        assert_eq!(v3, vec3u64(4, 6, 2));

        v3 += 1;
        assert_eq!(v3, vec3u64(5, 7, 3));
        assert_eq!(v3 + v3 - v3, v3);

        assert_eq!(v3 * 2, vec3u64(10, 14, 6));
        assert_eq!(v1 * v2, vec3u64(8, 15, 6));

        assert_eq!(v3 / v3, Vector3u64::ONES);
        v3 /= v3;
        assert_eq!(v3, Vector3u64::ONES);
    }

    /// Tests panic outputs
    #[should_panic]
    #[test]
    fn invalid_cross_product() {
        let v1 = vec3u64(1, 2, 4);
        let v2 = vec3u64(4, 3, 5);
        let _ = Vector3u64::cross(&v1, &v2);
    }

    /// Tests 3 dimensional vector operations
    #[test]
    fn operations() {
        let v1 = vec3u64(1, 2, 4);
        let v2 = vec3u64(4, 3, 5);
        assert_eq!(Vector3u64::dot(&v1, &v2), 30);

        let vx = Vector3u64::X;
        let vy = Vector3u64::Y;
        let vz = Vector3u64::Z;
        assert_eq!(Vector3u64::dot(&vx, &vy), 0);

        assert_eq!(vx, vec3u64(1, 0, 0));
        assert_eq!(vy, vec3u64(0, 1, 0));
        assert_eq!(vz, vec3u64(0, 0, 1));

        assert_eq!(Vector3u64::cross(&vx, &vy), vz);
        assert_eq!(Vector3u64::cross(&vy, &vz), vx);
        assert_eq!(Vector3u64::cross(&vz, &vx), vy);

        assert_eq!(vx.length(), 1.);
        assert_eq!(vy.length(), 1.);
        assert_eq!(vz.length(), 1.);
        assert_eq!(v1.length(), 21f32.sqrt());
    }

    /// Tests 3 dimensional vector's fields getters and setters
    #[test]
    fn deref() {
        let v1 = vec3u64(5, 2, 1);
        assert_eq!(v1.x, 5);
        assert_eq!(v1.y, 2);
        assert_eq!(v1.z, 1);
        let mut v2 = Vector3u64::ZEROS;
        assert_eq!(v2.x, 0);
        assert_eq!(v2.y, 0);
        assert_eq!(v2.z, 0);
        v2.x = 1;
        assert_eq!(v2.x, 1);
        v2.y = 3;
        assert_eq!(v2.y, 3);
        assert_eq!(v2.x, 1);
        v2.z = 4;
        assert_eq!(v2.y, 3);
        assert_eq!(v2.x, 1);
        assert_eq!(v2.z, 4);
    }

    /// Tests the formatting of 3 dimensional vectors
    #[test]
    fn format() {
        let v1 = vec3u64(3, 4, 6);
        assert_eq!(v1.to_string(), "(3, 4, 6)");
        assert_eq!(format!("{:?}", v1), "Vector3u64 { x: 3, y: 4, z: 6 }");
    }
}
