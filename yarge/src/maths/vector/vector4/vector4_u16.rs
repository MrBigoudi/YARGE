use std::simd::prelude::*;

/// A structure to represent a 4 dimensional u16 vector
#[derive(Clone, Copy, PartialEq)]
#[repr(C)]
pub struct Vector4u16 {
    data: u16x4,
}

/// A structure to be able to create `.x`, `.y`, `.z` and `.w` getters and setters
mod private {
    #[repr(C)]
    pub struct CoordsVector4 {
        pub x: u16,
        pub y: u16,
        pub z: u16,
        pub w: u16,
    }
}

/// Implements `Deref` to allow accessing `.x`, `.y`, `.z` and `.w`
impl std::ops::Deref for Vector4u16 {
    type Target = private::CoordsVector4;

    fn deref(&self) -> &Self::Target {
        let value: *const Vector4u16 = self;
        unsafe { &*(value as *const private::CoordsVector4) }
    }
}

/// Implements `DerefMut` to allow modifying `.x`, `.y`, `.z` and `.w`
impl std::ops::DerefMut for Vector4u16 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        let value: *mut Vector4u16 = self;
        unsafe { &mut *(value as *mut private::CoordsVector4) }
    }
}

/// Overrides the debug trait
impl std::fmt::Debug for Vector4u16 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Vector4u16")
            .field("x", &self.x)
            .field("y", &self.y)
            .field("z", &self.z)
            .field("w", &self.w)
            .finish()
    }
}

/// Overrides the display trait
impl std::fmt::Display for Vector4u16 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {}, {})", self.x, self.y, self.z, self.w)
    }
}

/// Sets a 4 dimensional u16 vector to `[0, 0, 0, 0]` as default
impl Default for Vector4u16 {
    fn default() -> Self {
        Self::ZEROS
    }
}

/// Creates a 4 dimensional u16 vector
pub fn vec4u16(x: u16, y: u16, z: u16, w: u16) -> Vector4u16 {
    Vector4u16::new(x, y, z, w)
}

/// A union to cast simd to array and allow const construction
union UnionCast {
    array: [u16; 4],
    simd: Vector4u16,
}

impl Vector4u16 {
    //////////////////////////////////////////////////////////
    /////////////      vector creation       /////////////////
    //////////////////////////////////////////////////////////

    /// Creates a new vector given its coordinates
    pub const fn new(x: u16, y: u16, z: u16, w: u16) -> Self {
        unsafe {
            UnionCast {
                array: [x, y, z, w],
            }
            .simd
        }
    }

    /// Creates a new vector with all coordinates set to `value`
    const fn splat(value: u16) -> Self {
        Self::new(value, value, value, value)
    }

    /// Creates a new vector filled with `values`
    pub const fn filled(value: u16) -> Self {
        Self::splat(value)
    }

    /// Creates a new vector filled with ones
    pub const ONES: Self = Self::splat(1);

    /// Creates a new vector filled with zeros
    pub const ZEROS: Self = Self::splat(0);

    /// Creates a new vector filled with `u16::MIN``
    pub const MIN: Self = Self::splat(u16::MIN);

    /// Creates a new vector filled with `u16::MAX`
    pub const MAX: Self = Self::splat(u16::MAX);

    /// Creates a new vector pointing along the positive X axis
    pub const X: Self = Self::new(1, 0, 0, 0);

    /// Creates a new vector pointing along the positive Y axis
    pub const Y: Self = Self::new(0, 1, 0, 0);

    /// Creates a new vector pointing along the positive Z axis
    pub const Z: Self = Self::new(0, 0, 1, 0);

    /// Creates a new vector pointing along the positive W axis
    pub const W: Self = Self::new(0, 0, 0, 1);

    //////////////////////////////////////////////////////////
    /////////////     vector operations      /////////////////
    //////////////////////////////////////////////////////////

    /// Sums up the element of the vector
    pub fn prefix_sum(self) -> u16 {
        self.x + self.y + self.z + self.w
    }

    /// Dot product between two vectors
    pub fn dot(v1: &Vector4u16, v2: &Vector4u16) -> u16 {
        (v1 * v2).prefix_sum()
    }

    /// Returns the length of the vector
    pub fn length(&self) -> f32 {
        (Self::dot(self, self) as f32).sqrt()
    }
}

//////////////////////////////////////////////////////////
//////////////     vector additions     //////////////////
//////////////////////////////////////////////////////////

/// Components wise addition
/// v1 + v2 = [v1.x + v2.x, v1.y + v2.y, v1.z + v2.z, v1.w + v2.w]
impl std::ops::Add<Vector4u16> for Vector4u16 {
    type Output = Vector4u16;

    fn add(self, rhs: Vector4u16) -> Self::Output {
        Vector4u16 {
            data: self.data + rhs.data,
        }
    }
}

/// Components wise addition
/// v1 + v2 = [v1.x + v2.x, v1.y + v2.y, v1.z + v2.z, v1.w + v2.w]
impl std::ops::Add<&Vector4u16> for Vector4u16 {
    type Output = Vector4u16;

    fn add(self, rhs: &Vector4u16) -> Self::Output {
        self.add(*rhs)
    }
}

/// Components wise addition
/// v1 + v2 = [v1.x + v2.x, v1.y + v2.y, v1.z + v2.z, v1.w + v2.w]
impl std::ops::Add<Vector4u16> for &Vector4u16 {
    type Output = Vector4u16;

    fn add(self, rhs: Vector4u16) -> Self::Output {
        (*self).add(rhs)
    }
}

/// Components wise addition
/// v1 + v2 = [v1.x + v2.x, v1.y + v2.y, v1.z + v2.z, v1.w + v2.w]
impl std::ops::Add<&Vector4u16> for &Vector4u16 {
    type Output = Vector4u16;

    fn add(self, rhs: &Vector4u16) -> Self::Output {
        (*self).add(*rhs)
    }
}

/// Adds `rhs` to all components of the vector
/// v + f = [v.x + f, v.y + f, v.z + f, v.w + f]
impl std::ops::Add<u16> for Vector4u16 {
    type Output = Vector4u16;

    fn add(self, rhs: u16) -> Self::Output {
        Vector4u16 {
            data: self.data + u16x4::from_array([rhs, rhs, rhs, rhs]),
        }
    }
}

/// Adds `rhs` to all components of the vector
/// v + f = [v.x + f, v.y + f, v.z + f, v.w + f]
impl std::ops::Add<u16> for &Vector4u16 {
    type Output = Vector4u16;

    fn add(self, rhs: u16) -> Self::Output {
        (*self).add(rhs)
    }
}

/// Adds `rhs` to all components of the vector
/// v + f = [v.x + f, v.y + f, v.z + f, v.w + f]
impl std::ops::Add<&u16> for Vector4u16 {
    type Output = Vector4u16;

    fn add(self, rhs: &u16) -> Self::Output {
        self.add(*rhs)
    }
}

/// Adds `rhs` to all components of the vector
/// v + f = [v.x + f, v.y + f, v.z + f, v.w + f]
impl std::ops::Add<&u16> for &Vector4u16 {
    type Output = Vector4u16;

    fn add(self, rhs: &u16) -> Self::Output {
        (*self).add(*rhs)
    }
}

//////////////////////////////////////////////////////////
//////////////   vector substractions    /////////////////
//////////////////////////////////////////////////////////

/// Components wise substraction
/// v1 - v2 = [v1.x - v2.x, v1.y - v2.y, v1.z - v2.z, v1.w - v2.w]
impl std::ops::Sub<Vector4u16> for Vector4u16 {
    type Output = Vector4u16;

    fn sub(self, rhs: Vector4u16) -> Self::Output {
        Vector4u16 {
            data: self.data - rhs.data,
        }
    }
}

/// Components wise substraction
/// v1 - v2 = [v1.x - v2.x, v1.y - v2.y, v1.z - v2.z, v1.w - v2.w]
impl std::ops::Sub<&Vector4u16> for Vector4u16 {
    type Output = Vector4u16;

    fn sub(self, rhs: &Vector4u16) -> Self::Output {
        self.sub(*rhs)
    }
}

/// Components wise substraction
/// v1 - v2 = [v1.x - v2.x, v1.y - v2.y, v1.z - v2.z, v1.w - v2.w]
impl std::ops::Sub<Vector4u16> for &Vector4u16 {
    type Output = Vector4u16;

    fn sub(self, rhs: Vector4u16) -> Self::Output {
        (*self).sub(rhs)
    }
}

/// Components wise substraction
/// v1 - v2 = [v1.x - v2.x, v1.y - v2.y, v1.z - v2.z, v1.w - v2.w]
impl std::ops::Sub<&Vector4u16> for &Vector4u16 {
    type Output = Vector4u16;

    fn sub(self, rhs: &Vector4u16) -> Self::Output {
        (*self).sub(*rhs)
    }
}

/// Substract `rhs` to all components of the vector
/// v - f = [v.x - f, v.y - f, v.z - f, v.w - f]
impl std::ops::Sub<u16> for Vector4u16 {
    type Output = Vector4u16;

    fn sub(self, rhs: u16) -> Self::Output {
        Vector4u16 {
            data: self.data - u16x4::from_array([rhs, rhs, rhs, rhs]),
        }
    }
}

/// Substracts `rhs` to all components of the vector
/// v - f = [v.x - f, v.y - f, v.z - f, v.w - f]
impl std::ops::Sub<u16> for &Vector4u16 {
    type Output = Vector4u16;

    fn sub(self, rhs: u16) -> Self::Output {
        (*self).sub(rhs)
    }
}

/// Substracts `rhs` to all components of the vector
/// v - f = [v.x - f, v.y - f, v.z - f, v.w - f]
impl std::ops::Sub<&u16> for Vector4u16 {
    type Output = Vector4u16;

    fn sub(self, rhs: &u16) -> Self::Output {
        self.sub(*rhs)
    }
}

/// Substracts `rhs` to all components of the vector
/// v - f = [v.x - f, v.y - f, v.z - f, v.w - f]
impl std::ops::Sub<&u16> for &Vector4u16 {
    type Output = Vector4u16;

    fn sub(self, rhs: &u16) -> Self::Output {
        (*self).sub(*rhs)
    }
}

//////////////////////////////////////////////////////////
//////////////   vector multiplications   ////////////////
//////////////////////////////////////////////////////////

/// Components wise multiplication
/// v1 * v2 = [v1.x * v2.x, v1.y * v2.y, v1.z * v2.z, v1.w * v2.w]
impl std::ops::Mul<Vector4u16> for Vector4u16 {
    type Output = Vector4u16;

    fn mul(self, rhs: Vector4u16) -> Self::Output {
        Vector4u16 {
            data: self.data * rhs.data,
        }
    }
}

/// Components wise multiplication
/// v1 * v2 = [v1.x * v2.x, v1.y * v2.y, v1.z * v2.z, v1.w * v2.w]
impl std::ops::Mul<&Vector4u16> for Vector4u16 {
    type Output = Vector4u16;

    fn mul(self, rhs: &Vector4u16) -> Self::Output {
        self.mul(*rhs)
    }
}

/// Components wise multiplication
/// v1 * v2 = [v1.x * v2.x, v1.y * v2.y, v1.z * v2.z, v1.w * v2.w]
impl std::ops::Mul<Vector4u16> for &Vector4u16 {
    type Output = Vector4u16;

    fn mul(self, rhs: Vector4u16) -> Self::Output {
        (*self).mul(rhs)
    }
}

/// Components wise multiplication
/// v1 * v2 = [v1.x * v2.x, v1.y * v2.y, v1.z * v2.z, v1.w * v2.w]
impl std::ops::Mul<&Vector4u16> for &Vector4u16 {
    type Output = Vector4u16;

    fn mul(self, rhs: &Vector4u16) -> Self::Output {
        (*self).mul(*rhs)
    }
}

/// Multiplies all components of the vector by `rhs`
/// v * f = [v.x * f, v.y * f, v.z * f, v.w * f]
impl std::ops::Mul<u16> for Vector4u16 {
    type Output = Vector4u16;

    fn mul(self, rhs: u16) -> Self::Output {
        Vector4u16 {
            data: self.data * u16x4::from_array([rhs, rhs, rhs, rhs]),
        }
    }
}

/// Multiplies all components of the vector by `rhs`
/// v * f = [v.x * f, v.y * f, v.z * f, v.w * f]
impl std::ops::Mul<u16> for &Vector4u16 {
    type Output = Vector4u16;

    fn mul(self, rhs: u16) -> Self::Output {
        (*self).mul(rhs)
    }
}

/// Multiplies all components of the vector by `rhs`
/// v * f = [v.x * f, v.y * f, v.z * f, v.w * f]
impl std::ops::Mul<&u16> for Vector4u16 {
    type Output = Vector4u16;

    fn mul(self, rhs: &u16) -> Self::Output {
        self.mul(*rhs)
    }
}

/// Multiplies all components of the vector by `rhs`
/// v * f = [v.x * f, v.y * f, v.z * f, v.w * f]
impl std::ops::Mul<&u16> for &Vector4u16 {
    type Output = Vector4u16;

    fn mul(self, rhs: &u16) -> Self::Output {
        (*self).mul(*rhs)
    }
}

//////////////////////////////////////////////////////////
//////////////     vector divisions     //////////////////
//////////////////////////////////////////////////////////

/// Components wise division
/// v1 / v2 = [v1.x / v2.x, v1.y / v2.y, v1.z / v2.z, v1.w / v2.w]
impl std::ops::Div<Vector4u16> for Vector4u16 {
    type Output = Vector4u16;

    fn div(self, rhs: Vector4u16) -> Self::Output {
        Vector4u16 {
            data: self.data / rhs.data,
        }
    }
}

/// Components wise division
/// v1 / v2 = [v1.x / v2.x, v1.y / v2.y, v1.z / v2.z, v1.w / v2.w]
impl std::ops::Div<&Vector4u16> for Vector4u16 {
    type Output = Vector4u16;

    fn div(self, rhs: &Vector4u16) -> Self::Output {
        self.div(*rhs)
    }
}

/// Components wise division
/// v1 / v2 = [v1.x / v2.x, v1.y / v2.y, v1.z / v2.z, v1.w / v2.w]
impl std::ops::Div<Vector4u16> for &Vector4u16 {
    type Output = Vector4u16;

    fn div(self, rhs: Vector4u16) -> Self::Output {
        (*self).div(rhs)
    }
}

/// Components wise division
/// v1 / v2 = [v1.x / v2.x, v1.y / v2.y, v1.z / v2.z, v1.w / v2.w]
impl std::ops::Div<&Vector4u16> for &Vector4u16 {
    type Output = Vector4u16;

    fn div(self, rhs: &Vector4u16) -> Self::Output {
        (*self).div(*rhs)
    }
}

/// Divides all components of the vector by `rhs`
/// v / f = [v.x / f, v.y / f, v.z / f, v.w / f]
impl std::ops::Div<u16> for Vector4u16 {
    type Output = Vector4u16;

    fn div(self, rhs: u16) -> Self::Output {
        Vector4u16 {
            data: self.data / u16x4::from_array([rhs, rhs, rhs, rhs]),
        }
    }
}

/// Divides all components of the vector by `rhs`
/// v / f = [v.x / f, v.y / f, v.z / f, v.w / f]
impl std::ops::Div<u16> for &Vector4u16 {
    type Output = Vector4u16;

    fn div(self, rhs: u16) -> Self::Output {
        (*self).div(rhs)
    }
}

/// Divides all components of the vector by `rhs`
/// v / f = [v.x / f, v.y / f, v.z / f, v.w / f]
impl std::ops::Div<&u16> for Vector4u16 {
    type Output = Vector4u16;

    fn div(self, rhs: &u16) -> Self::Output {
        self.div(*rhs)
    }
}

/// Divides all components of the vector by `rhs`
/// v / f = [v.x / f, v.y / f, v.z / f, v.w / f]
impl std::ops::Div<&u16> for &Vector4u16 {
    type Output = Vector4u16;

    fn div(self, rhs: &u16) -> Self::Output {
        (*self).div(*rhs)
    }
}

//////////////////////////////////////////////////////////
//////////////      vector assign       //////////////////
//////////////////////////////////////////////////////////

/// Components wise addition
/// v1 += v2 <=> v1 = [v1.x + v2.x, v1.y + v2.y, v1.z + v2.z, v1.w + v2.w]
impl std::ops::AddAssign<Vector4u16> for Vector4u16 {
    fn add_assign(&mut self, rhs: Vector4u16) {
        self.data = self.data + rhs.data;
    }
}

/// Components wise addition
/// v1 += v2 <=> v1 = [v1.x + v2.x, v1.y + v2.y, v1.z + v2.z, v1.w + v2.w]
impl std::ops::AddAssign<&Vector4u16> for Vector4u16 {
    fn add_assign(&mut self, rhs: &Vector4u16) {
        self.add_assign(*rhs);
    }
}

/// Adds `rhs` to all components of the vector
/// v += f <=> v = [v.x + f, v.y + f, v.z + f, v.w + f]
impl std::ops::AddAssign<u16> for Vector4u16 {
    fn add_assign(&mut self, rhs: u16) {
        self.data = self.data + u16x4::from_array([rhs, rhs, rhs, rhs]);
    }
}

/// Adds `rhs` to all components of the vector
/// v += f <=> v = [v.x + f, v.y + f, v.z + f, v.w + f]
impl std::ops::AddAssign<&u16> for Vector4u16 {
    fn add_assign(&mut self, rhs: &u16) {
        self.add_assign(*rhs);
    }
}

/// Components wise substraction
/// v1 -= v2 <=> v1 = [v1.x - v2.x, v1.y - v2.y, v1.z - v2.z, v1.w - v2.w]
impl std::ops::SubAssign<Vector4u16> for Vector4u16 {
    fn sub_assign(&mut self, rhs: Vector4u16) {
        self.data = self.data - rhs.data;
    }
}

/// Components wise substraction
/// v1 -= v2 <=> v1 = [v1.x - v2.x, v1.y - v2.y, v1.z - v2.z, v1.w - v2.w]
impl std::ops::SubAssign<&Vector4u16> for Vector4u16 {
    fn sub_assign(&mut self, rhs: &Vector4u16) {
        self.sub_assign(*rhs);
    }
}

/// Substracts `rhs` to all components of the vector
/// v -= f <=> v = [v.x - f, v.y - f, v.z - f, v.w - f]
impl std::ops::SubAssign<u16> for Vector4u16 {
    fn sub_assign(&mut self, rhs: u16) {
        self.data = self.data - u16x4::from_array([rhs, rhs, rhs, rhs]);
    }
}

/// Substracts `rhs` to all components of the vector
/// v -= f <=> v = [v.x - f, v.y - f, v.z - f, v.w - f]
impl std::ops::SubAssign<&u16> for Vector4u16 {
    fn sub_assign(&mut self, rhs: &u16) {
        self.sub_assign(*rhs);
    }
}

/// Components wise multiplication
/// v1 *= v2 <=> v1 = [v1.x * v2.x, v1.y * v2.y, v1.z * v2.z, v1.w * v2.w]
impl std::ops::MulAssign<Vector4u16> for Vector4u16 {
    fn mul_assign(&mut self, rhs: Vector4u16) {
        self.data = self.data * rhs.data;
    }
}

/// Components wise multiplication
/// v1 *= v2 <=> v1 = [v1.x * v2.x, v1.y * v2.y, v1.z * v2.z, v1.w * v2.w]
impl std::ops::MulAssign<&Vector4u16> for Vector4u16 {
    fn mul_assign(&mut self, rhs: &Vector4u16) {
        self.mul_assign(*rhs);
    }
}

/// Multiplies all components of the vector by `rhs`
/// v *= f <=> v = [v.x * f, v.y * f, v.z * f, v.w * f]
impl std::ops::MulAssign<u16> for Vector4u16 {
    fn mul_assign(&mut self, rhs: u16) {
        self.data = self.data * u16x4::from_array([rhs, rhs, rhs, rhs]);
    }
}

/// Multiplies all components of the vector by `rhs`
/// v *= f <=> v = [v.x * f, v.y * f, v.z * f, v.w * f]
impl std::ops::MulAssign<&u16> for Vector4u16 {
    fn mul_assign(&mut self, rhs: &u16) {
        self.mul_assign(*rhs);
    }
}

/// Components wise division
/// v1 /= v2 <=> v1 = [v1.x / v2.x, v1.y / v2.y, v1.z / v2.z, v1.w / v2.w]
impl std::ops::DivAssign<Vector4u16> for Vector4u16 {
    fn div_assign(&mut self, rhs: Vector4u16) {
        self.data = self.data / rhs.data;
    }
}

/// Components wise division
/// v1 /= v2 <=> v1 = [v1.x / v2.x, v1.y / v2.y, v1.z / v2.z, v1.w / v2.w]
impl std::ops::DivAssign<&Vector4u16> for Vector4u16 {
    fn div_assign(&mut self, rhs: &Vector4u16) {
        self.div_assign(*rhs);
    }
}

/// Divides all components of the vector by `rhs`
/// v /= f <=> v1 = [v.x / f, v.y / f, v.z / f, v.w / f]
impl std::ops::DivAssign<u16> for Vector4u16 {
    fn div_assign(&mut self, rhs: u16) {
        self.data = self.data / u16x4::from_array([rhs, rhs, rhs, rhs]);
    }
}

/// Divides all components of the vector by `rhs`
/// v /= f <=> v = [v.x / f, v.y / f, v.z / f, v.w / f]
impl std::ops::DivAssign<&u16> for Vector4u16 {
    fn div_assign(&mut self, rhs: &u16) {
        self.div_assign(*rhs);
    }
}

//////////////////////////////////////////////////////////
///////////////     vector tests      ////////////////////
//////////////////////////////////////////////////////////
#[cfg(test)]
mod tests {
    use super::*;

    /// Tests 4 dimensional vector initialization
    #[test]
    fn initialization() {
        let v1 = vec4u16(1, 1, 1, 1);
        let v2 = Vector4u16::ONES;
        assert_eq!(v1, v2);
    }

    /// Tests 4 dimensional vector operators
    #[test]
    fn operators() {
        let v1 = vec4u16(2, 3, 1, 5);
        let v2 = vec4u16(4, 5, 2, 2);
        assert_eq!(v1 + v2, vec4u16(6, 8, 3, 7));
        let mut v3 = v1;
        assert_eq!(v1, v3);
        v3 += v1;
        assert_eq!(v3, vec4u16(4, 6, 2, 10));

        v3 += 1;
        assert_eq!(v3, vec4u16(5, 7, 3, 11));
        assert_eq!(v3 + v3 - v3, v3);

        assert_eq!(v3 * 2, vec4u16(10, 14, 6, 22));
        assert_eq!(v1 * v2, vec4u16(8, 15, 2, 10));

        assert_eq!(v3 / v3, Vector4u16::ONES);
        v3 /= v3;
        assert_eq!(v3, Vector4u16::ONES);
    }

    /// Tests 4 dimensional vector operations
    #[test]
    fn operations() {
        let v1 = vec4u16(1, 2, 5, 6);
        let v2 = vec4u16(4, 3, 1, 1);
        assert_eq!(Vector4u16::dot(&v1, &v2), 21);
        let vx = Vector4u16::X;
        let vy = Vector4u16::Y;
        assert_eq!(Vector4u16::dot(&vx, &vy), 0);

        assert_eq!(vx.length(), 1.);
        assert_eq!(vy.length(), 1.);
        assert_eq!(v1.length(), 66f32.sqrt());
    }

    /// Tests 4 dimensional vector's fields getters and setters
    #[test]
    fn deref() {
        let v1 = vec4u16(5, 2, 1, 0);
        assert_eq!(v1.x, 5);
        assert_eq!(v1.y, 2);
        assert_eq!(v1.z, 1);
        assert_eq!(v1.w, 0);
        let mut v2 = Vector4u16::ZEROS;
        assert_eq!(v2.x, 0);
        assert_eq!(v2.y, 0);
        assert_eq!(v2.z, 0);
        assert_eq!(v2.w, 0);
        v2.x = 1;
        assert_eq!(v2.x, 1);
        v2.y = 3;
        assert_eq!(v2.y, 3);
        assert_eq!(v2.x, 1);
        v2.z = 4;
        v2.w = 5;
        assert_eq!(v2.y, 3);
        assert_eq!(v2.x, 1);
        assert_eq!(v2.z, 4);
        assert_eq!(v2.w, 5);
    }

    /// Tests the formatting of 4 dimensional vectors
    #[test]
    fn format() {
        let v1 = vec4u16(3, 4, 6, 1);
        assert_eq!(v1.to_string(), "(3, 4, 6, 1)");
        assert_eq!(format!("{:?}", v1), "Vector4u16 { x: 3, y: 4, z: 6, w: 1 }");
    }
}
