use std::simd::prelude::*;

/// A structure to represent a 4 dimensional isize vector
#[derive(Clone, Copy, PartialEq)]
#[repr(C)]
pub struct Vector4isize {
    data: isizex4,
}

/// A structure to be able to create `.x`, `.y`, `.z` and `.w` getters and setters
mod private {
    #[repr(C)]
    pub struct CoordsVector4 {
        pub x: isize,
        pub y: isize,
        pub z: isize,
        pub w: isize,
    }
}

/// Implements `Deref` to allow accessing `.x`, `.y`, `.z` and `.w`
impl std::ops::Deref for Vector4isize {
    type Target = private::CoordsVector4;

    fn deref(&self) -> &Self::Target {
        let value: *const Vector4isize = self;
        unsafe { &*(value as *const private::CoordsVector4) }
    }
}

/// Implements `DerefMut` to allow modifying `.x`, `.y`, `.z` and `.w`
impl std::ops::DerefMut for Vector4isize {
    fn deref_mut(&mut self) -> &mut Self::Target {
        let value: *mut Vector4isize = self;
        unsafe { &mut *(value as *mut private::CoordsVector4) }
    }
}

/// Overrides the debug trait
impl std::fmt::Debug for Vector4isize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Vector4isize")
            .field("x", &self.x)
            .field("y", &self.y)
            .field("z", &self.z)
            .field("w", &self.w)
            .finish()
    }
}

/// Overrides the display trait
impl std::fmt::Display for Vector4isize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {}, {})", self.x, self.y, self.z, self.w)
    }
}

/// Sets a 4 dimensional isize vector to `[0, 0, 0, 0]` as default
impl Default for Vector4isize {
    fn default() -> Self {
        Self::ZEROS
    }
}

/// Creates a 4 dimensional isize vector
pub fn vec4isize(x: isize, y: isize, z: isize, w: isize) -> Vector4isize {
    Vector4isize::new(x, y, z, w)
}

/// A union to cast simd to array and allow const construction
union UnionCast {
    array: [isize; 4],
    simd: Vector4isize,
}

impl Vector4isize {
    //////////////////////////////////////////////////////////
    /////////////      vector creation       /////////////////
    //////////////////////////////////////////////////////////

    /// Creates a new vector given its coordinates
    pub const fn new(x: isize, y: isize, z: isize, w: isize) -> Self {
        unsafe {
            UnionCast {
                array: [x, y, z, w],
            }
            .simd
        }
    }

    /// Creates a new vector with all coordinates set to `value`
    const fn splat(value: isize) -> Self {
        Self::new(value, value, value, value)
    }

    /// Creates a new vector filled with `values`
    pub const fn filled(value: isize) -> Self {
        Self::splat(value)
    }

    /// Creates a new vector filled with ones
    pub const ONES: Self = Self::splat(1);

    /// Creates a new vector filled with negative ones
    pub const NEG_ONES: Self = Self::splat(-1);

    /// Creates a new vector filled with zeros
    pub const ZEROS: Self = Self::splat(0);

    /// Creates a new vector filled with `isize::MIN``
    pub const MIN: Self = Self::splat(isize::MIN);

    /// Creates a new vector filled with `isize::MAX`
    pub const MAX: Self = Self::splat(isize::MAX);

    /// Creates a new vector pointing along the positive X axis
    pub const X: Self = Self::new(1, 0, 0, 0);

    /// Creates a new vector pointing along the positive Y axis
    pub const Y: Self = Self::new(0, 1, 0, 0);

    /// Creates a new vector pointing along the positive Z axis
    pub const Z: Self = Self::new(0, 0, 1, 0);

    /// Creates a new vector pointing along the positive W axis
    pub const W: Self = Self::new(0, 0, 0, 1);

    /// Creates a new vector pointing along the negative X axis
    pub const NEG_X: Self = Self::new(-1, 0, 0, 0);

    /// Creates a new vector pointing along the negative Y axis
    pub const NEG_Y: Self = Self::new(0, -1, 0, 0);

    /// Creates a new vector pointing along the negative Z axis
    pub const NEG_Z: Self = Self::new(0, 0, -1, 0);

    /// Creates a new vector pointing along the negative W axis
    pub const NEG_W: Self = Self::new(0, 0, 0, -1);

    //////////////////////////////////////////////////////////
    /////////////     vector operations      /////////////////
    //////////////////////////////////////////////////////////

    /// Sums up the element of the vector
    pub fn prefix_sum(self) -> isize {
        self.x + self.y + self.z + self.w
    }

    /// Dot product between two vectors
    pub fn dot(v1: &Vector4isize, v2: &Vector4isize) -> isize {
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
impl std::ops::Add<Vector4isize> for Vector4isize {
    type Output = Vector4isize;

    fn add(self, rhs: Vector4isize) -> Self::Output {
        Vector4isize {
            data: self.data + rhs.data,
        }
    }
}

/// Components wise addition
/// v1 + v2 = [v1.x + v2.x, v1.y + v2.y, v1.z + v2.z, v1.w + v2.w]
impl std::ops::Add<&Vector4isize> for Vector4isize {
    type Output = Vector4isize;

    fn add(self, rhs: &Vector4isize) -> Self::Output {
        self.add(*rhs)
    }
}

/// Components wise addition
/// v1 + v2 = [v1.x + v2.x, v1.y + v2.y, v1.z + v2.z, v1.w + v2.w]
impl std::ops::Add<Vector4isize> for &Vector4isize {
    type Output = Vector4isize;

    fn add(self, rhs: Vector4isize) -> Self::Output {
        (*self).add(rhs)
    }
}

/// Components wise addition
/// v1 + v2 = [v1.x + v2.x, v1.y + v2.y, v1.z + v2.z, v1.w + v2.w]
impl std::ops::Add<&Vector4isize> for &Vector4isize {
    type Output = Vector4isize;

    fn add(self, rhs: &Vector4isize) -> Self::Output {
        (*self).add(*rhs)
    }
}

/// Adds `rhs` to all components of the vector
/// v + f = [v.x + f, v.y + f, v.z + f, v.w + f]
impl std::ops::Add<isize> for Vector4isize {
    type Output = Vector4isize;

    fn add(self, rhs: isize) -> Self::Output {
        Vector4isize {
            data: self.data + isizex4::from_array([rhs, rhs, rhs, rhs]),
        }
    }
}

/// Adds `rhs` to all components of the vector
/// v + f = [v.x + f, v.y + f, v.z + f, v.w + f]
impl std::ops::Add<isize> for &Vector4isize {
    type Output = Vector4isize;

    fn add(self, rhs: isize) -> Self::Output {
        (*self).add(rhs)
    }
}

/// Adds `rhs` to all components of the vector
/// v + f = [v.x + f, v.y + f, v.z + f, v.w + f]
impl std::ops::Add<&isize> for Vector4isize {
    type Output = Vector4isize;

    fn add(self, rhs: &isize) -> Self::Output {
        self.add(*rhs)
    }
}

/// Adds `rhs` to all components of the vector
/// v + f = [v.x + f, v.y + f, v.z + f, v.w + f]
impl std::ops::Add<&isize> for &Vector4isize {
    type Output = Vector4isize;

    fn add(self, rhs: &isize) -> Self::Output {
        (*self).add(*rhs)
    }
}

//////////////////////////////////////////////////////////
//////////////   vector substractions    /////////////////
//////////////////////////////////////////////////////////

/// Components wise negation
/// -v = [-v.x, -v.y, -v.z, -v.w]
impl std::ops::Neg for Vector4isize {
    type Output = Vector4isize;

    fn neg(self) -> Self::Output {
        Vector4isize { data: -self.data }
    }
}

/// Components wise substraction
/// v1 - v2 = [v1.x - v2.x, v1.y - v2.y, v1.z - v2.z, v1.w - v2.w]
impl std::ops::Sub<Vector4isize> for Vector4isize {
    type Output = Vector4isize;

    fn sub(self, rhs: Vector4isize) -> Self::Output {
        Vector4isize {
            data: self.data - rhs.data,
        }
    }
}

/// Components wise substraction
/// v1 - v2 = [v1.x - v2.x, v1.y - v2.y, v1.z - v2.z, v1.w - v2.w]
impl std::ops::Sub<&Vector4isize> for Vector4isize {
    type Output = Vector4isize;

    fn sub(self, rhs: &Vector4isize) -> Self::Output {
        self.sub(*rhs)
    }
}

/// Components wise substraction
/// v1 - v2 = [v1.x - v2.x, v1.y - v2.y, v1.z - v2.z, v1.w - v2.w]
impl std::ops::Sub<Vector4isize> for &Vector4isize {
    type Output = Vector4isize;

    fn sub(self, rhs: Vector4isize) -> Self::Output {
        (*self).sub(rhs)
    }
}

/// Components wise substraction
/// v1 - v2 = [v1.x - v2.x, v1.y - v2.y, v1.z - v2.z, v1.w - v2.w]
impl std::ops::Sub<&Vector4isize> for &Vector4isize {
    type Output = Vector4isize;

    fn sub(self, rhs: &Vector4isize) -> Self::Output {
        (*self).sub(*rhs)
    }
}

/// Substract `rhs` to all components of the vector
/// v - f = [v.x - f, v.y - f, v.z - f, v.w - f]
impl std::ops::Sub<isize> for Vector4isize {
    type Output = Vector4isize;

    fn sub(self, rhs: isize) -> Self::Output {
        Vector4isize {
            data: self.data - isizex4::from_array([rhs, rhs, rhs, rhs]),
        }
    }
}

/// Substracts `rhs` to all components of the vector
/// v - f = [v.x - f, v.y - f, v.z - f, v.w - f]
impl std::ops::Sub<isize> for &Vector4isize {
    type Output = Vector4isize;

    fn sub(self, rhs: isize) -> Self::Output {
        (*self).sub(rhs)
    }
}

/// Substracts `rhs` to all components of the vector
/// v - f = [v.x - f, v.y - f, v.z - f, v.w - f]
impl std::ops::Sub<&isize> for Vector4isize {
    type Output = Vector4isize;

    fn sub(self, rhs: &isize) -> Self::Output {
        self.sub(*rhs)
    }
}

/// Substracts `rhs` to all components of the vector
/// v - f = [v.x - f, v.y - f, v.z - f, v.w - f]
impl std::ops::Sub<&isize> for &Vector4isize {
    type Output = Vector4isize;

    fn sub(self, rhs: &isize) -> Self::Output {
        (*self).sub(*rhs)
    }
}

//////////////////////////////////////////////////////////
//////////////   vector multiplications   ////////////////
//////////////////////////////////////////////////////////

/// Components wise multiplication
/// v1 * v2 = [v1.x * v2.x, v1.y * v2.y, v1.z * v2.z, v1.w * v2.w]
impl std::ops::Mul<Vector4isize> for Vector4isize {
    type Output = Vector4isize;

    fn mul(self, rhs: Vector4isize) -> Self::Output {
        Vector4isize {
            data: self.data * rhs.data,
        }
    }
}

/// Components wise multiplication
/// v1 * v2 = [v1.x * v2.x, v1.y * v2.y, v1.z * v2.z, v1.w * v2.w]
impl std::ops::Mul<&Vector4isize> for Vector4isize {
    type Output = Vector4isize;

    fn mul(self, rhs: &Vector4isize) -> Self::Output {
        self.mul(*rhs)
    }
}

/// Components wise multiplication
/// v1 * v2 = [v1.x * v2.x, v1.y * v2.y, v1.z * v2.z, v1.w * v2.w]
impl std::ops::Mul<Vector4isize> for &Vector4isize {
    type Output = Vector4isize;

    fn mul(self, rhs: Vector4isize) -> Self::Output {
        (*self).mul(rhs)
    }
}

/// Components wise multiplication
/// v1 * v2 = [v1.x * v2.x, v1.y * v2.y, v1.z * v2.z, v1.w * v2.w]
impl std::ops::Mul<&Vector4isize> for &Vector4isize {
    type Output = Vector4isize;

    fn mul(self, rhs: &Vector4isize) -> Self::Output {
        (*self).mul(*rhs)
    }
}

/// Multiplies all components of the vector by `rhs`
/// v * f = [v.x * f, v.y * f, v.z * f, v.w * f]
impl std::ops::Mul<isize> for Vector4isize {
    type Output = Vector4isize;

    fn mul(self, rhs: isize) -> Self::Output {
        Vector4isize {
            data: self.data * isizex4::from_array([rhs, rhs, rhs, rhs]),
        }
    }
}

/// Multiplies all components of the vector by `rhs`
/// v * f = [v.x * f, v.y * f, v.z * f, v.w * f]
impl std::ops::Mul<isize> for &Vector4isize {
    type Output = Vector4isize;

    fn mul(self, rhs: isize) -> Self::Output {
        (*self).mul(rhs)
    }
}

/// Multiplies all components of the vector by `rhs`
/// v * f = [v.x * f, v.y * f, v.z * f, v.w * f]
impl std::ops::Mul<&isize> for Vector4isize {
    type Output = Vector4isize;

    fn mul(self, rhs: &isize) -> Self::Output {
        self.mul(*rhs)
    }
}

/// Multiplies all components of the vector by `rhs`
/// v * f = [v.x * f, v.y * f, v.z * f, v.w * f]
impl std::ops::Mul<&isize> for &Vector4isize {
    type Output = Vector4isize;

    fn mul(self, rhs: &isize) -> Self::Output {
        (*self).mul(*rhs)
    }
}

//////////////////////////////////////////////////////////
//////////////     vector divisions     //////////////////
//////////////////////////////////////////////////////////

/// Components wise division
/// v1 / v2 = [v1.x / v2.x, v1.y / v2.y, v1.z / v2.z, v1.w / v2.w]
impl std::ops::Div<Vector4isize> for Vector4isize {
    type Output = Vector4isize;

    fn div(self, rhs: Vector4isize) -> Self::Output {
        Vector4isize {
            data: self.data / rhs.data,
        }
    }
}

/// Components wise division
/// v1 / v2 = [v1.x / v2.x, v1.y / v2.y, v1.z / v2.z, v1.w / v2.w]
impl std::ops::Div<&Vector4isize> for Vector4isize {
    type Output = Vector4isize;

    fn div(self, rhs: &Vector4isize) -> Self::Output {
        self.div(*rhs)
    }
}

/// Components wise division
/// v1 / v2 = [v1.x / v2.x, v1.y / v2.y, v1.z / v2.z, v1.w / v2.w]
impl std::ops::Div<Vector4isize> for &Vector4isize {
    type Output = Vector4isize;

    fn div(self, rhs: Vector4isize) -> Self::Output {
        (*self).div(rhs)
    }
}

/// Components wise division
/// v1 / v2 = [v1.x / v2.x, v1.y / v2.y, v1.z / v2.z, v1.w / v2.w]
impl std::ops::Div<&Vector4isize> for &Vector4isize {
    type Output = Vector4isize;

    fn div(self, rhs: &Vector4isize) -> Self::Output {
        (*self).div(*rhs)
    }
}

/// Divides all components of the vector by `rhs`
/// v / f = [v.x / f, v.y / f, v.z / f, v.w / f]
impl std::ops::Div<isize> for Vector4isize {
    type Output = Vector4isize;

    fn div(self, rhs: isize) -> Self::Output {
        Vector4isize {
            data: self.data / isizex4::from_array([rhs, rhs, rhs, rhs]),
        }
    }
}

/// Divides all components of the vector by `rhs`
/// v / f = [v.x / f, v.y / f, v.z / f, v.w / f]
impl std::ops::Div<isize> for &Vector4isize {
    type Output = Vector4isize;

    fn div(self, rhs: isize) -> Self::Output {
        (*self).div(rhs)
    }
}

/// Divides all components of the vector by `rhs`
/// v / f = [v.x / f, v.y / f, v.z / f, v.w / f]
impl std::ops::Div<&isize> for Vector4isize {
    type Output = Vector4isize;

    fn div(self, rhs: &isize) -> Self::Output {
        self.div(*rhs)
    }
}

/// Divides all components of the vector by `rhs`
/// v / f = [v.x / f, v.y / f, v.z / f, v.w / f]
impl std::ops::Div<&isize> for &Vector4isize {
    type Output = Vector4isize;

    fn div(self, rhs: &isize) -> Self::Output {
        (*self).div(*rhs)
    }
}

//////////////////////////////////////////////////////////
//////////////      vector assign       //////////////////
//////////////////////////////////////////////////////////

/// Components wise addition
/// v1 += v2 <=> v1 = [v1.x + v2.x, v1.y + v2.y, v1.z + v2.z, v1.w + v2.w]
impl std::ops::AddAssign<Vector4isize> for Vector4isize {
    fn add_assign(&mut self, rhs: Vector4isize) {
        self.data = self.data + rhs.data;
    }
}

/// Components wise addition
/// v1 += v2 <=> v1 = [v1.x + v2.x, v1.y + v2.y, v1.z + v2.z, v1.w + v2.w]
impl std::ops::AddAssign<&Vector4isize> for Vector4isize {
    fn add_assign(&mut self, rhs: &Vector4isize) {
        self.add_assign(*rhs);
    }
}

/// Adds `rhs` to all components of the vector
/// v += f <=> v = [v.x + f, v.y + f, v.z + f, v.w + f]
impl std::ops::AddAssign<isize> for Vector4isize {
    fn add_assign(&mut self, rhs: isize) {
        self.data = self.data + isizex4::from_array([rhs, rhs, rhs, rhs]);
    }
}

/// Adds `rhs` to all components of the vector
/// v += f <=> v = [v.x + f, v.y + f, v.z + f, v.w + f]
impl std::ops::AddAssign<&isize> for Vector4isize {
    fn add_assign(&mut self, rhs: &isize) {
        self.add_assign(*rhs);
    }
}

/// Components wise substraction
/// v1 -= v2 <=> v1 = [v1.x - v2.x, v1.y - v2.y, v1.z - v2.z, v1.w - v2.w]
impl std::ops::SubAssign<Vector4isize> for Vector4isize {
    fn sub_assign(&mut self, rhs: Vector4isize) {
        self.data = self.data - rhs.data;
    }
}

/// Components wise substraction
/// v1 -= v2 <=> v1 = [v1.x - v2.x, v1.y - v2.y, v1.z - v2.z, v1.w - v2.w]
impl std::ops::SubAssign<&Vector4isize> for Vector4isize {
    fn sub_assign(&mut self, rhs: &Vector4isize) {
        self.sub_assign(*rhs);
    }
}

/// Substracts `rhs` to all components of the vector
/// v -= f <=> v = [v.x - f, v.y - f, v.z - f, v.w - f]
impl std::ops::SubAssign<isize> for Vector4isize {
    fn sub_assign(&mut self, rhs: isize) {
        self.data = self.data - isizex4::from_array([rhs, rhs, rhs, rhs]);
    }
}

/// Substracts `rhs` to all components of the vector
/// v -= f <=> v = [v.x - f, v.y - f, v.z - f, v.w - f]
impl std::ops::SubAssign<&isize> for Vector4isize {
    fn sub_assign(&mut self, rhs: &isize) {
        self.sub_assign(*rhs);
    }
}

/// Components wise multiplication
/// v1 *= v2 <=> v1 = [v1.x * v2.x, v1.y * v2.y, v1.z * v2.z, v1.w * v2.w]
impl std::ops::MulAssign<Vector4isize> for Vector4isize {
    fn mul_assign(&mut self, rhs: Vector4isize) {
        self.data = self.data * rhs.data;
    }
}

/// Components wise multiplication
/// v1 *= v2 <=> v1 = [v1.x * v2.x, v1.y * v2.y, v1.z * v2.z, v1.w * v2.w]
impl std::ops::MulAssign<&Vector4isize> for Vector4isize {
    fn mul_assign(&mut self, rhs: &Vector4isize) {
        self.mul_assign(*rhs);
    }
}

/// Multiplies all components of the vector by `rhs`
/// v *= f <=> v = [v.x * f, v.y * f, v.z * f, v.w * f]
impl std::ops::MulAssign<isize> for Vector4isize {
    fn mul_assign(&mut self, rhs: isize) {
        self.data = self.data * isizex4::from_array([rhs, rhs, rhs, rhs]);
    }
}

/// Multiplies all components of the vector by `rhs`
/// v *= f <=> v = [v.x * f, v.y * f, v.z * f, v.w * f]
impl std::ops::MulAssign<&isize> for Vector4isize {
    fn mul_assign(&mut self, rhs: &isize) {
        self.mul_assign(*rhs);
    }
}

/// Components wise division
/// v1 /= v2 <=> v1 = [v1.x / v2.x, v1.y / v2.y, v1.z / v2.z, v1.w / v2.w]
impl std::ops::DivAssign<Vector4isize> for Vector4isize {
    fn div_assign(&mut self, rhs: Vector4isize) {
        self.data = self.data / rhs.data;
    }
}

/// Components wise division
/// v1 /= v2 <=> v1 = [v1.x / v2.x, v1.y / v2.y, v1.z / v2.z, v1.w / v2.w]
impl std::ops::DivAssign<&Vector4isize> for Vector4isize {
    fn div_assign(&mut self, rhs: &Vector4isize) {
        self.div_assign(*rhs);
    }
}

/// Divides all components of the vector by `rhs`
/// v /= f <=> v1 = [v.x / f, v.y / f, v.z / f, v.w / f]
impl std::ops::DivAssign<isize> for Vector4isize {
    fn div_assign(&mut self, rhs: isize) {
        self.data = self.data / isizex4::from_array([rhs, rhs, rhs, rhs]);
    }
}

/// Divides all components of the vector by `rhs`
/// v /= f <=> v = [v.x / f, v.y / f, v.z / f, v.w / f]
impl std::ops::DivAssign<&isize> for Vector4isize {
    fn div_assign(&mut self, rhs: &isize) {
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
        let v1 = vec4isize(1, 1, 1, 1);
        let v2 = Vector4isize::ONES;
        assert_eq!(v1, v2);
    }

    /// Tests 4 dimensional vector operators
    #[test]
    fn operators() {
        let v1 = vec4isize(2, 3, 1, 5);
        let v2 = vec4isize(4, 5, 2, 2);
        assert_eq!(v1 + v2, vec4isize(6, 8, 3, 7));
        let mut v3 = v1;
        assert_eq!(v1, v3);
        v3 += v1;
        assert_eq!(v3, vec4isize(4, 6, 2, 10));

        v3 += 1;
        assert_eq!(v3, vec4isize(5, 7, 3, 11));
        assert_eq!(v3 + v3 - v3, v3);

        assert_eq!(v3 * 2, vec4isize(10, 14, 6, 22));
        assert_eq!(v1 * v2, vec4isize(8, 15, 2, 10));

        assert_eq!(v3 / v3, Vector4isize::ONES);
        v3 /= v3;
        assert_eq!(v3, Vector4isize::ONES);
    }

    /// Tests 4 dimensional vector operations
    #[test]
    fn operations() {
        let v1 = vec4isize(1, 2, 5, 6);
        let v2 = vec4isize(4, 3, 1, 1);
        assert_eq!(Vector4isize::dot(&v1, &v2), 21);
        let vx = Vector4isize::X;
        let vy = Vector4isize::Y;
        assert_eq!(Vector4isize::dot(&vx, &vy), 0);

        assert_eq!(vx.length(), 1.);
        assert_eq!(vy.length(), 1.);
        assert_eq!(v1.length(), 66f32.sqrt());
    }

    /// Tests 4 dimensional vector's fields getters and setters
    #[test]
    fn deref() {
        let v1 = vec4isize(5, 2, 1, 0);
        assert_eq!(v1.x, 5);
        assert_eq!(v1.y, 2);
        assert_eq!(v1.z, 1);
        assert_eq!(v1.w, 0);
        let mut v2 = Vector4isize::ZEROS;
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
        let v1 = vec4isize(3, 4, 6, 1);
        assert_eq!(v1.to_string(), "(3, 4, 6, 1)");
        assert_eq!(
            format!("{:?}", v1),
            "Vector4isize { x: 3, y: 4, z: 6, w: 1 }"
        );
    }
}
