use std::simd::prelude::*;

/// A structure to represent a 3 dimensional usize vector
#[derive(Clone, Copy)]
#[repr(C)]
pub struct Vector3usize {
    data: usizex4, // can't do simd on line of 3
}

/// A structure to be able to create `.x`, `.y`, and `.z` getters and setters
mod private {
    #[repr(C)]
    pub struct CoordsVector3 {
        pub x: usize,
        pub y: usize,
        pub z: usize,
        _pad: usize, // can't do simd on line of 3
    }
}

/// Implements `Deref` to allow accessing `.x`, `.y`, and `.z`
impl std::ops::Deref for Vector3usize {
    type Target = private::CoordsVector3;

    fn deref(&self) -> &Self::Target {
        let value: *const Vector3usize = self;
        unsafe { &*(value as *const private::CoordsVector3) }
    }
}

/// Implements `DerefMut` to allow modifying `.x`, `.y`, and `.z`
impl std::ops::DerefMut for Vector3usize {
    fn deref_mut(&mut self) -> &mut Self::Target {
        let value: *mut Vector3usize = self;
        unsafe { &mut *(value as *mut private::CoordsVector3) }
    }
}

/// Overrides the debug trait
impl std::fmt::Debug for Vector3usize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Vector3usize")
            .field("x", &self.x)
            .field("y", &self.y)
            .field("z", &self.z)
            .finish()
    }
}

/// Overrides the display trait
impl std::fmt::Display for Vector3usize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

/// Sets a 3 dimensional usize vector to `[0, 0, 0]` as default
impl Default for Vector3usize {
    fn default() -> Self {
        Self::ZEROS
    }
}

/// Creates a 3 dimensional usize vector
pub fn vec3usize(x: usize, y: usize, z: usize) -> Vector3usize {
    Vector3usize::new(x, y, z)
}

/// A union to cast simd to array and allow const construction
union UnionCast {
    array: [usize; 4],
    simd: Vector3usize,
}

impl Vector3usize {
    //////////////////////////////////////////////////////////
    /////////////      vector creation       /////////////////
    //////////////////////////////////////////////////////////

    /// Creates a new vector given its coordinates
    pub const fn new(x: usize, y: usize, z: usize) -> Self {
        unsafe {
            UnionCast {
                array: [x, y, z, 1],
            }
            .simd
        }
    }

    /// Creates a new vector with all coordinates set to `value`
    const fn splat(value: usize) -> Self {
        Self::new(value, value, value)
    }

    /// Creates a new vector filled with `values`
    pub const fn filled(value: usize) -> Self {
        Self::splat(value)
    }

    /// Creates a new vector filled with ones
    pub const ONES: Self = Self::splat(1);

    /// Creates a new vector filled with zeros
    pub const ZEROS: Self = Self::splat(0);

    /// Creates a new vector filled with `usize::MIN``
    pub const MIN: Self = Self::splat(usize::MIN);

    /// Creates a new vector filled with `usize::MAX`
    pub const MAX: Self = Self::splat(usize::MAX);

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
    pub fn prefix_sum(self) -> usize {
        self.x + self.y + self.z
    }

    /// Dot product between two vectors
    pub fn dot(v1: &Vector3usize, v2: &Vector3usize) -> usize {
        (v1 * v2).prefix_sum()
    }

    /// Cross product between two vectors
    pub fn cross(v1: &Vector3usize, v2: &Vector3usize) -> Vector3usize {
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

impl PartialEq for Vector3usize {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}

//////////////////////////////////////////////////////////
//////////////     vector additions     //////////////////
//////////////////////////////////////////////////////////

/// Components wise addition
/// v1 + v2 = [v1.x + v2.x, v1.y + v2.y, v1.z + v2.z]
impl std::ops::Add<Vector3usize> for Vector3usize {
    type Output = Vector3usize;

    fn add(self, rhs: Vector3usize) -> Self::Output {
        Vector3usize {
            data: self.data + rhs.data,
        }
    }
}

/// Components wise addition
/// v1 + v2 = [v1.x + v2.x, v1.y + v2.y, v1.z + v2.z]
impl std::ops::Add<&Vector3usize> for Vector3usize {
    type Output = Vector3usize;

    fn add(self, rhs: &Vector3usize) -> Self::Output {
        self.add(*rhs)
    }
}

/// Components wise addition
/// v1 + v2 = [v1.x + v2.x, v1.y + v2.y, v1.z + v2.z]
impl std::ops::Add<Vector3usize> for &Vector3usize {
    type Output = Vector3usize;

    fn add(self, rhs: Vector3usize) -> Self::Output {
        (*self).add(rhs)
    }
}

/// Components wise addition
/// v1 + v2 = [v1.x + v2.x, v1.y + v2.y, v1.z + v2.z]
impl std::ops::Add<&Vector3usize> for &Vector3usize {
    type Output = Vector3usize;

    fn add(self, rhs: &Vector3usize) -> Self::Output {
        (*self).add(*rhs)
    }
}

/// Adds `rhs` to all components of the vector
/// v + f = [v.x + f, v.y + f, v.z + f]
impl std::ops::Add<usize> for Vector3usize {
    type Output = Vector3usize;

    fn add(self, rhs: usize) -> Self::Output {
        Vector3usize {
            data: self.data + usizex4::from_array([rhs, rhs, rhs, 0]),
        }
    }
}

/// Adds `rhs` to all components of the vector
/// v + f = [v.x + f, v.y + f, v.z + f]
impl std::ops::Add<usize> for &Vector3usize {
    type Output = Vector3usize;

    fn add(self, rhs: usize) -> Self::Output {
        (*self).add(rhs)
    }
}

/// Adds `rhs` to all components of the vector
/// v + f = [v.x + f, v.y + f, v.z + f]
impl std::ops::Add<&usize> for Vector3usize {
    type Output = Vector3usize;

    fn add(self, rhs: &usize) -> Self::Output {
        self.add(*rhs)
    }
}

/// Adds `rhs` to all components of the vector
/// v + f = [v.x + f, v.y + f, v.z + f]
impl std::ops::Add<&usize> for &Vector3usize {
    type Output = Vector3usize;

    fn add(self, rhs: &usize) -> Self::Output {
        (*self).add(*rhs)
    }
}

//////////////////////////////////////////////////////////
//////////////   vector substractions    /////////////////
//////////////////////////////////////////////////////////

/// Components wise substraction
/// v1 - v2 = [v1.x - v2.x, v1.y - v2.y, v1.z - v2.z]
impl std::ops::Sub<Vector3usize> for Vector3usize {
    type Output = Vector3usize;

    fn sub(self, rhs: Vector3usize) -> Self::Output {
        Vector3usize {
            data: self.data - rhs.data,
        }
    }
}

/// Components wise substraction
/// v1 - v2 = [v1.x - v2.x, v1.y - v2.y, v1.z - v2.z]
impl std::ops::Sub<&Vector3usize> for Vector3usize {
    type Output = Vector3usize;

    fn sub(self, rhs: &Vector3usize) -> Self::Output {
        self.sub(*rhs)
    }
}

/// Components wise substraction
/// v1 - v2 = [v1.x - v2.x, v1.y - v2.y, v1.z - v2.z]
impl std::ops::Sub<Vector3usize> for &Vector3usize {
    type Output = Vector3usize;

    fn sub(self, rhs: Vector3usize) -> Self::Output {
        (*self).sub(rhs)
    }
}

/// Components wise substraction
/// v1 - v2 = [v1.x - v2.x, v1.y - v2.y, v1.z - v2.z]
impl std::ops::Sub<&Vector3usize> for &Vector3usize {
    type Output = Vector3usize;

    fn sub(self, rhs: &Vector3usize) -> Self::Output {
        (*self).sub(*rhs)
    }
}

/// Substract `rhs` to all components of the vector
/// v - f = [v.x - f, v.y - f, v.z - f]
impl std::ops::Sub<usize> for Vector3usize {
    type Output = Vector3usize;

    fn sub(self, rhs: usize) -> Self::Output {
        Vector3usize {
            data: self.data - usizex4::from_array([rhs, rhs, rhs, 0]),
        }
    }
}

/// Substracts `rhs` to all components of the vector
/// v - f = [v.x - f, v.y - f, v.z - f]
impl std::ops::Sub<usize> for &Vector3usize {
    type Output = Vector3usize;

    fn sub(self, rhs: usize) -> Self::Output {
        (*self).sub(rhs)
    }
}

/// Substracts `rhs` to all components of the vector
/// v - f = [v.x - f, v.y - f, v.z - f]
impl std::ops::Sub<&usize> for Vector3usize {
    type Output = Vector3usize;

    fn sub(self, rhs: &usize) -> Self::Output {
        self.sub(*rhs)
    }
}

/// Substracts `rhs` to all components of the vector
/// v - f = [v.x - f, v.y - f, v.z - f]
impl std::ops::Sub<&usize> for &Vector3usize {
    type Output = Vector3usize;

    fn sub(self, rhs: &usize) -> Self::Output {
        (*self).sub(*rhs)
    }
}

//////////////////////////////////////////////////////////
//////////////   vector multiplications   ////////////////
//////////////////////////////////////////////////////////

/// Components wise multiplication
/// v1 * v2 = [v1.x * v2.x, v1.y * v2.y, v1.z * v2.z]
impl std::ops::Mul<Vector3usize> for Vector3usize {
    type Output = Vector3usize;

    fn mul(self, rhs: Vector3usize) -> Self::Output {
        Vector3usize {
            data: self.data * rhs.data,
        }
    }
}

/// Components wise multiplication
/// v1 * v2 = [v1.x * v2.x, v1.y * v2.y, v1.z * v2.z]
impl std::ops::Mul<&Vector3usize> for Vector3usize {
    type Output = Vector3usize;

    fn mul(self, rhs: &Vector3usize) -> Self::Output {
        self.mul(*rhs)
    }
}

/// Components wise multiplication
/// v1 * v2 = [v1.x * v2.x, v1.y * v2.y, v1.z * v2.z]
impl std::ops::Mul<Vector3usize> for &Vector3usize {
    type Output = Vector3usize;

    fn mul(self, rhs: Vector3usize) -> Self::Output {
        (*self).mul(rhs)
    }
}

/// Components wise multiplication
/// v1 * v2 = [v1.x * v2.x, v1.y * v2.y, v1.z * v2.z]
impl std::ops::Mul<&Vector3usize> for &Vector3usize {
    type Output = Vector3usize;

    fn mul(self, rhs: &Vector3usize) -> Self::Output {
        (*self).mul(*rhs)
    }
}

/// Multiplies all components of the vector by `rhs`
/// v * f = [v.x * f, v.y * f, v.z * f]
impl std::ops::Mul<usize> for Vector3usize {
    type Output = Vector3usize;

    fn mul(self, rhs: usize) -> Self::Output {
        Vector3usize {
            data: self.data * usizex4::from_array([rhs, rhs, rhs, 1]),
        }
    }
}

/// Multiplies all components of the vector by `rhs`
/// v * f = [v.x * f, v.y * f, v.z * f]
impl std::ops::Mul<usize> for &Vector3usize {
    type Output = Vector3usize;

    fn mul(self, rhs: usize) -> Self::Output {
        (*self).mul(rhs)
    }
}

/// Multiplies all components of the vector by `rhs`
/// v * f = [v.x * f, v.y * f, v.z * f]
impl std::ops::Mul<&usize> for Vector3usize {
    type Output = Vector3usize;

    fn mul(self, rhs: &usize) -> Self::Output {
        self.mul(*rhs)
    }
}

/// Multiplies all components of the vector by `rhs`
/// v * f = [v.x * f, v.y * f, v.z * f]
impl std::ops::Mul<&usize> for &Vector3usize {
    type Output = Vector3usize;

    fn mul(self, rhs: &usize) -> Self::Output {
        (*self).mul(*rhs)
    }
}

//////////////////////////////////////////////////////////
//////////////     vector divisions     //////////////////
//////////////////////////////////////////////////////////

/// Components wise division
/// v1 / v2 = [v1.x / v2.x, v1.y / v2.y, v1.z / v2.z]
impl std::ops::Div<Vector3usize> for Vector3usize {
    type Output = Vector3usize;

    fn div(self, rhs: Vector3usize) -> Self::Output {
        Vector3usize {
            data: self.data / rhs.data,
        }
    }
}

/// Components wise division
/// v1 / v2 = [v1.x / v2.x, v1.y / v2.y, v1.z / v2.z]
impl std::ops::Div<&Vector3usize> for Vector3usize {
    type Output = Vector3usize;

    fn div(self, rhs: &Vector3usize) -> Self::Output {
        self.div(*rhs)
    }
}

/// Components wise division
/// v1 / v2 = [v1.x / v2.x, v1.y / v2.y, v1.z / v2.z]
impl std::ops::Div<Vector3usize> for &Vector3usize {
    type Output = Vector3usize;

    fn div(self, rhs: Vector3usize) -> Self::Output {
        (*self).div(rhs)
    }
}

/// Components wise division
/// v1 / v2 = [v1.x / v2.x, v1.y / v2.y, v1.z / v2.z]
impl std::ops::Div<&Vector3usize> for &Vector3usize {
    type Output = Vector3usize;

    fn div(self, rhs: &Vector3usize) -> Self::Output {
        (*self).div(*rhs)
    }
}

/// Divides all components of the vector by `rhs`
/// v / f = [v.x / f, v.y / f, v.z / f]
impl std::ops::Div<usize> for Vector3usize {
    type Output = Vector3usize;

    fn div(self, rhs: usize) -> Self::Output {
        Vector3usize {
            data: self.data / usizex4::from_array([rhs, rhs, rhs, 1]),
        }
    }
}

/// Divides all components of the vector by `rhs`
/// v / f = [v.x / f, v.y / f, v.z / f]
impl std::ops::Div<usize> for &Vector3usize {
    type Output = Vector3usize;

    fn div(self, rhs: usize) -> Self::Output {
        (*self).div(rhs)
    }
}

/// Divides all components of the vector by `rhs`
/// v / f = [v.x / f, v.y / f, v.z / f]
impl std::ops::Div<&usize> for Vector3usize {
    type Output = Vector3usize;

    fn div(self, rhs: &usize) -> Self::Output {
        self.div(*rhs)
    }
}

/// Divides all components of the vector by `rhs`
/// v / f = [v.x / f, v.y / f, v.z / f]
impl std::ops::Div<&usize> for &Vector3usize {
    type Output = Vector3usize;

    fn div(self, rhs: &usize) -> Self::Output {
        (*self).div(*rhs)
    }
}

//////////////////////////////////////////////////////////
//////////////      vector assign       //////////////////
//////////////////////////////////////////////////////////

/// Components wise addition
/// v1 += v2 <=> v1 = [v1.x + v2.x, v1.y + v2.y, v1.z + v2.z]
impl std::ops::AddAssign<Vector3usize> for Vector3usize {
    fn add_assign(&mut self, rhs: Vector3usize) {
        self.data = self.data + rhs.data;
    }
}

/// Components wise addition
/// v1 += v2 <=> v1 = [v1.x + v2.x, v1.y + v2.y, v1.z + v2.z]
impl std::ops::AddAssign<&Vector3usize> for Vector3usize {
    fn add_assign(&mut self, rhs: &Vector3usize) {
        self.add_assign(*rhs);
    }
}

/// Adds `rhs` to all components of the vector
/// v += f <=> v = [v.x + f, v.y + f]
impl std::ops::AddAssign<usize> for Vector3usize {
    fn add_assign(&mut self, rhs: usize) {
        self.data = self.data + usizex4::from_array([rhs, rhs, rhs, 0]);
    }
}

/// Adds `rhs` to all components of the vector
/// v += f <=> v = [v.x + f, v.y + f]
impl std::ops::AddAssign<&usize> for Vector3usize {
    fn add_assign(&mut self, rhs: &usize) {
        self.add_assign(*rhs);
    }
}

/// Components wise substraction
/// v1 -= v2 <=> v1 = [v1.x - v2.x, v1.y - v2.y, v1.z - v2.z]
impl std::ops::SubAssign<Vector3usize> for Vector3usize {
    fn sub_assign(&mut self, rhs: Vector3usize) {
        self.data = self.data - rhs.data;
    }
}

/// Components wise substraction
/// v1 -= v2 <=> v1 = [v1.x - v2.x, v1.y - v2.y, v1.z - v2.z]
impl std::ops::SubAssign<&Vector3usize> for Vector3usize {
    fn sub_assign(&mut self, rhs: &Vector3usize) {
        self.sub_assign(*rhs);
    }
}

/// Substracts `rhs` to all components of the vector
/// v -= f <=> v = [v.x - f, v.y - f, v.z - f]
impl std::ops::SubAssign<usize> for Vector3usize {
    fn sub_assign(&mut self, rhs: usize) {
        self.data = self.data - usizex4::from_array([rhs, rhs, rhs, 0]);
    }
}

/// Substracts `rhs` to all components of the vector
/// v -= f <=> v = [v.x - f, v.y - f, v.z - f]
impl std::ops::SubAssign<&usize> for Vector3usize {
    fn sub_assign(&mut self, rhs: &usize) {
        self.sub_assign(*rhs);
    }
}

/// Components wise multiplication
/// v1 *= v2 <=> v1 = [v1.x * v2.x, v1.y * v2.y, v1.z * v2.z]
impl std::ops::MulAssign<Vector3usize> for Vector3usize {
    fn mul_assign(&mut self, rhs: Vector3usize) {
        self.data = self.data * rhs.data;
    }
}

/// Components wise multiplication
/// v1 *= v2 <=> v1 = [v1.x * v2.x, v1.y * v2.y, v1.z * v2.z]
impl std::ops::MulAssign<&Vector3usize> for Vector3usize {
    fn mul_assign(&mut self, rhs: &Vector3usize) {
        self.mul_assign(*rhs);
    }
}

/// Multiplies all components of the vector by `rhs`
/// v *= f <=> v = [v.x * f, v.y * f, v.z * f]
impl std::ops::MulAssign<usize> for Vector3usize {
    fn mul_assign(&mut self, rhs: usize) {
        self.data = self.data * usizex4::from_array([rhs, rhs, rhs, 1]);
    }
}

/// Multiplies all components of the vector by `rhs`
/// v *= f <=> v = [v.x * f, v.y * f, v.z * f]
impl std::ops::MulAssign<&usize> for Vector3usize {
    fn mul_assign(&mut self, rhs: &usize) {
        self.mul_assign(*rhs);
    }
}

/// Components wise division
/// v1 /= v2 <=> v1 = [v1.x / v2.x, v1.y / v2.y, v1.z / v2.z]
impl std::ops::DivAssign<Vector3usize> for Vector3usize {
    fn div_assign(&mut self, rhs: Vector3usize) {
        self.data = self.data / rhs.data;
    }
}

/// Components wise division
/// v1 /= v2 <=> v1 = [v1.x / v2.x, v1.y / v2.y, v1.z / v2.z]
impl std::ops::DivAssign<&Vector3usize> for Vector3usize {
    fn div_assign(&mut self, rhs: &Vector3usize) {
        self.div_assign(*rhs);
    }
}

/// Divides all components of the vector by `rhs`
/// v /= f <=> v1 = [v.x / f, v.y / f, v.z / f]
impl std::ops::DivAssign<usize> for Vector3usize {
    fn div_assign(&mut self, rhs: usize) {
        self.data = self.data / usizex4::from_array([rhs, rhs, rhs, 1]);
    }
}

/// Divides all components of the vector by `rhs`
/// v /= f <=> v = [v.x / f, v.y / f, v.z / f]
impl std::ops::DivAssign<&usize> for Vector3usize {
    fn div_assign(&mut self, rhs: &usize) {
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
        let v1 = vec3usize(1, 1, 1);
        let v2 = Vector3usize::ONES;
        assert_eq!(v1, v2);
    }

    /// Tests 3 dimensional vector operators
    #[test]
    fn operators() {
        let v1 = vec3usize(2, 3, 1);
        let v2 = vec3usize(4, 5, 6);
        assert_eq!(v1 + v2, vec3usize(6, 8, 7));
        let mut v3 = v1;
        assert_eq!(v1, v3);
        v3 += v1;
        assert_eq!(v3, vec3usize(4, 6, 2));

        v3 += 1;
        assert_eq!(v3, vec3usize(5, 7, 3));
        assert_eq!(v3 + v3 - v3, v3);

        assert_eq!(v3 * 2, vec3usize(10, 14, 6));
        assert_eq!(v1 * v2, vec3usize(8, 15, 6));

        assert_eq!(v3 / v3, Vector3usize::ONES);
        v3 /= v3;
        assert_eq!(v3, Vector3usize::ONES);
    }

    /// Tests panic outputs
    #[should_panic]
    #[test]
    fn invalid_cross_product() {
        let v1 = vec3usize(1, 2, 4);
        let v2 = vec3usize(4, 3, 5);
        let _ = Vector3usize::cross(&v1, &v2);
    }

    /// Tests 3 dimensional vector operations
    #[test]
    fn operations() {
        let v1 = vec3usize(1, 2, 4);
        let v2 = vec3usize(4, 3, 5);
        assert_eq!(Vector3usize::dot(&v1, &v2), 30);

        let vx = Vector3usize::X;
        let vy = Vector3usize::Y;
        let vz = Vector3usize::Z;
        assert_eq!(Vector3usize::dot(&vx, &vy), 0);

        assert_eq!(vx, vec3usize(1, 0, 0));
        assert_eq!(vy, vec3usize(0, 1, 0));
        assert_eq!(vz, vec3usize(0, 0, 1));

        assert_eq!(Vector3usize::cross(&vx, &vy), vz);
        assert_eq!(Vector3usize::cross(&vy, &vz), vx);
        assert_eq!(Vector3usize::cross(&vz, &vx), vy);

        assert_eq!(vx.length(), 1.);
        assert_eq!(vy.length(), 1.);
        assert_eq!(vz.length(), 1.);
        assert_eq!(v1.length(), 21f32.sqrt());
    }

    /// Tests 3 dimensional vector's fields getters and setters
    #[test]
    fn deref() {
        let v1 = vec3usize(5, 2, 1);
        assert_eq!(v1.x, 5);
        assert_eq!(v1.y, 2);
        assert_eq!(v1.z, 1);
        let mut v2 = Vector3usize::ZEROS;
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
        let v1 = vec3usize(3, 4, 6);
        assert_eq!(v1.to_string(), "(3, 4, 6)");
        assert_eq!(format!("{:?}", v1), "Vector3usize { x: 3, y: 4, z: 6 }");
    }
}
