use std::simd::prelude::*;

/// A structure to represent a 3 dimensional i16 vector
#[derive(Clone, Copy)]
#[repr(C)]
pub struct Vector3i16 {
    data: i16x4, // can't do simd on line of 3
}

/// A structure to be able to create `.x`, `.y`, and `.z` getters and setters
mod private {
    #[repr(C)]
    pub struct CoordsVector3 {
        pub x: i16,
        pub y: i16,
        pub z: i16,
        _pad: i16, // can't do simd on line of 3
    }
}

/// Implements `Deref` to allow accessing `.x`, `.y`, and `.z`
impl std::ops::Deref for Vector3i16 {
    type Target = private::CoordsVector3;

    fn deref(&self) -> &Self::Target {
        let value: *const Vector3i16 = self;
        unsafe { &*(value as *const private::CoordsVector3) }
    }
}

/// Implements `DerefMut` to allow modifying `.x`, `.y`, and `.z`
impl std::ops::DerefMut for Vector3i16 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        let value: *mut Vector3i16 = self;
        unsafe { &mut *(value as *mut private::CoordsVector3) }
    }
}

/// Overrides the debug trait
impl std::fmt::Debug for Vector3i16 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Vector3i16")
            .field("x", &self.x)
            .field("y", &self.y)
            .field("z", &self.z)
            .finish()
    }
}

/// Overrides the display trait
impl std::fmt::Display for Vector3i16 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

/// Sets a 3 dimensional i16 vector to `[0, 0, 0]` as default
impl Default for Vector3i16 {
    fn default() -> Self {
        Self::ZEROS
    }
}

/// Creates a 3 dimensional i16 vector
pub fn vec3i16(x: i16, y: i16, z: i16) -> Vector3i16 {
    Vector3i16::new(x, y, z)
}

/// A union to cast simd to array and allow const construction
union UnionCast {
    array: [i16; 4],
    simd: Vector3i16,
}

impl Vector3i16 {
    //////////////////////////////////////////////////////////
    /////////////      vector creation       /////////////////
    //////////////////////////////////////////////////////////

    /// Creates a new vector given its coordinates
    pub const fn new(x: i16, y: i16, z: i16) -> Self {
        unsafe {
            UnionCast {
                array: [x, y, z, 1],
            }
            .simd
        }
    }

    /// Creates a new vector with all coordinates set to `value`
    const fn splat(value: i16) -> Self {
        Self::new(value, value, value)
    }

    /// Creates a new vector filled with `values`
    pub const fn filled(value: i16) -> Self {
        Self::splat(value)
    }

    /// Creates a new vector filled with ones
    pub const ONES: Self = Self::splat(1);

    /// Creates a new vector filled with negative ones
    pub const NEG_ONES: Self = Self::splat(-1);

    /// Creates a new vector filled with zeros
    pub const ZEROS: Self = Self::splat(0);

    /// Creates a new vector filled with `i16::MIN``
    pub const MIN: Self = Self::splat(i16::MIN);

    /// Creates a new vector filled with `i16::MAX`
    pub const MAX: Self = Self::splat(i16::MAX);

    /// Creates a new vector pointing along the positive X axis
    pub const X: Self = Self::new(1, 0, 0);

    /// Creates a new vector pointing along the positive Y axis
    pub const Y: Self = Self::new(0, 1, 0);

    /// Creates a new vector pointing along the positive Z axis
    pub const Z: Self = Self::new(0, 0, 1);

    /// Creates a new vector pointing along the negative X axis
    pub const NEG_X: Self = Self::new(-1, 0, 0);

    /// Creates a new vector pointing along the negative Y axis
    pub const NEG_Y: Self = Self::new(0, -1, 0);

    /// Creates a new vector pointing along the negative Z axis
    pub const NEG_Z: Self = Self::new(0, 0, -1);

    //////////////////////////////////////////////////////////
    /////////////     vector operations      /////////////////
    //////////////////////////////////////////////////////////

    /// Sums up the element of the vector
    pub fn prefix_sum(self) -> i16 {
        self.x + self.y + self.z
    }

    /// Dot product between two vectors
    pub fn dot(v1: &Vector3i16, v2: &Vector3i16) -> i16 {
        (v1 * v2).prefix_sum()
    }

    /// Cross product between two vectors
    pub fn cross(v1: &Vector3i16, v2: &Vector3i16) -> Vector3i16 {
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

impl PartialEq for Vector3i16 {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}

//////////////////////////////////////////////////////////
//////////////     vector additions     //////////////////
//////////////////////////////////////////////////////////

/// Components wise addition
/// v1 + v2 = [v1.x + v2.x, v1.y + v2.y, v1.z + v2.z]
impl std::ops::Add<Vector3i16> for Vector3i16 {
    type Output = Vector3i16;

    fn add(self, rhs: Vector3i16) -> Self::Output {
        Vector3i16 {
            data: self.data + rhs.data,
        }
    }
}

/// Components wise addition
/// v1 + v2 = [v1.x + v2.x, v1.y + v2.y, v1.z + v2.z]
impl std::ops::Add<&Vector3i16> for Vector3i16 {
    type Output = Vector3i16;

    fn add(self, rhs: &Vector3i16) -> Self::Output {
        self.add(*rhs)
    }
}

/// Components wise addition
/// v1 + v2 = [v1.x + v2.x, v1.y + v2.y, v1.z + v2.z]
impl std::ops::Add<Vector3i16> for &Vector3i16 {
    type Output = Vector3i16;

    fn add(self, rhs: Vector3i16) -> Self::Output {
        (*self).add(rhs)
    }
}

/// Components wise addition
/// v1 + v2 = [v1.x + v2.x, v1.y + v2.y, v1.z + v2.z]
impl std::ops::Add<&Vector3i16> for &Vector3i16 {
    type Output = Vector3i16;

    fn add(self, rhs: &Vector3i16) -> Self::Output {
        (*self).add(*rhs)
    }
}

/// Adds `rhs` to all components of the vector
/// v + f = [v.x + f, v.y + f, v.z + f]
impl std::ops::Add<i16> for Vector3i16 {
    type Output = Vector3i16;

    fn add(self, rhs: i16) -> Self::Output {
        Vector3i16 {
            data: self.data + i16x4::from_array([rhs, rhs, rhs, 0]),
        }
    }
}

/// Adds `rhs` to all components of the vector
/// v + f = [v.x + f, v.y + f, v.z + f]
impl std::ops::Add<i16> for &Vector3i16 {
    type Output = Vector3i16;

    fn add(self, rhs: i16) -> Self::Output {
        (*self).add(rhs)
    }
}

/// Adds `rhs` to all components of the vector
/// v + f = [v.x + f, v.y + f, v.z + f]
impl std::ops::Add<&i16> for Vector3i16 {
    type Output = Vector3i16;

    fn add(self, rhs: &i16) -> Self::Output {
        self.add(*rhs)
    }
}

/// Adds `rhs` to all components of the vector
/// v + f = [v.x + f, v.y + f, v.z + f]
impl std::ops::Add<&i16> for &Vector3i16 {
    type Output = Vector3i16;

    fn add(self, rhs: &i16) -> Self::Output {
        (*self).add(*rhs)
    }
}

//////////////////////////////////////////////////////////
//////////////   vector substractions    /////////////////
//////////////////////////////////////////////////////////

/// Components wise negation
/// -v = [-v.x, -v.y, -v.z]
impl std::ops::Neg for Vector3i16 {
    type Output = Vector3i16;

    fn neg(self) -> Self::Output {
        Vector3i16 { data: -self.data }
    }
}

/// Components wise substraction
/// v1 - v2 = [v1.x - v2.x, v1.y - v2.y, v1.z - v2.z]
impl std::ops::Sub<Vector3i16> for Vector3i16 {
    type Output = Vector3i16;

    fn sub(self, rhs: Vector3i16) -> Self::Output {
        Vector3i16 {
            data: self.data - rhs.data,
        }
    }
}

/// Components wise substraction
/// v1 - v2 = [v1.x - v2.x, v1.y - v2.y, v1.z - v2.z]
impl std::ops::Sub<&Vector3i16> for Vector3i16 {
    type Output = Vector3i16;

    fn sub(self, rhs: &Vector3i16) -> Self::Output {
        self.sub(*rhs)
    }
}

/// Components wise substraction
/// v1 - v2 = [v1.x - v2.x, v1.y - v2.y, v1.z - v2.z]
impl std::ops::Sub<Vector3i16> for &Vector3i16 {
    type Output = Vector3i16;

    fn sub(self, rhs: Vector3i16) -> Self::Output {
        (*self).sub(rhs)
    }
}

/// Components wise substraction
/// v1 - v2 = [v1.x - v2.x, v1.y - v2.y, v1.z - v2.z]
impl std::ops::Sub<&Vector3i16> for &Vector3i16 {
    type Output = Vector3i16;

    fn sub(self, rhs: &Vector3i16) -> Self::Output {
        (*self).sub(*rhs)
    }
}

/// Substract `rhs` to all components of the vector
/// v - f = [v.x - f, v.y - f, v.z - f]
impl std::ops::Sub<i16> for Vector3i16 {
    type Output = Vector3i16;

    fn sub(self, rhs: i16) -> Self::Output {
        Vector3i16 {
            data: self.data - i16x4::from_array([rhs, rhs, rhs, 0]),
        }
    }
}

/// Substracts `rhs` to all components of the vector
/// v - f = [v.x - f, v.y - f, v.z - f]
impl std::ops::Sub<i16> for &Vector3i16 {
    type Output = Vector3i16;

    fn sub(self, rhs: i16) -> Self::Output {
        (*self).sub(rhs)
    }
}

/// Substracts `rhs` to all components of the vector
/// v - f = [v.x - f, v.y - f, v.z - f]
impl std::ops::Sub<&i16> for Vector3i16 {
    type Output = Vector3i16;

    fn sub(self, rhs: &i16) -> Self::Output {
        self.sub(*rhs)
    }
}

/// Substracts `rhs` to all components of the vector
/// v - f = [v.x - f, v.y - f, v.z - f]
impl std::ops::Sub<&i16> for &Vector3i16 {
    type Output = Vector3i16;

    fn sub(self, rhs: &i16) -> Self::Output {
        (*self).sub(*rhs)
    }
}

//////////////////////////////////////////////////////////
//////////////   vector multiplications   ////////////////
//////////////////////////////////////////////////////////

/// Components wise multiplication
/// v1 * v2 = [v1.x * v2.x, v1.y * v2.y, v1.z * v2.z]
impl std::ops::Mul<Vector3i16> for Vector3i16 {
    type Output = Vector3i16;

    fn mul(self, rhs: Vector3i16) -> Self::Output {
        Vector3i16 {
            data: self.data * rhs.data,
        }
    }
}

/// Components wise multiplication
/// v1 * v2 = [v1.x * v2.x, v1.y * v2.y, v1.z * v2.z]
impl std::ops::Mul<&Vector3i16> for Vector3i16 {
    type Output = Vector3i16;

    fn mul(self, rhs: &Vector3i16) -> Self::Output {
        self.mul(*rhs)
    }
}

/// Components wise multiplication
/// v1 * v2 = [v1.x * v2.x, v1.y * v2.y, v1.z * v2.z]
impl std::ops::Mul<Vector3i16> for &Vector3i16 {
    type Output = Vector3i16;

    fn mul(self, rhs: Vector3i16) -> Self::Output {
        (*self).mul(rhs)
    }
}

/// Components wise multiplication
/// v1 * v2 = [v1.x * v2.x, v1.y * v2.y, v1.z * v2.z]
impl std::ops::Mul<&Vector3i16> for &Vector3i16 {
    type Output = Vector3i16;

    fn mul(self, rhs: &Vector3i16) -> Self::Output {
        (*self).mul(*rhs)
    }
}

/// Multiplies all components of the vector by `rhs`
/// v * f = [v.x * f, v.y * f, v.z * f]
impl std::ops::Mul<i16> for Vector3i16 {
    type Output = Vector3i16;

    fn mul(self, rhs: i16) -> Self::Output {
        Vector3i16 {
            data: self.data * i16x4::from_array([rhs, rhs, rhs, 1]),
        }
    }
}

/// Multiplies all components of the vector by `rhs`
/// v * f = [v.x * f, v.y * f, v.z * f]
impl std::ops::Mul<i16> for &Vector3i16 {
    type Output = Vector3i16;

    fn mul(self, rhs: i16) -> Self::Output {
        (*self).mul(rhs)
    }
}

/// Multiplies all components of the vector by `rhs`
/// v * f = [v.x * f, v.y * f, v.z * f]
impl std::ops::Mul<&i16> for Vector3i16 {
    type Output = Vector3i16;

    fn mul(self, rhs: &i16) -> Self::Output {
        self.mul(*rhs)
    }
}

/// Multiplies all components of the vector by `rhs`
/// v * f = [v.x * f, v.y * f, v.z * f]
impl std::ops::Mul<&i16> for &Vector3i16 {
    type Output = Vector3i16;

    fn mul(self, rhs: &i16) -> Self::Output {
        (*self).mul(*rhs)
    }
}

//////////////////////////////////////////////////////////
//////////////     vector divisions     //////////////////
//////////////////////////////////////////////////////////

/// Components wise division
/// v1 / v2 = [v1.x / v2.x, v1.y / v2.y, v1.z / v2.z]
impl std::ops::Div<Vector3i16> for Vector3i16 {
    type Output = Vector3i16;

    fn div(self, rhs: Vector3i16) -> Self::Output {
        Vector3i16 {
            data: self.data / rhs.data,
        }
    }
}

/// Components wise division
/// v1 / v2 = [v1.x / v2.x, v1.y / v2.y, v1.z / v2.z]
impl std::ops::Div<&Vector3i16> for Vector3i16 {
    type Output = Vector3i16;

    fn div(self, rhs: &Vector3i16) -> Self::Output {
        self.div(*rhs)
    }
}

/// Components wise division
/// v1 / v2 = [v1.x / v2.x, v1.y / v2.y, v1.z / v2.z]
impl std::ops::Div<Vector3i16> for &Vector3i16 {
    type Output = Vector3i16;

    fn div(self, rhs: Vector3i16) -> Self::Output {
        (*self).div(rhs)
    }
}

/// Components wise division
/// v1 / v2 = [v1.x / v2.x, v1.y / v2.y, v1.z / v2.z]
impl std::ops::Div<&Vector3i16> for &Vector3i16 {
    type Output = Vector3i16;

    fn div(self, rhs: &Vector3i16) -> Self::Output {
        (*self).div(*rhs)
    }
}

/// Divides all components of the vector by `rhs`
/// v / f = [v.x / f, v.y / f, v.z / f]
impl std::ops::Div<i16> for Vector3i16 {
    type Output = Vector3i16;

    fn div(self, rhs: i16) -> Self::Output {
        Vector3i16 {
            data: self.data / i16x4::from_array([rhs, rhs, rhs, 1]),
        }
    }
}

/// Divides all components of the vector by `rhs`
/// v / f = [v.x / f, v.y / f, v.z / f]
impl std::ops::Div<i16> for &Vector3i16 {
    type Output = Vector3i16;

    fn div(self, rhs: i16) -> Self::Output {
        (*self).div(rhs)
    }
}

/// Divides all components of the vector by `rhs`
/// v / f = [v.x / f, v.y / f, v.z / f]
impl std::ops::Div<&i16> for Vector3i16 {
    type Output = Vector3i16;

    fn div(self, rhs: &i16) -> Self::Output {
        self.div(*rhs)
    }
}

/// Divides all components of the vector by `rhs`
/// v / f = [v.x / f, v.y / f, v.z / f]
impl std::ops::Div<&i16> for &Vector3i16 {
    type Output = Vector3i16;

    fn div(self, rhs: &i16) -> Self::Output {
        (*self).div(*rhs)
    }
}

//////////////////////////////////////////////////////////
//////////////      vector assign       //////////////////
//////////////////////////////////////////////////////////

/// Components wise addition
/// v1 += v2 <=> v1 = [v1.x + v2.x, v1.y + v2.y, v1.z + v2.z]
impl std::ops::AddAssign<Vector3i16> for Vector3i16 {
    fn add_assign(&mut self, rhs: Vector3i16) {
        self.data = self.data + rhs.data;
    }
}

/// Components wise addition
/// v1 += v2 <=> v1 = [v1.x + v2.x, v1.y + v2.y, v1.z + v2.z]
impl std::ops::AddAssign<&Vector3i16> for Vector3i16 {
    fn add_assign(&mut self, rhs: &Vector3i16) {
        self.add_assign(*rhs);
    }
}

/// Adds `rhs` to all components of the vector
/// v += f <=> v = [v.x + f, v.y + f]
impl std::ops::AddAssign<i16> for Vector3i16 {
    fn add_assign(&mut self, rhs: i16) {
        self.data = self.data + i16x4::from_array([rhs, rhs, rhs, 0]);
    }
}

/// Adds `rhs` to all components of the vector
/// v += f <=> v = [v.x + f, v.y + f]
impl std::ops::AddAssign<&i16> for Vector3i16 {
    fn add_assign(&mut self, rhs: &i16) {
        self.add_assign(*rhs);
    }
}

/// Components wise substraction
/// v1 -= v2 <=> v1 = [v1.x - v2.x, v1.y - v2.y, v1.z - v2.z]
impl std::ops::SubAssign<Vector3i16> for Vector3i16 {
    fn sub_assign(&mut self, rhs: Vector3i16) {
        self.data = self.data - rhs.data;
    }
}

/// Components wise substraction
/// v1 -= v2 <=> v1 = [v1.x - v2.x, v1.y - v2.y, v1.z - v2.z]
impl std::ops::SubAssign<&Vector3i16> for Vector3i16 {
    fn sub_assign(&mut self, rhs: &Vector3i16) {
        self.sub_assign(*rhs);
    }
}

/// Substracts `rhs` to all components of the vector
/// v -= f <=> v = [v.x - f, v.y - f, v.z - f]
impl std::ops::SubAssign<i16> for Vector3i16 {
    fn sub_assign(&mut self, rhs: i16) {
        self.data = self.data - i16x4::from_array([rhs, rhs, rhs, 0]);
    }
}

/// Substracts `rhs` to all components of the vector
/// v -= f <=> v = [v.x - f, v.y - f, v.z - f]
impl std::ops::SubAssign<&i16> for Vector3i16 {
    fn sub_assign(&mut self, rhs: &i16) {
        self.sub_assign(*rhs);
    }
}

/// Components wise multiplication
/// v1 *= v2 <=> v1 = [v1.x * v2.x, v1.y * v2.y, v1.z * v2.z]
impl std::ops::MulAssign<Vector3i16> for Vector3i16 {
    fn mul_assign(&mut self, rhs: Vector3i16) {
        self.data = self.data * rhs.data;
    }
}

/// Components wise multiplication
/// v1 *= v2 <=> v1 = [v1.x * v2.x, v1.y * v2.y, v1.z * v2.z]
impl std::ops::MulAssign<&Vector3i16> for Vector3i16 {
    fn mul_assign(&mut self, rhs: &Vector3i16) {
        self.mul_assign(*rhs);
    }
}

/// Multiplies all components of the vector by `rhs`
/// v *= f <=> v = [v.x * f, v.y * f, v.z * f]
impl std::ops::MulAssign<i16> for Vector3i16 {
    fn mul_assign(&mut self, rhs: i16) {
        self.data = self.data * i16x4::from_array([rhs, rhs, rhs, 1]);
    }
}

/// Multiplies all components of the vector by `rhs`
/// v *= f <=> v = [v.x * f, v.y * f, v.z * f]
impl std::ops::MulAssign<&i16> for Vector3i16 {
    fn mul_assign(&mut self, rhs: &i16) {
        self.mul_assign(*rhs);
    }
}

/// Components wise division
/// v1 /= v2 <=> v1 = [v1.x / v2.x, v1.y / v2.y, v1.z / v2.z]
impl std::ops::DivAssign<Vector3i16> for Vector3i16 {
    fn div_assign(&mut self, rhs: Vector3i16) {
        self.data = self.data / rhs.data;
    }
}

/// Components wise division
/// v1 /= v2 <=> v1 = [v1.x / v2.x, v1.y / v2.y, v1.z / v2.z]
impl std::ops::DivAssign<&Vector3i16> for Vector3i16 {
    fn div_assign(&mut self, rhs: &Vector3i16) {
        self.div_assign(*rhs);
    }
}

/// Divides all components of the vector by `rhs`
/// v /= f <=> v1 = [v.x / f, v.y / f, v.z / f]
impl std::ops::DivAssign<i16> for Vector3i16 {
    fn div_assign(&mut self, rhs: i16) {
        self.data = self.data / i16x4::from_array([rhs, rhs, rhs, 1]);
    }
}

/// Divides all components of the vector by `rhs`
/// v /= f <=> v = [v.x / f, v.y / f, v.z / f]
impl std::ops::DivAssign<&i16> for Vector3i16 {
    fn div_assign(&mut self, rhs: &i16) {
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
        let v1 = vec3i16(1, 1, 1);
        let v2 = Vector3i16::ONES;
        assert_eq!(v1, v2);
    }

    /// Tests 3 dimensional vector operators
    #[test]
    fn operators() {
        let v1 = vec3i16(2, 3, 1);
        let v2 = vec3i16(4, 5, 6);
        assert_eq!(v1 + v2, vec3i16(6, 8, 7));
        let mut v3 = v1;
        assert_eq!(v1, v3);
        v3 += v1;
        assert_eq!(v3, vec3i16(4, 6, 2));

        v3 += 1;
        assert_eq!(v3, vec3i16(5, 7, 3));
        assert_eq!(v3 + v3 - v3, v3);

        assert_eq!(v3 * 2, vec3i16(10, 14, 6));
        assert_eq!(v1 * v2, vec3i16(8, 15, 6));

        assert_eq!(v3 / v3, Vector3i16::ONES);
        v3 /= v3;
        assert_eq!(v3, Vector3i16::ONES);
    }

    /// Tests 3 dimensional vector operations
    #[test]
    fn operations() {
        let v1 = vec3i16(1, 2, 4);
        let v2 = vec3i16(4, 3, 5);
        assert_eq!(Vector3i16::dot(&v1, &v2), 30);
        assert_eq!(Vector3i16::cross(&v1, &v2), vec3i16(-2, 11, -5));
        assert_eq!(Vector3i16::cross(&v2, &v1), -Vector3i16::cross(&v1, &v2));

        let vx = Vector3i16::X;
        let vy = Vector3i16::Y;
        let vz = Vector3i16::Z;
        assert_eq!(Vector3i16::dot(&vx, &vy), 0);

        assert_eq!(vx, vec3i16(1, 0, 0));
        assert_eq!(vy, vec3i16(0, 1, 0));
        assert_eq!(vz, vec3i16(0, 0, 1));

        assert_eq!(Vector3i16::cross(&vx, &vy), vz);
        assert_eq!(Vector3i16::cross(&vy, &vx), Vector3i16::NEG_Z);

        assert_eq!(Vector3i16::cross(&vy, &vz), vx);
        assert_eq!(Vector3i16::cross(&vz, &vy), Vector3i16::NEG_X);

        assert_eq!(Vector3i16::cross(&vz, &vx), vy);
        assert_eq!(Vector3i16::cross(&vx, &vz), Vector3i16::NEG_Y);

        assert_eq!(vx.length(), 1.);
        assert_eq!(vy.length(), 1.);
        assert_eq!(vz.length(), 1.);
        assert_eq!(v1.length(), 21f32.sqrt());
    }

    /// Tests 3 dimensional vector's fields getters and setters
    #[test]
    fn deref() {
        let v1 = vec3i16(5, 2, 1);
        assert_eq!(v1.x, 5);
        assert_eq!(v1.y, 2);
        assert_eq!(v1.z, 1);
        let mut v2 = Vector3i16::ZEROS;
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
        let v1 = vec3i16(3, 4, 6);
        assert_eq!(v1.to_string(), "(3, 4, 6)");
        assert_eq!(format!("{:?}", v1), "Vector3i16 { x: 3, y: 4, z: 6 }");
    }
}
