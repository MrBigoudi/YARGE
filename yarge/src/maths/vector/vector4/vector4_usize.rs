use std::simd::prelude::*;

/// A structure to represent a 4 dimensional usize vector
#[derive(Clone, Copy, PartialEq)]
#[repr(C)]
pub struct Vector4usize {
    data: usizex4,
}

/// A structure to be able to create `.x`, `.y`, `.z` and `.w` getters and setters
mod private {
    #[repr(C)]
    pub struct CoordsVector4 {
        pub x: usize,
        pub y: usize,
        pub z: usize,
        pub w: usize,
    }
}

/// Implements `Deref` to allow accessing `.x`, `.y`, `.z` and `.w`
impl std::ops::Deref for Vector4usize {
    type Target = private::CoordsVector4;

    fn deref(&self) -> &Self::Target {
        let value: *const Vector4usize = self;
        unsafe { &*(value as *const private::CoordsVector4) }
    }
}

/// Implements `DerefMut` to allow modifying `.x`, `.y`, `.z` and `.w`
impl std::ops::DerefMut for Vector4usize {
    fn deref_mut(&mut self) -> &mut Self::Target {
        let value: *mut Vector4usize = self;
        unsafe { &mut *(value as *mut private::CoordsVector4) }
    }
}

/// Overrides the debug trait
impl std::fmt::Debug for Vector4usize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Vector4usize")
            .field("x", &self.x)
            .field("y", &self.y)
            .field("z", &self.z)
            .field("w", &self.w)
            .finish()
    }
}

/// Overrides the display trait
impl std::fmt::Display for Vector4usize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {}, {})", self.x, self.y, self.z, self.w)
    }
}

/// Sets a 4 dimensional usize vector to `[0, 0, 0, 0]` as default
impl Default for Vector4usize {
    fn default() -> Self {
        Self::ZEROS
    }
}

/// Creates a 4 dimensional usize vector
pub const fn vec4usize(x: usize, y: usize, z: usize, w: usize) -> Vector4usize {
    Vector4usize::new(x, y, z, w)
}

/// A union to cast simd to array and allow const construction
union UnionCast {
    array: [usize; 4],
    simd: Vector4usize,
}

impl Vector4usize {
    //////////////////////////////////////////////////////////
    /////////////      vector creation       /////////////////
    //////////////////////////////////////////////////////////

    /// Creates a new vector given its coordinates
    pub const fn new(x: usize, y: usize, z: usize, w: usize) -> Self {
        unsafe {
            UnionCast {
                array: [x, y, z, w],
            }
            .simd
        }
    }

    /// Creates a new vector with all coordinates set to `value`
    const fn splat(value: usize) -> Self {
        Self::new(value, value, value, value)
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
    pub fn prefix_sum(self) -> usize {
        self.x + self.y + self.z + self.w
    }

    /// Dot product between two vectors
    pub fn dot(v1: &Vector4usize, v2: &Vector4usize) -> usize {
        (v1 * v2).prefix_sum()
    }

    /// Returns the length of the vector
    pub fn length(&self) -> f32 {
        (Self::dot(self, self) as f32).sqrt()
    }

    /// Const accessor, only used for matrix initialization
    pub(in crate::maths) const fn x_const(&self) -> usize {
        self.data.as_array()[0]
    }
    /// Const accessor, only used for matrix initialization
    pub(in crate::maths) const fn y_const(&self) -> usize {
        self.data.as_array()[1]
    }
    /// Const accessor, only used for matrix initialization
    pub(in crate::maths) const fn z_const(&self) -> usize {
        self.data.as_array()[2]
    }
    /// Const accessor, only used for matrix initialization
    pub(in crate::maths) const fn w_const(&self) -> usize {
        self.data.as_array()[3]
    }
}

//////////////////////////////////////////////////////////
//////////////     vector additions     //////////////////
//////////////////////////////////////////////////////////

/// Components wise addition
/// v1 + v2 = [v1.x + v2.x, v1.y + v2.y, v1.z + v2.z, v1.w + v2.w]
impl std::ops::Add<Vector4usize> for Vector4usize {
    type Output = Vector4usize;

    fn add(self, rhs: Vector4usize) -> Self::Output {
        Vector4usize {
            data: self.data + rhs.data,
        }
    }
}

/// Components wise addition
/// v1 + v2 = [v1.x + v2.x, v1.y + v2.y, v1.z + v2.z, v1.w + v2.w]
impl std::ops::Add<&Vector4usize> for Vector4usize {
    type Output = Vector4usize;

    fn add(self, rhs: &Vector4usize) -> Self::Output {
        self.add(*rhs)
    }
}

/// Components wise addition
/// v1 + v2 = [v1.x + v2.x, v1.y + v2.y, v1.z + v2.z, v1.w + v2.w]
impl std::ops::Add<Vector4usize> for &Vector4usize {
    type Output = Vector4usize;

    fn add(self, rhs: Vector4usize) -> Self::Output {
        (*self).add(rhs)
    }
}

/// Components wise addition
/// v1 + v2 = [v1.x + v2.x, v1.y + v2.y, v1.z + v2.z, v1.w + v2.w]
impl std::ops::Add<&Vector4usize> for &Vector4usize {
    type Output = Vector4usize;

    fn add(self, rhs: &Vector4usize) -> Self::Output {
        (*self).add(*rhs)
    }
}

/// Adds `rhs` to all components of the vector
/// v + f = [v.x + f, v.y + f, v.z + f, v.w + f]
impl std::ops::Add<usize> for Vector4usize {
    type Output = Vector4usize;

    fn add(self, rhs: usize) -> Self::Output {
        Vector4usize {
            data: self.data + usizex4::from_array([rhs, rhs, rhs, rhs]),
        }
    }
}

/// Adds `rhs` to all components of the vector
/// v + f = [v.x + f, v.y + f, v.z + f, v.w + f]
impl std::ops::Add<usize> for &Vector4usize {
    type Output = Vector4usize;

    fn add(self, rhs: usize) -> Self::Output {
        (*self).add(rhs)
    }
}

/// Adds `rhs` to all components of the vector
/// v + f = [v.x + f, v.y + f, v.z + f, v.w + f]
impl std::ops::Add<&usize> for Vector4usize {
    type Output = Vector4usize;

    fn add(self, rhs: &usize) -> Self::Output {
        self.add(*rhs)
    }
}

/// Adds `rhs` to all components of the vector
/// v + f = [v.x + f, v.y + f, v.z + f, v.w + f]
impl std::ops::Add<&usize> for &Vector4usize {
    type Output = Vector4usize;

    fn add(self, rhs: &usize) -> Self::Output {
        (*self).add(*rhs)
    }
}

//////////////////////////////////////////////////////////
//////////////   vector substractions    /////////////////
//////////////////////////////////////////////////////////

/// Components wise substraction
/// v1 - v2 = [v1.x - v2.x, v1.y - v2.y, v1.z - v2.z, v1.w - v2.w]
impl std::ops::Sub<Vector4usize> for Vector4usize {
    type Output = Vector4usize;

    fn sub(self, rhs: Vector4usize) -> Self::Output {
        Vector4usize {
            data: self.data - rhs.data,
        }
    }
}

/// Components wise substraction
/// v1 - v2 = [v1.x - v2.x, v1.y - v2.y, v1.z - v2.z, v1.w - v2.w]
impl std::ops::Sub<&Vector4usize> for Vector4usize {
    type Output = Vector4usize;

    fn sub(self, rhs: &Vector4usize) -> Self::Output {
        self.sub(*rhs)
    }
}

/// Components wise substraction
/// v1 - v2 = [v1.x - v2.x, v1.y - v2.y, v1.z - v2.z, v1.w - v2.w]
impl std::ops::Sub<Vector4usize> for &Vector4usize {
    type Output = Vector4usize;

    fn sub(self, rhs: Vector4usize) -> Self::Output {
        (*self).sub(rhs)
    }
}

/// Components wise substraction
/// v1 - v2 = [v1.x - v2.x, v1.y - v2.y, v1.z - v2.z, v1.w - v2.w]
impl std::ops::Sub<&Vector4usize> for &Vector4usize {
    type Output = Vector4usize;

    fn sub(self, rhs: &Vector4usize) -> Self::Output {
        (*self).sub(*rhs)
    }
}

/// Substract `rhs` to all components of the vector
/// v - f = [v.x - f, v.y - f, v.z - f, v.w - f]
impl std::ops::Sub<usize> for Vector4usize {
    type Output = Vector4usize;

    fn sub(self, rhs: usize) -> Self::Output {
        Vector4usize {
            data: self.data - usizex4::from_array([rhs, rhs, rhs, rhs]),
        }
    }
}

/// Substracts `rhs` to all components of the vector
/// v - f = [v.x - f, v.y - f, v.z - f, v.w - f]
impl std::ops::Sub<usize> for &Vector4usize {
    type Output = Vector4usize;

    fn sub(self, rhs: usize) -> Self::Output {
        (*self).sub(rhs)
    }
}

/// Substracts `rhs` to all components of the vector
/// v - f = [v.x - f, v.y - f, v.z - f, v.w - f]
impl std::ops::Sub<&usize> for Vector4usize {
    type Output = Vector4usize;

    fn sub(self, rhs: &usize) -> Self::Output {
        self.sub(*rhs)
    }
}

/// Substracts `rhs` to all components of the vector
/// v - f = [v.x - f, v.y - f, v.z - f, v.w - f]
impl std::ops::Sub<&usize> for &Vector4usize {
    type Output = Vector4usize;

    fn sub(self, rhs: &usize) -> Self::Output {
        (*self).sub(*rhs)
    }
}

//////////////////////////////////////////////////////////
//////////////   vector multiplications   ////////////////
//////////////////////////////////////////////////////////

/// Components wise multiplication
/// v1 * v2 = [v1.x * v2.x, v1.y * v2.y, v1.z * v2.z, v1.w * v2.w]
impl std::ops::Mul<Vector4usize> for Vector4usize {
    type Output = Vector4usize;

    fn mul(self, rhs: Vector4usize) -> Self::Output {
        Vector4usize {
            data: self.data * rhs.data,
        }
    }
}

/// Components wise multiplication
/// v1 * v2 = [v1.x * v2.x, v1.y * v2.y, v1.z * v2.z, v1.w * v2.w]
impl std::ops::Mul<&Vector4usize> for Vector4usize {
    type Output = Vector4usize;

    fn mul(self, rhs: &Vector4usize) -> Self::Output {
        self.mul(*rhs)
    }
}

/// Components wise multiplication
/// v1 * v2 = [v1.x * v2.x, v1.y * v2.y, v1.z * v2.z, v1.w * v2.w]
impl std::ops::Mul<Vector4usize> for &Vector4usize {
    type Output = Vector4usize;

    fn mul(self, rhs: Vector4usize) -> Self::Output {
        (*self).mul(rhs)
    }
}

/// Components wise multiplication
/// v1 * v2 = [v1.x * v2.x, v1.y * v2.y, v1.z * v2.z, v1.w * v2.w]
impl std::ops::Mul<&Vector4usize> for &Vector4usize {
    type Output = Vector4usize;

    fn mul(self, rhs: &Vector4usize) -> Self::Output {
        (*self).mul(*rhs)
    }
}

/// Multiplies all components of the vector by `rhs`
/// v * f = [v.x * f, v.y * f, v.z * f, v.w * f]
impl std::ops::Mul<usize> for Vector4usize {
    type Output = Vector4usize;

    fn mul(self, rhs: usize) -> Self::Output {
        Vector4usize {
            data: self.data * usizex4::from_array([rhs, rhs, rhs, rhs]),
        }
    }
}

/// Multiplies all components of the vector by `rhs`
/// v * f = [v.x * f, v.y * f, v.z * f, v.w * f]
impl std::ops::Mul<usize> for &Vector4usize {
    type Output = Vector4usize;

    fn mul(self, rhs: usize) -> Self::Output {
        (*self).mul(rhs)
    }
}

/// Multiplies all components of the vector by `rhs`
/// v * f = [v.x * f, v.y * f, v.z * f, v.w * f]
impl std::ops::Mul<&usize> for Vector4usize {
    type Output = Vector4usize;

    fn mul(self, rhs: &usize) -> Self::Output {
        self.mul(*rhs)
    }
}

/// Multiplies all components of the vector by `rhs`
/// v * f = [v.x * f, v.y * f, v.z * f, v.w * f]
impl std::ops::Mul<&usize> for &Vector4usize {
    type Output = Vector4usize;

    fn mul(self, rhs: &usize) -> Self::Output {
        (*self).mul(*rhs)
    }
}

//////////////////////////////////////////////////////////
//////////////     vector divisions     //////////////////
//////////////////////////////////////////////////////////

/// Components wise division
/// v1 / v2 = [v1.x / v2.x, v1.y / v2.y, v1.z / v2.z, v1.w / v2.w]
impl std::ops::Div<Vector4usize> for Vector4usize {
    type Output = Vector4usize;

    fn div(self, rhs: Vector4usize) -> Self::Output {
        Vector4usize {
            data: self.data / rhs.data,
        }
    }
}

/// Components wise division
/// v1 / v2 = [v1.x / v2.x, v1.y / v2.y, v1.z / v2.z, v1.w / v2.w]
impl std::ops::Div<&Vector4usize> for Vector4usize {
    type Output = Vector4usize;

    fn div(self, rhs: &Vector4usize) -> Self::Output {
        self.div(*rhs)
    }
}

/// Components wise division
/// v1 / v2 = [v1.x / v2.x, v1.y / v2.y, v1.z / v2.z, v1.w / v2.w]
impl std::ops::Div<Vector4usize> for &Vector4usize {
    type Output = Vector4usize;

    fn div(self, rhs: Vector4usize) -> Self::Output {
        (*self).div(rhs)
    }
}

/// Components wise division
/// v1 / v2 = [v1.x / v2.x, v1.y / v2.y, v1.z / v2.z, v1.w / v2.w]
impl std::ops::Div<&Vector4usize> for &Vector4usize {
    type Output = Vector4usize;

    fn div(self, rhs: &Vector4usize) -> Self::Output {
        (*self).div(*rhs)
    }
}

/// Divides all components of the vector by `rhs`
/// v / f = [v.x / f, v.y / f, v.z / f, v.w / f]
impl std::ops::Div<usize> for Vector4usize {
    type Output = Vector4usize;

    fn div(self, rhs: usize) -> Self::Output {
        Vector4usize {
            data: self.data / usizex4::from_array([rhs, rhs, rhs, rhs]),
        }
    }
}

/// Divides all components of the vector by `rhs`
/// v / f = [v.x / f, v.y / f, v.z / f, v.w / f]
impl std::ops::Div<usize> for &Vector4usize {
    type Output = Vector4usize;

    fn div(self, rhs: usize) -> Self::Output {
        (*self).div(rhs)
    }
}

/// Divides all components of the vector by `rhs`
/// v / f = [v.x / f, v.y / f, v.z / f, v.w / f]
impl std::ops::Div<&usize> for Vector4usize {
    type Output = Vector4usize;

    fn div(self, rhs: &usize) -> Self::Output {
        self.div(*rhs)
    }
}

/// Divides all components of the vector by `rhs`
/// v / f = [v.x / f, v.y / f, v.z / f, v.w / f]
impl std::ops::Div<&usize> for &Vector4usize {
    type Output = Vector4usize;

    fn div(self, rhs: &usize) -> Self::Output {
        (*self).div(*rhs)
    }
}

//////////////////////////////////////////////////////////
//////////////      vector assign       //////////////////
//////////////////////////////////////////////////////////

/// Components wise addition
/// v1 += v2 <=> v1 = [v1.x + v2.x, v1.y + v2.y, v1.z + v2.z, v1.w + v2.w]
impl std::ops::AddAssign<Vector4usize> for Vector4usize {
    fn add_assign(&mut self, rhs: Vector4usize) {
        self.data = self.data + rhs.data;
    }
}

/// Components wise addition
/// v1 += v2 <=> v1 = [v1.x + v2.x, v1.y + v2.y, v1.z + v2.z, v1.w + v2.w]
impl std::ops::AddAssign<&Vector4usize> for Vector4usize {
    fn add_assign(&mut self, rhs: &Vector4usize) {
        self.add_assign(*rhs);
    }
}

/// Adds `rhs` to all components of the vector
/// v += f <=> v = [v.x + f, v.y + f, v.z + f, v.w + f]
impl std::ops::AddAssign<usize> for Vector4usize {
    fn add_assign(&mut self, rhs: usize) {
        self.data = self.data + usizex4::from_array([rhs, rhs, rhs, rhs]);
    }
}

/// Adds `rhs` to all components of the vector
/// v += f <=> v = [v.x + f, v.y + f, v.z + f, v.w + f]
impl std::ops::AddAssign<&usize> for Vector4usize {
    fn add_assign(&mut self, rhs: &usize) {
        self.add_assign(*rhs);
    }
}

/// Components wise substraction
/// v1 -= v2 <=> v1 = [v1.x - v2.x, v1.y - v2.y, v1.z - v2.z, v1.w - v2.w]
impl std::ops::SubAssign<Vector4usize> for Vector4usize {
    fn sub_assign(&mut self, rhs: Vector4usize) {
        self.data = self.data - rhs.data;
    }
}

/// Components wise substraction
/// v1 -= v2 <=> v1 = [v1.x - v2.x, v1.y - v2.y, v1.z - v2.z, v1.w - v2.w]
impl std::ops::SubAssign<&Vector4usize> for Vector4usize {
    fn sub_assign(&mut self, rhs: &Vector4usize) {
        self.sub_assign(*rhs);
    }
}

/// Substracts `rhs` to all components of the vector
/// v -= f <=> v = [v.x - f, v.y - f, v.z - f, v.w - f]
impl std::ops::SubAssign<usize> for Vector4usize {
    fn sub_assign(&mut self, rhs: usize) {
        self.data = self.data - usizex4::from_array([rhs, rhs, rhs, rhs]);
    }
}

/// Substracts `rhs` to all components of the vector
/// v -= f <=> v = [v.x - f, v.y - f, v.z - f, v.w - f]
impl std::ops::SubAssign<&usize> for Vector4usize {
    fn sub_assign(&mut self, rhs: &usize) {
        self.sub_assign(*rhs);
    }
}

/// Components wise multiplication
/// v1 *= v2 <=> v1 = [v1.x * v2.x, v1.y * v2.y, v1.z * v2.z, v1.w * v2.w]
impl std::ops::MulAssign<Vector4usize> for Vector4usize {
    fn mul_assign(&mut self, rhs: Vector4usize) {
        self.data = self.data * rhs.data;
    }
}

/// Components wise multiplication
/// v1 *= v2 <=> v1 = [v1.x * v2.x, v1.y * v2.y, v1.z * v2.z, v1.w * v2.w]
impl std::ops::MulAssign<&Vector4usize> for Vector4usize {
    fn mul_assign(&mut self, rhs: &Vector4usize) {
        self.mul_assign(*rhs);
    }
}

/// Multiplies all components of the vector by `rhs`
/// v *= f <=> v = [v.x * f, v.y * f, v.z * f, v.w * f]
impl std::ops::MulAssign<usize> for Vector4usize {
    fn mul_assign(&mut self, rhs: usize) {
        self.data = self.data * usizex4::from_array([rhs, rhs, rhs, rhs]);
    }
}

/// Multiplies all components of the vector by `rhs`
/// v *= f <=> v = [v.x * f, v.y * f, v.z * f, v.w * f]
impl std::ops::MulAssign<&usize> for Vector4usize {
    fn mul_assign(&mut self, rhs: &usize) {
        self.mul_assign(*rhs);
    }
}

/// Components wise division
/// v1 /= v2 <=> v1 = [v1.x / v2.x, v1.y / v2.y, v1.z / v2.z, v1.w / v2.w]
impl std::ops::DivAssign<Vector4usize> for Vector4usize {
    fn div_assign(&mut self, rhs: Vector4usize) {
        self.data = self.data / rhs.data;
    }
}

/// Components wise division
/// v1 /= v2 <=> v1 = [v1.x / v2.x, v1.y / v2.y, v1.z / v2.z, v1.w / v2.w]
impl std::ops::DivAssign<&Vector4usize> for Vector4usize {
    fn div_assign(&mut self, rhs: &Vector4usize) {
        self.div_assign(*rhs);
    }
}

/// Divides all components of the vector by `rhs`
/// v /= f <=> v1 = [v.x / f, v.y / f, v.z / f, v.w / f]
impl std::ops::DivAssign<usize> for Vector4usize {
    fn div_assign(&mut self, rhs: usize) {
        self.data = self.data / usizex4::from_array([rhs, rhs, rhs, rhs]);
    }
}

/// Divides all components of the vector by `rhs`
/// v /= f <=> v = [v.x / f, v.y / f, v.z / f, v.w / f]
impl std::ops::DivAssign<&usize> for Vector4usize {
    fn div_assign(&mut self, rhs: &usize) {
        self.div_assign(*rhs);
    }
}

//////////////////////////////////////////////////////////
///////////////     vector indices     ///////////////////
//////////////////////////////////////////////////////////
impl std::ops::Index<usize> for Vector4usize {
    type Output = usize;
    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            3 => &self.w,
            _ => panic!("Index out of bounds"),
        }
    }
}

impl std::ops::IndexMut<usize> for Vector4usize {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            3 => &mut self.w,
            _ => panic!("Index out of bounds"),
        }
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
        let v1 = vec4usize(1, 1, 1, 1);
        let v2 = Vector4usize::ONES;
        assert_eq!(v1, v2);
    }

    /// Tests 4 dimensional vector operators
    #[test]
    fn operators() {
        let v1 = vec4usize(2, 3, 1, 5);
        let v2 = vec4usize(4, 5, 2, 2);
        assert_eq!(v1 + v2, vec4usize(6, 8, 3, 7));
        let mut v3 = v1;
        assert_eq!(v1, v3);
        v3 += v1;
        assert_eq!(v3, vec4usize(4, 6, 2, 10));

        v3 += 1;
        assert_eq!(v3, vec4usize(5, 7, 3, 11));
        assert_eq!(v3 + v3 - v3, v3);

        assert_eq!(v3 * 2, vec4usize(10, 14, 6, 22));
        assert_eq!(v1 * v2, vec4usize(8, 15, 2, 10));

        assert_eq!(v3 / v3, Vector4usize::ONES);
        v3 /= v3;
        assert_eq!(v3, Vector4usize::ONES);
    }

    /// Tests 4 dimensional vector operations
    #[test]
    fn operations() {
        let v1 = vec4usize(1, 2, 5, 6);
        let v2 = vec4usize(4, 3, 1, 1);
        assert_eq!(Vector4usize::dot(&v1, &v2), 21);
        let vx = Vector4usize::X;
        let vy = Vector4usize::Y;
        assert_eq!(Vector4usize::dot(&vx, &vy), 0);

        assert_eq!(vx.length(), 1.);
        assert_eq!(vy.length(), 1.);
        assert_eq!(v1.length(), 66f32.sqrt());
    }

    /// Tests 4 dimensional vector's fields getters and setters
    #[test]
    fn deref() {
        let v1 = vec4usize(5, 2, 1, 0);
        assert_eq!(v1.x, 5);
        assert_eq!(v1.y, 2);
        assert_eq!(v1.z, 1);
        assert_eq!(v1.w, 0);
        let mut v2 = Vector4usize::ZEROS;
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
        let v1 = vec4usize(3, 4, 6, 1);
        assert_eq!(v1.to_string(), "(3, 4, 6, 1)");
        assert_eq!(
            format!("{:?}", v1),
            "Vector4usize { x: 3, y: 4, z: 6, w: 1 }"
        );
    }

    /// Tests indices access
    #[test]
    fn indices() {
        let mut v1 = vec4usize(2, 3, 1, 5);
        assert_eq!(v1[2], 1);
        v1[3] = 2;
        assert_eq!(v1[3], 2);
    }
}
