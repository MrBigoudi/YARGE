use std::simd::prelude::*;

/// A structure to represent a 3 dimensional isize vector
#[derive(Clone, Copy)]
#[repr(C)]
pub struct Vector3isize {
    data: isizex4, // can't do simd on line of 3
}

/// A structure to be able to create `.x`, `.y`, and `.z` getters and setters
mod private {
    #[repr(C)]
    pub struct CoordsVector3 {
        pub x: isize,
        pub y: isize,
        pub z: isize,
        _pad: isize, // can't do simd on line of 3
    }
}

/// Implements `Deref` to allow accessing `.x`, `.y`, and `.z`
impl std::ops::Deref for Vector3isize {
    type Target = private::CoordsVector3;

    fn deref(&self) -> &Self::Target {
        let value: *const Vector3isize = self;
        unsafe { &*(value as *const private::CoordsVector3) }
    }
}

/// Implements `DerefMut` to allow modifying `.x`, `.y`, and `.z`
impl std::ops::DerefMut for Vector3isize {
    fn deref_mut(&mut self) -> &mut Self::Target {
        let value: *mut Vector3isize = self;
        unsafe { &mut *(value as *mut private::CoordsVector3) }
    }
}

/// Overrides the debug trait
impl std::fmt::Debug for Vector3isize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Vector3isize")
            .field("x", &self.x)
            .field("y", &self.y)
            .field("z", &self.z)
            .finish()
    }
}

/// Overrides the display trait
impl std::fmt::Display for Vector3isize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

/// Sets a 3 dimensional isize vector to `[0, 0, 0]` as default
impl Default for Vector3isize {
    fn default() -> Self {
        Self::ZEROS
    }
}

/// Creates a 3 dimensional isize vector
pub const fn vec3isize(x: isize, y: isize, z: isize) -> Vector3isize {
    Vector3isize::new(x, y, z)
}

/// A union to cast simd to array and allow const construction
union UnionCast {
    array: [isize; 4],
    simd: Vector3isize,
}

impl Vector3isize {
    //////////////////////////////////////////////////////////
    /////////////      vector creation       /////////////////
    //////////////////////////////////////////////////////////

    /// Creates a new vector given its coordinates
    pub const fn new(x: isize, y: isize, z: isize) -> Self {
        unsafe {
            UnionCast {
                array: [x, y, z, 1],
            }
            .simd
        }
    }

    /// Creates a new vector with all coordinates set to `value`
    const fn splat(value: isize) -> Self {
        Self::new(value, value, value)
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
    pub fn prefix_sum(self) -> isize {
        self.x + self.y + self.z
    }

    /// Dot product between two vectors
    pub fn dot(v1: &Vector3isize, v2: &Vector3isize) -> isize {
        (v1 * v2).prefix_sum()
    }

    /// Cross product between two vectors
    pub fn cross(v1: &Vector3isize, v2: &Vector3isize) -> Vector3isize {
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

    /// Const accessor, only used for matrix initialization
    pub(in crate::maths) const fn x_const(&self) -> isize {
        self.data.as_array()[0]
    }
    /// Const accessor, only used for matrix initialization
    pub(in crate::maths) const fn y_const(&self) -> isize {
        self.data.as_array()[1]
    }
    /// Const accessor, only used for matrix initialization
    pub(in crate::maths) const fn z_const(&self) -> isize {
        self.data.as_array()[2]
    }
}

impl PartialEq for Vector3isize {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}

//////////////////////////////////////////////////////////
//////////////     vector additions     //////////////////
//////////////////////////////////////////////////////////

/// Components wise addition
/// v1 + v2 = [v1.x + v2.x, v1.y + v2.y, v1.z + v2.z]
impl std::ops::Add<Vector3isize> for Vector3isize {
    type Output = Vector3isize;

    fn add(self, rhs: Vector3isize) -> Self::Output {
        Vector3isize {
            data: self.data + rhs.data,
        }
    }
}

/// Components wise addition
/// v1 + v2 = [v1.x + v2.x, v1.y + v2.y, v1.z + v2.z]
impl std::ops::Add<&Vector3isize> for Vector3isize {
    type Output = Vector3isize;

    fn add(self, rhs: &Vector3isize) -> Self::Output {
        self.add(*rhs)
    }
}

/// Components wise addition
/// v1 + v2 = [v1.x + v2.x, v1.y + v2.y, v1.z + v2.z]
impl std::ops::Add<Vector3isize> for &Vector3isize {
    type Output = Vector3isize;

    fn add(self, rhs: Vector3isize) -> Self::Output {
        (*self).add(rhs)
    }
}

/// Components wise addition
/// v1 + v2 = [v1.x + v2.x, v1.y + v2.y, v1.z + v2.z]
impl std::ops::Add<&Vector3isize> for &Vector3isize {
    type Output = Vector3isize;

    fn add(self, rhs: &Vector3isize) -> Self::Output {
        (*self).add(*rhs)
    }
}

/// Adds `rhs` to all components of the vector
/// v + f = [v.x + f, v.y + f, v.z + f]
impl std::ops::Add<isize> for Vector3isize {
    type Output = Vector3isize;

    fn add(self, rhs: isize) -> Self::Output {
        Vector3isize {
            data: self.data + isizex4::from_array([rhs, rhs, rhs, 0]),
        }
    }
}

/// Adds `rhs` to all components of the vector
/// v + f = [v.x + f, v.y + f, v.z + f]
impl std::ops::Add<isize> for &Vector3isize {
    type Output = Vector3isize;

    fn add(self, rhs: isize) -> Self::Output {
        (*self).add(rhs)
    }
}

/// Adds `rhs` to all components of the vector
/// v + f = [v.x + f, v.y + f, v.z + f]
impl std::ops::Add<&isize> for Vector3isize {
    type Output = Vector3isize;

    fn add(self, rhs: &isize) -> Self::Output {
        self.add(*rhs)
    }
}

/// Adds `rhs` to all components of the vector
/// v + f = [v.x + f, v.y + f, v.z + f]
impl std::ops::Add<&isize> for &Vector3isize {
    type Output = Vector3isize;

    fn add(self, rhs: &isize) -> Self::Output {
        (*self).add(*rhs)
    }
}

//////////////////////////////////////////////////////////
//////////////   vector substractions    /////////////////
//////////////////////////////////////////////////////////

/// Components wise negation
/// -v = [-v.x, -v.y, -v.z]
impl std::ops::Neg for Vector3isize {
    type Output = Vector3isize;

    fn neg(self) -> Self::Output {
        Vector3isize { data: -self.data }
    }
}

/// Components wise substraction
/// v1 - v2 = [v1.x - v2.x, v1.y - v2.y, v1.z - v2.z]
impl std::ops::Sub<Vector3isize> for Vector3isize {
    type Output = Vector3isize;

    fn sub(self, rhs: Vector3isize) -> Self::Output {
        Vector3isize {
            data: self.data - rhs.data,
        }
    }
}

/// Components wise substraction
/// v1 - v2 = [v1.x - v2.x, v1.y - v2.y, v1.z - v2.z]
impl std::ops::Sub<&Vector3isize> for Vector3isize {
    type Output = Vector3isize;

    fn sub(self, rhs: &Vector3isize) -> Self::Output {
        self.sub(*rhs)
    }
}

/// Components wise substraction
/// v1 - v2 = [v1.x - v2.x, v1.y - v2.y, v1.z - v2.z]
impl std::ops::Sub<Vector3isize> for &Vector3isize {
    type Output = Vector3isize;

    fn sub(self, rhs: Vector3isize) -> Self::Output {
        (*self).sub(rhs)
    }
}

/// Components wise substraction
/// v1 - v2 = [v1.x - v2.x, v1.y - v2.y, v1.z - v2.z]
impl std::ops::Sub<&Vector3isize> for &Vector3isize {
    type Output = Vector3isize;

    fn sub(self, rhs: &Vector3isize) -> Self::Output {
        (*self).sub(*rhs)
    }
}

/// Substract `rhs` to all components of the vector
/// v - f = [v.x - f, v.y - f, v.z - f]
impl std::ops::Sub<isize> for Vector3isize {
    type Output = Vector3isize;

    fn sub(self, rhs: isize) -> Self::Output {
        Vector3isize {
            data: self.data - isizex4::from_array([rhs, rhs, rhs, 0]),
        }
    }
}

/// Substracts `rhs` to all components of the vector
/// v - f = [v.x - f, v.y - f, v.z - f]
impl std::ops::Sub<isize> for &Vector3isize {
    type Output = Vector3isize;

    fn sub(self, rhs: isize) -> Self::Output {
        (*self).sub(rhs)
    }
}

/// Substracts `rhs` to all components of the vector
/// v - f = [v.x - f, v.y - f, v.z - f]
impl std::ops::Sub<&isize> for Vector3isize {
    type Output = Vector3isize;

    fn sub(self, rhs: &isize) -> Self::Output {
        self.sub(*rhs)
    }
}

/// Substracts `rhs` to all components of the vector
/// v - f = [v.x - f, v.y - f, v.z - f]
impl std::ops::Sub<&isize> for &Vector3isize {
    type Output = Vector3isize;

    fn sub(self, rhs: &isize) -> Self::Output {
        (*self).sub(*rhs)
    }
}

//////////////////////////////////////////////////////////
//////////////   vector multiplications   ////////////////
//////////////////////////////////////////////////////////

/// Components wise multiplication
/// v1 * v2 = [v1.x * v2.x, v1.y * v2.y, v1.z * v2.z]
impl std::ops::Mul<Vector3isize> for Vector3isize {
    type Output = Vector3isize;

    fn mul(self, rhs: Vector3isize) -> Self::Output {
        Vector3isize {
            data: self.data * rhs.data,
        }
    }
}

/// Components wise multiplication
/// v1 * v2 = [v1.x * v2.x, v1.y * v2.y, v1.z * v2.z]
impl std::ops::Mul<&Vector3isize> for Vector3isize {
    type Output = Vector3isize;

    fn mul(self, rhs: &Vector3isize) -> Self::Output {
        self.mul(*rhs)
    }
}

/// Components wise multiplication
/// v1 * v2 = [v1.x * v2.x, v1.y * v2.y, v1.z * v2.z]
impl std::ops::Mul<Vector3isize> for &Vector3isize {
    type Output = Vector3isize;

    fn mul(self, rhs: Vector3isize) -> Self::Output {
        (*self).mul(rhs)
    }
}

/// Components wise multiplication
/// v1 * v2 = [v1.x * v2.x, v1.y * v2.y, v1.z * v2.z]
impl std::ops::Mul<&Vector3isize> for &Vector3isize {
    type Output = Vector3isize;

    fn mul(self, rhs: &Vector3isize) -> Self::Output {
        (*self).mul(*rhs)
    }
}

/// Multiplies all components of the vector by `rhs`
/// v * f = [v.x * f, v.y * f, v.z * f]
impl std::ops::Mul<isize> for Vector3isize {
    type Output = Vector3isize;

    fn mul(self, rhs: isize) -> Self::Output {
        Vector3isize {
            data: self.data * isizex4::from_array([rhs, rhs, rhs, 1]),
        }
    }
}

/// Multiplies all components of the vector by `rhs`
/// v * f = [v.x * f, v.y * f, v.z * f]
impl std::ops::Mul<isize> for &Vector3isize {
    type Output = Vector3isize;

    fn mul(self, rhs: isize) -> Self::Output {
        (*self).mul(rhs)
    }
}

/// Multiplies all components of the vector by `rhs`
/// v * f = [v.x * f, v.y * f, v.z * f]
impl std::ops::Mul<&isize> for Vector3isize {
    type Output = Vector3isize;

    fn mul(self, rhs: &isize) -> Self::Output {
        self.mul(*rhs)
    }
}

/// Multiplies all components of the vector by `rhs`
/// v * f = [v.x * f, v.y * f, v.z * f]
impl std::ops::Mul<&isize> for &Vector3isize {
    type Output = Vector3isize;

    fn mul(self, rhs: &isize) -> Self::Output {
        (*self).mul(*rhs)
    }
}

//////////////////////////////////////////////////////////
//////////////     vector divisions     //////////////////
//////////////////////////////////////////////////////////

/// Components wise division
/// v1 / v2 = [v1.x / v2.x, v1.y / v2.y, v1.z / v2.z]
impl std::ops::Div<Vector3isize> for Vector3isize {
    type Output = Vector3isize;

    fn div(self, rhs: Vector3isize) -> Self::Output {
        Vector3isize {
            data: self.data / rhs.data,
        }
    }
}

/// Components wise division
/// v1 / v2 = [v1.x / v2.x, v1.y / v2.y, v1.z / v2.z]
impl std::ops::Div<&Vector3isize> for Vector3isize {
    type Output = Vector3isize;

    fn div(self, rhs: &Vector3isize) -> Self::Output {
        self.div(*rhs)
    }
}

/// Components wise division
/// v1 / v2 = [v1.x / v2.x, v1.y / v2.y, v1.z / v2.z]
impl std::ops::Div<Vector3isize> for &Vector3isize {
    type Output = Vector3isize;

    fn div(self, rhs: Vector3isize) -> Self::Output {
        (*self).div(rhs)
    }
}

/// Components wise division
/// v1 / v2 = [v1.x / v2.x, v1.y / v2.y, v1.z / v2.z]
impl std::ops::Div<&Vector3isize> for &Vector3isize {
    type Output = Vector3isize;

    fn div(self, rhs: &Vector3isize) -> Self::Output {
        (*self).div(*rhs)
    }
}

/// Divides all components of the vector by `rhs`
/// v / f = [v.x / f, v.y / f, v.z / f]
impl std::ops::Div<isize> for Vector3isize {
    type Output = Vector3isize;

    fn div(self, rhs: isize) -> Self::Output {
        Vector3isize {
            data: self.data / isizex4::from_array([rhs, rhs, rhs, 1]),
        }
    }
}

/// Divides all components of the vector by `rhs`
/// v / f = [v.x / f, v.y / f, v.z / f]
impl std::ops::Div<isize> for &Vector3isize {
    type Output = Vector3isize;

    fn div(self, rhs: isize) -> Self::Output {
        (*self).div(rhs)
    }
}

/// Divides all components of the vector by `rhs`
/// v / f = [v.x / f, v.y / f, v.z / f]
impl std::ops::Div<&isize> for Vector3isize {
    type Output = Vector3isize;

    fn div(self, rhs: &isize) -> Self::Output {
        self.div(*rhs)
    }
}

/// Divides all components of the vector by `rhs`
/// v / f = [v.x / f, v.y / f, v.z / f]
impl std::ops::Div<&isize> for &Vector3isize {
    type Output = Vector3isize;

    fn div(self, rhs: &isize) -> Self::Output {
        (*self).div(*rhs)
    }
}

//////////////////////////////////////////////////////////
//////////////      vector assign       //////////////////
//////////////////////////////////////////////////////////

/// Components wise addition
/// v1 += v2 <=> v1 = [v1.x + v2.x, v1.y + v2.y, v1.z + v2.z]
impl std::ops::AddAssign<Vector3isize> for Vector3isize {
    fn add_assign(&mut self, rhs: Vector3isize) {
        self.data = self.data + rhs.data;
    }
}

/// Components wise addition
/// v1 += v2 <=> v1 = [v1.x + v2.x, v1.y + v2.y, v1.z + v2.z]
impl std::ops::AddAssign<&Vector3isize> for Vector3isize {
    fn add_assign(&mut self, rhs: &Vector3isize) {
        self.add_assign(*rhs);
    }
}

/// Adds `rhs` to all components of the vector
/// v += f <=> v = [v.x + f, v.y + f]
impl std::ops::AddAssign<isize> for Vector3isize {
    fn add_assign(&mut self, rhs: isize) {
        self.data = self.data + isizex4::from_array([rhs, rhs, rhs, 0]);
    }
}

/// Adds `rhs` to all components of the vector
/// v += f <=> v = [v.x + f, v.y + f]
impl std::ops::AddAssign<&isize> for Vector3isize {
    fn add_assign(&mut self, rhs: &isize) {
        self.add_assign(*rhs);
    }
}

/// Components wise substraction
/// v1 -= v2 <=> v1 = [v1.x - v2.x, v1.y - v2.y, v1.z - v2.z]
impl std::ops::SubAssign<Vector3isize> for Vector3isize {
    fn sub_assign(&mut self, rhs: Vector3isize) {
        self.data = self.data - rhs.data;
    }
}

/// Components wise substraction
/// v1 -= v2 <=> v1 = [v1.x - v2.x, v1.y - v2.y, v1.z - v2.z]
impl std::ops::SubAssign<&Vector3isize> for Vector3isize {
    fn sub_assign(&mut self, rhs: &Vector3isize) {
        self.sub_assign(*rhs);
    }
}

/// Substracts `rhs` to all components of the vector
/// v -= f <=> v = [v.x - f, v.y - f, v.z - f]
impl std::ops::SubAssign<isize> for Vector3isize {
    fn sub_assign(&mut self, rhs: isize) {
        self.data = self.data - isizex4::from_array([rhs, rhs, rhs, 0]);
    }
}

/// Substracts `rhs` to all components of the vector
/// v -= f <=> v = [v.x - f, v.y - f, v.z - f]
impl std::ops::SubAssign<&isize> for Vector3isize {
    fn sub_assign(&mut self, rhs: &isize) {
        self.sub_assign(*rhs);
    }
}

/// Components wise multiplication
/// v1 *= v2 <=> v1 = [v1.x * v2.x, v1.y * v2.y, v1.z * v2.z]
impl std::ops::MulAssign<Vector3isize> for Vector3isize {
    fn mul_assign(&mut self, rhs: Vector3isize) {
        self.data = self.data * rhs.data;
    }
}

/// Components wise multiplication
/// v1 *= v2 <=> v1 = [v1.x * v2.x, v1.y * v2.y, v1.z * v2.z]
impl std::ops::MulAssign<&Vector3isize> for Vector3isize {
    fn mul_assign(&mut self, rhs: &Vector3isize) {
        self.mul_assign(*rhs);
    }
}

/// Multiplies all components of the vector by `rhs`
/// v *= f <=> v = [v.x * f, v.y * f, v.z * f]
impl std::ops::MulAssign<isize> for Vector3isize {
    fn mul_assign(&mut self, rhs: isize) {
        self.data = self.data * isizex4::from_array([rhs, rhs, rhs, 1]);
    }
}

/// Multiplies all components of the vector by `rhs`
/// v *= f <=> v = [v.x * f, v.y * f, v.z * f]
impl std::ops::MulAssign<&isize> for Vector3isize {
    fn mul_assign(&mut self, rhs: &isize) {
        self.mul_assign(*rhs);
    }
}

/// Components wise division
/// v1 /= v2 <=> v1 = [v1.x / v2.x, v1.y / v2.y, v1.z / v2.z]
impl std::ops::DivAssign<Vector3isize> for Vector3isize {
    fn div_assign(&mut self, rhs: Vector3isize) {
        self.data = self.data / rhs.data;
    }
}

/// Components wise division
/// v1 /= v2 <=> v1 = [v1.x / v2.x, v1.y / v2.y, v1.z / v2.z]
impl std::ops::DivAssign<&Vector3isize> for Vector3isize {
    fn div_assign(&mut self, rhs: &Vector3isize) {
        self.div_assign(*rhs);
    }
}

/// Divides all components of the vector by `rhs`
/// v /= f <=> v1 = [v.x / f, v.y / f, v.z / f]
impl std::ops::DivAssign<isize> for Vector3isize {
    fn div_assign(&mut self, rhs: isize) {
        self.data = self.data / isizex4::from_array([rhs, rhs, rhs, 1]);
    }
}

/// Divides all components of the vector by `rhs`
/// v /= f <=> v = [v.x / f, v.y / f, v.z / f]
impl std::ops::DivAssign<&isize> for Vector3isize {
    fn div_assign(&mut self, rhs: &isize) {
        self.div_assign(*rhs);
    }
}

//////////////////////////////////////////////////////////
///////////////     vector indices     ///////////////////
//////////////////////////////////////////////////////////
impl std::ops::Index<usize> for Vector3isize {
    type Output = isize;
    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Index out of bounds"),
        }
    }
}

impl std::ops::IndexMut<usize> for Vector3isize {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
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

    /// Tests 3 dimensional vector initialization
    #[test]
    fn initialization() {
        let v1 = vec3isize(1, 1, 1);
        let v2 = Vector3isize::ONES;
        assert_eq!(v1, v2);
    }

    /// Tests 3 dimensional vector operators
    #[test]
    fn operators() {
        let v1 = vec3isize(2, 3, 1);
        let v2 = vec3isize(4, 5, 6);
        assert_eq!(v1 + v2, vec3isize(6, 8, 7));
        let mut v3 = v1;
        assert_eq!(v1, v3);
        v3 += v1;
        assert_eq!(v3, vec3isize(4, 6, 2));

        v3 += 1;
        assert_eq!(v3, vec3isize(5, 7, 3));
        assert_eq!(v3 + v3 - v3, v3);

        assert_eq!(v3 * 2, vec3isize(10, 14, 6));
        assert_eq!(v1 * v2, vec3isize(8, 15, 6));

        assert_eq!(v3 / v3, Vector3isize::ONES);
        v3 /= v3;
        assert_eq!(v3, Vector3isize::ONES);
    }

    /// Tests 3 dimensional vector operations
    #[test]
    fn operations() {
        let v1 = vec3isize(1, 2, 4);
        let v2 = vec3isize(4, 3, 5);
        assert_eq!(Vector3isize::dot(&v1, &v2), 30);
        assert_eq!(Vector3isize::cross(&v1, &v2), vec3isize(-2, 11, -5));
        assert_eq!(
            Vector3isize::cross(&v2, &v1),
            -Vector3isize::cross(&v1, &v2)
        );

        let vx = Vector3isize::X;
        let vy = Vector3isize::Y;
        let vz = Vector3isize::Z;
        assert_eq!(Vector3isize::dot(&vx, &vy), 0);

        assert_eq!(vx, vec3isize(1, 0, 0));
        assert_eq!(vy, vec3isize(0, 1, 0));
        assert_eq!(vz, vec3isize(0, 0, 1));

        assert_eq!(Vector3isize::cross(&vx, &vy), vz);
        assert_eq!(Vector3isize::cross(&vy, &vx), Vector3isize::NEG_Z);

        assert_eq!(Vector3isize::cross(&vy, &vz), vx);
        assert_eq!(Vector3isize::cross(&vz, &vy), Vector3isize::NEG_X);

        assert_eq!(Vector3isize::cross(&vz, &vx), vy);
        assert_eq!(Vector3isize::cross(&vx, &vz), Vector3isize::NEG_Y);

        assert_eq!(vx.length(), 1.);
        assert_eq!(vy.length(), 1.);
        assert_eq!(vz.length(), 1.);
        assert_eq!(v1.length(), 21f32.sqrt());
    }

    /// Tests 3 dimensional vector's fields getters and setters
    #[test]
    fn deref() {
        let v1 = vec3isize(5, 2, 1);
        assert_eq!(v1.x, 5);
        assert_eq!(v1.y, 2);
        assert_eq!(v1.z, 1);
        let mut v2 = Vector3isize::ZEROS;
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
        let v1 = vec3isize(3, 4, 6);
        assert_eq!(v1.to_string(), "(3, 4, 6)");
        assert_eq!(format!("{:?}", v1), "Vector3isize { x: 3, y: 4, z: 6 }");
    }

    /// Tests indices access
    #[test]
    fn indices() {
        let mut v1 = vec3isize(2, 3, 1);
        assert_eq!(v1[2], 1);
        v1[1] = 2;
        assert_eq!(v1[1], 2);
    }
}
