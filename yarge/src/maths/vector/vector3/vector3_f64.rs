use std::simd::prelude::*;

/// A structure to represent a 3 dimensional f64 vector
#[derive(Clone, Copy)]
#[repr(C)]
pub struct Vector3f64 {
    data: f64x4, // can't do simd on line of 3
}

/// A structure to be able to create `.x`, `.y`, and `.z` getters and setters
mod private {
    #[repr(C)]
    pub struct CoordsVector3 {
        pub x: f64,
        pub y: f64,
        pub z: f64,
        _pad: f64, // can't do simd on line of 3
    }
}

/// Implements `Deref` to allow accessing `.x`, `.y`, and `.z`
impl std::ops::Deref for Vector3f64 {
    type Target = private::CoordsVector3;

    fn deref(&self) -> &Self::Target {
        let value: *const Vector3f64 = self;
        unsafe { &*(value as *const private::CoordsVector3) }
    }
}

/// Implements `DerefMut` to allow modifying `.x`, `.y`, and `.z`
impl std::ops::DerefMut for Vector3f64 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        let value: *mut Vector3f64 = self;
        unsafe { &mut *(value as *mut private::CoordsVector3) }
    }
}

/// Overrides the debug trait
impl std::fmt::Debug for Vector3f64 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Vector3f64")
            .field("x", &self.x)
            .field("y", &self.y)
            .field("z", &self.z)
            .finish()
    }
}

/// Overrides the display trait
impl std::fmt::Display for Vector3f64 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

/// Sets a 3 dimensional f64 vector to `[0., 0., 0.]` as default
impl Default for Vector3f64 {
    fn default() -> Self {
        Self::ZEROS
    }
}

/// Creates a 3 dimensional f64 vector
pub fn vec3f64(x: f64, y: f64, z: f64) -> Vector3f64 {
    Vector3f64::new(x, y, z)
}

/// A union to cast simd to array and allow const construction
union UnionCast {
    array: [f64; 4],
    simd: Vector3f64,
}

impl Vector3f64 {
    //////////////////////////////////////////////////////////
    /////////////      vector creation       /////////////////
    //////////////////////////////////////////////////////////

    /// Creates a new vector given its coordinates
    pub const fn new(x: f64, y: f64, z: f64) -> Self {
        unsafe {
            UnionCast {
                array: [x, y, z, 1.],
            }
            .simd
        }
    }

    /// Creates a new vector with all coordinates set to `value`
    const fn splat(value: f64) -> Self {
        Self::new(value, value, value)
    }

    /// Creates a new vector filled with `values`
    pub const fn filled(value: f64) -> Self {
        Self::splat(value)
    }

    /// Creates a new vector filled with ones
    pub const ONES: Self = Self::splat(1.);

    /// Creates a new vector filled with negative ones
    pub const NEG_ONES: Self = Self::splat(-1.);

    /// Creates a new vector filled with zeros
    pub const ZEROS: Self = Self::splat(0.);

    /// Creates a new vector filled with `f64::INFINITY`
    pub const INFINITY: Self = Self::splat(f64::INFINITY);

    /// Creates a new vector filled with `f64::NEG_INFINITY`
    pub const NEG_INFINITY: Self = Self::splat(f64::NEG_INFINITY);

    /// Creates a new vector filled with `f64::MIN``
    pub const MIN: Self = Self::splat(f64::MIN);

    /// Creates a new vector filled with `f64::MAX`
    pub const MAX: Self = Self::splat(f64::MAX);

    /// Creates a new vector pointing along the positive X axis
    pub const X: Self = Self::new(1., 0., 0.);

    /// Creates a new vector pointing along the positive Y axis
    pub const Y: Self = Self::new(0., 1., 0.);

    /// Creates a new vector pointing along the positive Z axis
    pub const Z: Self = Self::new(0., 0., 1.);

    /// Creates a new vector pointing along the negative X axis
    pub const NEG_X: Self = Self::new(-1., 0., 0.);

    /// Creates a new vector pointing along the negative Y axis
    pub const NEG_Y: Self = Self::new(0., -1., 0.);

    /// Creates a new vector pointing along the negative Z axis
    pub const NEG_Z: Self = Self::new(0., 0., -1.);

    //////////////////////////////////////////////////////////
    /////////////     vector operations      /////////////////
    //////////////////////////////////////////////////////////

    /// Sums up the element of the vector
    pub fn prefix_sum(self) -> f64 {
        self.x + self.y + self.z
    }

    /// Dot product between two vectors
    pub fn dot(v1: &Vector3f64, v2: &Vector3f64) -> f64 {
        (v1 * v2).prefix_sum()
    }

    /// Cross product between two vectors
    pub fn cross(v1: &Vector3f64, v2: &Vector3f64) -> Vector3f64 {
        Self::new(
            v1.y * v2.z - v1.z * v2.y,
            v1.z * v2.x - v1.x * v2.z,
            v1.x * v2.y - v1.y * v2.x,
        )
    }

    /// Returns the length of the vector
    pub fn length(&self) -> f64 {
        Self::dot(self, self).sqrt()
    }
}

impl PartialEq for Vector3f64 {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}

//////////////////////////////////////////////////////////
//////////////     vector additions     //////////////////
//////////////////////////////////////////////////////////

/// Components wise addition
/// v1 + v2 = [v1.x + v2.x, v1.y + v2.y, v1.z + v2.z]
impl std::ops::Add<Vector3f64> for Vector3f64 {
    type Output = Vector3f64;

    fn add(self, rhs: Vector3f64) -> Self::Output {
        Vector3f64 {
            data: self.data + rhs.data,
        }
    }
}

/// Components wise addition
/// v1 + v2 = [v1.x + v2.x, v1.y + v2.y, v1.z + v2.z]
impl std::ops::Add<&Vector3f64> for Vector3f64 {
    type Output = Vector3f64;

    fn add(self, rhs: &Vector3f64) -> Self::Output {
        self.add(*rhs)
    }
}

/// Components wise addition
/// v1 + v2 = [v1.x + v2.x, v1.y + v2.y, v1.z + v2.z]
impl std::ops::Add<Vector3f64> for &Vector3f64 {
    type Output = Vector3f64;

    fn add(self, rhs: Vector3f64) -> Self::Output {
        (*self).add(rhs)
    }
}

/// Components wise addition
/// v1 + v2 = [v1.x + v2.x, v1.y + v2.y, v1.z + v2.z]
impl std::ops::Add<&Vector3f64> for &Vector3f64 {
    type Output = Vector3f64;

    fn add(self, rhs: &Vector3f64) -> Self::Output {
        (*self).add(*rhs)
    }
}

/// Adds `rhs` to all components of the vector
/// v + f = [v.x + f, v.y + f, v.z + f]
impl std::ops::Add<f64> for Vector3f64 {
    type Output = Vector3f64;

    fn add(self, rhs: f64) -> Self::Output {
        Vector3f64 {
            data: self.data + f64x4::from_array([rhs, rhs, rhs, 0.]),
        }
    }
}

/// Adds `rhs` to all components of the vector
/// v + f = [v.x + f, v.y + f, v.z + f]
impl std::ops::Add<f64> for &Vector3f64 {
    type Output = Vector3f64;

    fn add(self, rhs: f64) -> Self::Output {
        (*self).add(rhs)
    }
}

/// Adds `rhs` to all components of the vector
/// v + f = [v.x + f, v.y + f, v.z + f]
impl std::ops::Add<&f64> for Vector3f64 {
    type Output = Vector3f64;

    fn add(self, rhs: &f64) -> Self::Output {
        self.add(*rhs)
    }
}

/// Adds `rhs` to all components of the vector
/// v + f = [v.x + f, v.y + f, v.z + f]
impl std::ops::Add<&f64> for &Vector3f64 {
    type Output = Vector3f64;

    fn add(self, rhs: &f64) -> Self::Output {
        (*self).add(*rhs)
    }
}

//////////////////////////////////////////////////////////
//////////////   vector substractions    /////////////////
//////////////////////////////////////////////////////////

/// Components wise negation
/// -v = [-v.x, -v.y, -v.z]
impl std::ops::Neg for Vector3f64 {
    type Output = Vector3f64;

    fn neg(self) -> Self::Output {
        Vector3f64 { data: -self.data }
    }
}

/// Components wise substraction
/// v1 - v2 = [v1.x - v2.x, v1.y - v2.y, v1.z - v2.z]
impl std::ops::Sub<Vector3f64> for Vector3f64 {
    type Output = Vector3f64;

    fn sub(self, rhs: Vector3f64) -> Self::Output {
        Vector3f64 {
            data: self.data - rhs.data,
        }
    }
}

/// Components wise substraction
/// v1 - v2 = [v1.x - v2.x, v1.y - v2.y, v1.z - v2.z]
impl std::ops::Sub<&Vector3f64> for Vector3f64 {
    type Output = Vector3f64;

    fn sub(self, rhs: &Vector3f64) -> Self::Output {
        self.sub(*rhs)
    }
}

/// Components wise substraction
/// v1 - v2 = [v1.x - v2.x, v1.y - v2.y, v1.z - v2.z]
impl std::ops::Sub<Vector3f64> for &Vector3f64 {
    type Output = Vector3f64;

    fn sub(self, rhs: Vector3f64) -> Self::Output {
        (*self).sub(rhs)
    }
}

/// Components wise substraction
/// v1 - v2 = [v1.x - v2.x, v1.y - v2.y, v1.z - v2.z]
impl std::ops::Sub<&Vector3f64> for &Vector3f64 {
    type Output = Vector3f64;

    fn sub(self, rhs: &Vector3f64) -> Self::Output {
        (*self).sub(*rhs)
    }
}

/// Substract `rhs` to all components of the vector
/// v - f = [v.x - f, v.y - f, v.z - f]
impl std::ops::Sub<f64> for Vector3f64 {
    type Output = Vector3f64;

    fn sub(self, rhs: f64) -> Self::Output {
        Vector3f64 {
            data: self.data - f64x4::from_array([rhs, rhs, rhs, 0.]),
        }
    }
}

/// Substracts `rhs` to all components of the vector
/// v - f = [v.x - f, v.y - f, v.z - f]
impl std::ops::Sub<f64> for &Vector3f64 {
    type Output = Vector3f64;

    fn sub(self, rhs: f64) -> Self::Output {
        (*self).sub(rhs)
    }
}

/// Substracts `rhs` to all components of the vector
/// v - f = [v.x - f, v.y - f, v.z - f]
impl std::ops::Sub<&f64> for Vector3f64 {
    type Output = Vector3f64;

    fn sub(self, rhs: &f64) -> Self::Output {
        self.sub(*rhs)
    }
}

/// Substracts `rhs` to all components of the vector
/// v - f = [v.x - f, v.y - f, v.z - f]
impl std::ops::Sub<&f64> for &Vector3f64 {
    type Output = Vector3f64;

    fn sub(self, rhs: &f64) -> Self::Output {
        (*self).sub(*rhs)
    }
}

//////////////////////////////////////////////////////////
//////////////   vector multiplications   ////////////////
//////////////////////////////////////////////////////////

/// Components wise multiplication
/// v1 * v2 = [v1.x * v2.x, v1.y * v2.y, v1.z * v2.z]
impl std::ops::Mul<Vector3f64> for Vector3f64 {
    type Output = Vector3f64;

    fn mul(self, rhs: Vector3f64) -> Self::Output {
        Vector3f64 {
            data: self.data * rhs.data,
        }
    }
}

/// Components wise multiplication
/// v1 * v2 = [v1.x * v2.x, v1.y * v2.y, v1.z * v2.z]
impl std::ops::Mul<&Vector3f64> for Vector3f64 {
    type Output = Vector3f64;

    fn mul(self, rhs: &Vector3f64) -> Self::Output {
        self.mul(*rhs)
    }
}

/// Components wise multiplication
/// v1 * v2 = [v1.x * v2.x, v1.y * v2.y, v1.z * v2.z]
impl std::ops::Mul<Vector3f64> for &Vector3f64 {
    type Output = Vector3f64;

    fn mul(self, rhs: Vector3f64) -> Self::Output {
        (*self).mul(rhs)
    }
}

/// Components wise multiplication
/// v1 * v2 = [v1.x * v2.x, v1.y * v2.y, v1.z * v2.z]
impl std::ops::Mul<&Vector3f64> for &Vector3f64 {
    type Output = Vector3f64;

    fn mul(self, rhs: &Vector3f64) -> Self::Output {
        (*self).mul(*rhs)
    }
}

/// Multiplies all components of the vector by `rhs`
/// v * f = [v.x * f, v.y * f, v.z * f]
impl std::ops::Mul<f64> for Vector3f64 {
    type Output = Vector3f64;

    fn mul(self, rhs: f64) -> Self::Output {
        Vector3f64 {
            data: self.data * f64x4::from_array([rhs, rhs, rhs, 1.]),
        }
    }
}

/// Multiplies all components of the vector by `rhs`
/// v * f = [v.x * f, v.y * f, v.z * f]
impl std::ops::Mul<f64> for &Vector3f64 {
    type Output = Vector3f64;

    fn mul(self, rhs: f64) -> Self::Output {
        (*self).mul(rhs)
    }
}

/// Multiplies all components of the vector by `rhs`
/// v * f = [v.x * f, v.y * f, v.z * f]
impl std::ops::Mul<&f64> for Vector3f64 {
    type Output = Vector3f64;

    fn mul(self, rhs: &f64) -> Self::Output {
        self.mul(*rhs)
    }
}

/// Multiplies all components of the vector by `rhs`
/// v * f = [v.x * f, v.y * f, v.z * f]
impl std::ops::Mul<&f64> for &Vector3f64 {
    type Output = Vector3f64;

    fn mul(self, rhs: &f64) -> Self::Output {
        (*self).mul(*rhs)
    }
}

//////////////////////////////////////////////////////////
//////////////     vector divisions     //////////////////
//////////////////////////////////////////////////////////

/// Components wise division
/// v1 / v2 = [v1.x / v2.x, v1.y / v2.y, v1.z / v2.z]
impl std::ops::Div<Vector3f64> for Vector3f64 {
    type Output = Vector3f64;

    fn div(self, rhs: Vector3f64) -> Self::Output {
        Vector3f64 {
            data: self.data / rhs.data,
        }
    }
}

/// Components wise division
/// v1 / v2 = [v1.x / v2.x, v1.y / v2.y, v1.z / v2.z]
impl std::ops::Div<&Vector3f64> for Vector3f64 {
    type Output = Vector3f64;

    fn div(self, rhs: &Vector3f64) -> Self::Output {
        self.div(*rhs)
    }
}

/// Components wise division
/// v1 / v2 = [v1.x / v2.x, v1.y / v2.y, v1.z / v2.z]
impl std::ops::Div<Vector3f64> for &Vector3f64 {
    type Output = Vector3f64;

    fn div(self, rhs: Vector3f64) -> Self::Output {
        (*self).div(rhs)
    }
}

/// Components wise division
/// v1 / v2 = [v1.x / v2.x, v1.y / v2.y, v1.z / v2.z]
impl std::ops::Div<&Vector3f64> for &Vector3f64 {
    type Output = Vector3f64;

    fn div(self, rhs: &Vector3f64) -> Self::Output {
        (*self).div(*rhs)
    }
}

/// Divides all components of the vector by `rhs`
/// v / f = [v.x / f, v.y / f, v.z / f]
impl std::ops::Div<f64> for Vector3f64 {
    type Output = Vector3f64;

    fn div(self, rhs: f64) -> Self::Output {
        Vector3f64 {
            data: self.data / f64x4::from_array([rhs, rhs, rhs, 1.]),
        }
    }
}

/// Divides all components of the vector by `rhs`
/// v / f = [v.x / f, v.y / f, v.z / f]
impl std::ops::Div<f64> for &Vector3f64 {
    type Output = Vector3f64;

    fn div(self, rhs: f64) -> Self::Output {
        (*self).div(rhs)
    }
}

/// Divides all components of the vector by `rhs`
/// v / f = [v.x / f, v.y / f, v.z / f]
impl std::ops::Div<&f64> for Vector3f64 {
    type Output = Vector3f64;

    fn div(self, rhs: &f64) -> Self::Output {
        self.div(*rhs)
    }
}

/// Divides all components of the vector by `rhs`
/// v / f = [v.x / f, v.y / f, v.z / f]
impl std::ops::Div<&f64> for &Vector3f64 {
    type Output = Vector3f64;

    fn div(self, rhs: &f64) -> Self::Output {
        (*self).div(*rhs)
    }
}

//////////////////////////////////////////////////////////
//////////////      vector assign       //////////////////
//////////////////////////////////////////////////////////

/// Components wise addition
/// v1 += v2 <=> v1 = [v1.x + v2.x, v1.y + v2.y, v1.z + v2.z]
impl std::ops::AddAssign<Vector3f64> for Vector3f64 {
    fn add_assign(&mut self, rhs: Vector3f64) {
        self.data = self.data + rhs.data;
    }
}

/// Components wise addition
/// v1 += v2 <=> v1 = [v1.x + v2.x, v1.y + v2.y, v1.z + v2.z]
impl std::ops::AddAssign<&Vector3f64> for Vector3f64 {
    fn add_assign(&mut self, rhs: &Vector3f64) {
        self.add_assign(*rhs);
    }
}

/// Adds `rhs` to all components of the vector
/// v += f <=> v = [v.x + f, v.y + f]
impl std::ops::AddAssign<f64> for Vector3f64 {
    fn add_assign(&mut self, rhs: f64) {
        self.data = self.data + f64x4::from_array([rhs, rhs, rhs, 0.]);
    }
}

/// Adds `rhs` to all components of the vector
/// v += f <=> v = [v.x + f, v.y + f]
impl std::ops::AddAssign<&f64> for Vector3f64 {
    fn add_assign(&mut self, rhs: &f64) {
        self.add_assign(*rhs);
    }
}

/// Components wise substraction
/// v1 -= v2 <=> v1 = [v1.x - v2.x, v1.y - v2.y, v1.z - v2.z]
impl std::ops::SubAssign<Vector3f64> for Vector3f64 {
    fn sub_assign(&mut self, rhs: Vector3f64) {
        self.data = self.data - rhs.data;
    }
}

/// Components wise substraction
/// v1 -= v2 <=> v1 = [v1.x - v2.x, v1.y - v2.y, v1.z - v2.z]
impl std::ops::SubAssign<&Vector3f64> for Vector3f64 {
    fn sub_assign(&mut self, rhs: &Vector3f64) {
        self.sub_assign(*rhs);
    }
}

/// Substracts `rhs` to all components of the vector
/// v -= f <=> v = [v.x - f, v.y - f, v.z - f]
impl std::ops::SubAssign<f64> for Vector3f64 {
    fn sub_assign(&mut self, rhs: f64) {
        self.data = self.data - f64x4::from_array([rhs, rhs, rhs, 0.]);
    }
}

/// Substracts `rhs` to all components of the vector
/// v -= f <=> v = [v.x - f, v.y - f, v.z - f]
impl std::ops::SubAssign<&f64> for Vector3f64 {
    fn sub_assign(&mut self, rhs: &f64) {
        self.sub_assign(*rhs);
    }
}

/// Components wise multiplication
/// v1 *= v2 <=> v1 = [v1.x * v2.x, v1.y * v2.y, v1.z * v2.z]
impl std::ops::MulAssign<Vector3f64> for Vector3f64 {
    fn mul_assign(&mut self, rhs: Vector3f64) {
        self.data = self.data * rhs.data;
    }
}

/// Components wise multiplication
/// v1 *= v2 <=> v1 = [v1.x * v2.x, v1.y * v2.y, v1.z * v2.z]
impl std::ops::MulAssign<&Vector3f64> for Vector3f64 {
    fn mul_assign(&mut self, rhs: &Vector3f64) {
        self.mul_assign(*rhs);
    }
}

/// Multiplies all components of the vector by `rhs`
/// v *= f <=> v = [v.x * f, v.y * f, v.z * f]
impl std::ops::MulAssign<f64> for Vector3f64 {
    fn mul_assign(&mut self, rhs: f64) {
        self.data = self.data * f64x4::from_array([rhs, rhs, rhs, 1.]);
    }
}

/// Multiplies all components of the vector by `rhs`
/// v *= f <=> v = [v.x * f, v.y * f, v.z * f]
impl std::ops::MulAssign<&f64> for Vector3f64 {
    fn mul_assign(&mut self, rhs: &f64) {
        self.mul_assign(*rhs);
    }
}

/// Components wise division
/// v1 /= v2 <=> v1 = [v1.x / v2.x, v1.y / v2.y, v1.z / v2.z]
impl std::ops::DivAssign<Vector3f64> for Vector3f64 {
    fn div_assign(&mut self, rhs: Vector3f64) {
        self.data = self.data / rhs.data;
    }
}

/// Components wise division
/// v1 /= v2 <=> v1 = [v1.x / v2.x, v1.y / v2.y, v1.z / v2.z]
impl std::ops::DivAssign<&Vector3f64> for Vector3f64 {
    fn div_assign(&mut self, rhs: &Vector3f64) {
        self.div_assign(*rhs);
    }
}

/// Divides all components of the vector by `rhs`
/// v /= f <=> v1 = [v.x / f, v.y / f, v.z / f]
impl std::ops::DivAssign<f64> for Vector3f64 {
    fn div_assign(&mut self, rhs: f64) {
        self.data = self.data / f64x4::from_array([rhs, rhs, rhs, 1.]);
    }
}

/// Divides all components of the vector by `rhs`
/// v /= f <=> v = [v.x / f, v.y / f, v.z / f]
impl std::ops::DivAssign<&f64> for Vector3f64 {
    fn div_assign(&mut self, rhs: &f64) {
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
        let v1 = vec3f64(1., 1., 1.);
        let v2 = Vector3f64::ONES;
        assert_eq!(v1, v2);
    }

    /// Tests 3 dimensional vector operators
    #[test]
    fn operators() {
        let v1 = vec3f64(2., 3., 1.);
        let v2 = vec3f64(4., 5., 6.);
        assert_eq!(v1 + v2, vec3f64(6., 8., 7.));
        let mut v3 = v1;
        assert_eq!(v1, v3);
        v3 += v1;
        assert_eq!(v3, vec3f64(4., 6., 2.));

        v3 += 1.;
        assert_eq!(v3, vec3f64(5., 7., 3.));
        assert_eq!(v3 + v3 - v3, v3);

        assert_eq!(v3 * 2., vec3f64(10., 14., 6.));
        assert_eq!(v1 * v2, vec3f64(8., 15., 6.));

        assert_eq!(v3 / v3, Vector3f64::ONES);
        v3 /= v3;
        assert_eq!(v3, Vector3f64::ONES);
    }

    /// Tests 3 dimensional vector operations
    #[test]
    fn operations() {
        let v1 = vec3f64(1., 2., 4.);
        let v2 = vec3f64(4., 3., 5.);
        assert_eq!(Vector3f64::dot(&v1, &v2), 30.);
        assert_eq!(Vector3f64::cross(&v1, &v2), vec3f64(-2., 11., -5.));
        assert_eq!(Vector3f64::cross(&v2, &v1), -Vector3f64::cross(&v1, &v2));

        let vx = Vector3f64::X;
        let vy = Vector3f64::Y;
        let vz = Vector3f64::Z;
        assert_eq!(Vector3f64::dot(&vx, &vy), 0.);

        assert_eq!(vx, vec3f64(1., 0., 0.));
        assert_eq!(vy, vec3f64(0., 1., 0.));
        assert_eq!(vz, vec3f64(0., 0., 1.));

        assert_eq!(Vector3f64::cross(&vx, &vy), vz);
        assert_eq!(Vector3f64::cross(&vy, &vx), Vector3f64::NEG_Z);

        assert_eq!(Vector3f64::cross(&vy, &vz), vx);
        assert_eq!(Vector3f64::cross(&vz, &vy), Vector3f64::NEG_X);

        assert_eq!(Vector3f64::cross(&vz, &vx), vy);
        assert_eq!(Vector3f64::cross(&vx, &vz), Vector3f64::NEG_Y);

        assert_eq!(vx.length(), 1.);
        assert_eq!(vy.length(), 1.);
        assert_eq!(vz.length(), 1.);
        assert_eq!(v1.length(), 21f64.sqrt());
    }

    /// Tests 3 dimensional vector's fields getters and setters
    #[test]
    fn deref() {
        let v1 = vec3f64(5., 2., 1.);
        assert_eq!(v1.x, 5.);
        assert_eq!(v1.y, 2.);
        assert_eq!(v1.z, 1.);
        let mut v2 = Vector3f64::ZEROS;
        assert_eq!(v2.x, 0.);
        assert_eq!(v2.y, 0.);
        assert_eq!(v2.z, 0.);
        v2.x = 1.;
        assert_eq!(v2.x, 1.);
        v2.y = 3.;
        assert_eq!(v2.y, 3.);
        assert_eq!(v2.x, 1.);
        v2.z = 4.;
        assert_eq!(v2.y, 3.);
        assert_eq!(v2.x, 1.);
        assert_eq!(v2.z, 4.);
    }

    /// Tests the formatting of 3 dimensional vectors
    #[test]
    fn format() {
        let v1 = vec3f64(3.1, 4.2, 6.9);
        assert_eq!(v1.to_string(), "(3.1, 4.2, 6.9)");
        assert_eq!(format!("{:?}", v1), "Vector3f64 { x: 3.1, y: 4.2, z: 6.9 }");
    }
}
