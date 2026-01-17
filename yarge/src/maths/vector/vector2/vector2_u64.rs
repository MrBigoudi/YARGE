use std::simd::prelude::*;

/// A structure to represent a 2 dimensional u64 vector
#[derive(Clone, Copy, PartialEq)]
#[repr(C)]
pub struct Vector2u64 {
    data: u64x2,
}

/// A structure to be able to create `.x` and `.y` getters and setters
mod private {
    #[repr(C)]
    pub struct CoordsVector2u64 {
        pub x: u64,
        pub y: u64,
    }
}

/// Implements `Deref` to allow accessing `.x` and `.y`
impl std::ops::Deref for Vector2u64 {
    type Target = private::CoordsVector2u64;

    fn deref(&self) -> &Self::Target {
        let value: *const Vector2u64 = self;
        unsafe { &*(value as *const private::CoordsVector2u64) }
    }
}

/// Implements `DerefMut` to allow modifying `.x` and `.y`
impl std::ops::DerefMut for Vector2u64 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        let value: *mut Vector2u64 = self;
        unsafe { &mut *(value as *mut private::CoordsVector2u64) }
    }
}

/// Overrides the debug trait
impl std::fmt::Debug for Vector2u64 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Vector2u64")
            .field("x", &self.x)
            .field("y", &self.y)
            .finish()
    }
}

/// Overrides the display trait
impl std::fmt::Display for Vector2u64 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

/// Sets a 2 dimensional u64 vector to `[0, 0]` as default
impl Default for Vector2u64 {
    fn default() -> Self {
        Self::ZEROS
    }
}

/// Creates a 2 dimensional u64 vector
pub const fn vec2u64(x: u64, y: u64) -> Vector2u64 {
    Vector2u64::new(x, y)
}

/// A union to cast simd to array and allow const construction
union UnionCast {
    array: [u64; 2],
    simd: Vector2u64,
}

impl Vector2u64 {
    //////////////////////////////////////////////////////////
    /////////////      vector creation       /////////////////
    //////////////////////////////////////////////////////////

    /// Creates a new vector given its coordinates
    pub const fn new(x: u64, y: u64) -> Self {
        unsafe { UnionCast { array: [x, y] }.simd }
    }

    /// Creates a new vector with all coordinates set to `value`
    const fn splat(value: u64) -> Self {
        Self::new(value, value)
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
    pub const X: Self = Self::new(1, 0);

    /// Creates a new vector pointing along the positive Y axis
    pub const Y: Self = Self::new(0, 1);

    //////////////////////////////////////////////////////////
    /////////////     vector operations      /////////////////
    //////////////////////////////////////////////////////////

    /// Sums up the element of the vector
    pub fn prefix_sum(self) -> u64 {
        self.x + self.y
    }

    /// Dot product between two vectors
    pub fn dot(v1: &Vector2u64, v2: &Vector2u64) -> u64 {
        (v1 * v2).prefix_sum()
    }

    /// Returns the length of the vector
    pub fn length(&self) -> f64 {
        (Self::dot(self, self) as f64).sqrt()
    }

    /// Const accessor, only used for matrix initialization
    pub(in crate::maths) const fn x_const(&self) -> u64 {
        self.data.as_array()[0]
    }
    /// Const accessor, only used for matrix initialization
    pub(in crate::maths) const fn y_const(&self) -> u64 {
        self.data.as_array()[1]
    }
}

//////////////////////////////////////////////////////////
//////////////     vector additions     //////////////////
//////////////////////////////////////////////////////////

/// Components wise addition
/// v1 + v2 = [v1.x + v2.x, v1.y + v2.y]
impl std::ops::Add<Vector2u64> for Vector2u64 {
    type Output = Vector2u64;

    fn add(self, rhs: Vector2u64) -> Self::Output {
        Vector2u64 {
            data: self.data + rhs.data,
        }
    }
}

/// Components wise addition
/// v1 + v2 = [v1.x + v2.x, v1.y + v2.y]
impl std::ops::Add<&Vector2u64> for Vector2u64 {
    type Output = Vector2u64;

    fn add(self, rhs: &Vector2u64) -> Self::Output {
        self.add(*rhs)
    }
}

/// Components wise addition
/// v1 + v2 = [v1.x + v2.x, v1.y + v2.y]
impl std::ops::Add<Vector2u64> for &Vector2u64 {
    type Output = Vector2u64;

    fn add(self, rhs: Vector2u64) -> Self::Output {
        (*self).add(rhs)
    }
}

/// Components wise addition
/// v1 + v2 = [v1.x + v2.x, v1.y + v2.y]
impl std::ops::Add<&Vector2u64> for &Vector2u64 {
    type Output = Vector2u64;

    fn add(self, rhs: &Vector2u64) -> Self::Output {
        (*self).add(*rhs)
    }
}

/// Adds `rhs` to all components of the vector
/// v + f = [v.x + f, v.y + f]
impl std::ops::Add<u64> for Vector2u64 {
    type Output = Vector2u64;

    fn add(self, rhs: u64) -> Self::Output {
        Vector2u64 {
            data: self.data + u64x2::from_array([rhs, rhs]),
        }
    }
}

/// Adds `rhs` to all components of the vector
/// v + f = [v.x + f, v.y + f]
impl std::ops::Add<u64> for &Vector2u64 {
    type Output = Vector2u64;

    fn add(self, rhs: u64) -> Self::Output {
        (*self).add(rhs)
    }
}

/// Adds `rhs` to all components of the vector
/// v + f = [v.x + f, v.y + f]
impl std::ops::Add<&u64> for Vector2u64 {
    type Output = Vector2u64;

    fn add(self, rhs: &u64) -> Self::Output {
        self.add(*rhs)
    }
}

/// Adds `rhs` to all components of the vector
/// v + f = [v.x + f, v.y + f]
impl std::ops::Add<&u64> for &Vector2u64 {
    type Output = Vector2u64;

    fn add(self, rhs: &u64) -> Self::Output {
        (*self).add(*rhs)
    }
}

//////////////////////////////////////////////////////////
//////////////   vector substractions    /////////////////
//////////////////////////////////////////////////////////

/// Components wise substraction
/// v1 - v2 = [v1.x - v2.x, v1.y - v2.y]
impl std::ops::Sub<Vector2u64> for Vector2u64 {
    type Output = Vector2u64;

    fn sub(self, rhs: Vector2u64) -> Self::Output {
        Vector2u64 {
            data: self.data - rhs.data,
        }
    }
}

/// Components wise substraction
/// v1 - v2 = [v1.x - v2.x, v1.y - v2.y]
impl std::ops::Sub<&Vector2u64> for Vector2u64 {
    type Output = Vector2u64;

    fn sub(self, rhs: &Vector2u64) -> Self::Output {
        self.sub(*rhs)
    }
}

/// Components wise substraction
/// v1 - v2 = [v1.x - v2.x, v1.y - v2.y]
impl std::ops::Sub<Vector2u64> for &Vector2u64 {
    type Output = Vector2u64;

    fn sub(self, rhs: Vector2u64) -> Self::Output {
        (*self).sub(rhs)
    }
}

/// Components wise substraction
/// v1 - v2 = [v1.x - v2.x, v1.y - v2.y]
impl std::ops::Sub<&Vector2u64> for &Vector2u64 {
    type Output = Vector2u64;

    fn sub(self, rhs: &Vector2u64) -> Self::Output {
        (*self).sub(*rhs)
    }
}

/// Substract `rhs` to all components of the vector
/// v - f = [v.x - f, v.y - f]
impl std::ops::Sub<u64> for Vector2u64 {
    type Output = Vector2u64;

    fn sub(self, rhs: u64) -> Self::Output {
        Vector2u64 {
            data: self.data - u64x2::from_array([rhs, rhs]),
        }
    }
}

/// Substracts `rhs` to all components of the vector
/// v - f = [v.x - f, v.y - f]
impl std::ops::Sub<u64> for &Vector2u64 {
    type Output = Vector2u64;

    fn sub(self, rhs: u64) -> Self::Output {
        (*self).sub(rhs)
    }
}

/// Substracts `rhs` to all components of the vector
/// v - f = [v.x - f, v.y - f]
impl std::ops::Sub<&u64> for Vector2u64 {
    type Output = Vector2u64;

    fn sub(self, rhs: &u64) -> Self::Output {
        self.sub(*rhs)
    }
}

/// Substracts `rhs` to all components of the vector
/// v - f = [v.x - f, v.y - f]
impl std::ops::Sub<&u64> for &Vector2u64 {
    type Output = Vector2u64;

    fn sub(self, rhs: &u64) -> Self::Output {
        (*self).sub(*rhs)
    }
}

//////////////////////////////////////////////////////////
//////////////   vector multiplications   ////////////////
//////////////////////////////////////////////////////////

/// Components wise multiplication
/// v1 * v2 = [v1.x * v2.x, v1.y * v2.y]
impl std::ops::Mul<Vector2u64> for Vector2u64 {
    type Output = Vector2u64;

    fn mul(self, rhs: Vector2u64) -> Self::Output {
        Vector2u64 {
            data: self.data * rhs.data,
        }
    }
}

/// Components wise multiplication
/// v1 * v2 = [v1.x * v2.x, v1.y * v2.y]
impl std::ops::Mul<&Vector2u64> for Vector2u64 {
    type Output = Vector2u64;

    fn mul(self, rhs: &Vector2u64) -> Self::Output {
        self.mul(*rhs)
    }
}

/// Components wise multiplication
/// v1 * v2 = [v1.x * v2.x, v1.y * v2.y]
impl std::ops::Mul<Vector2u64> for &Vector2u64 {
    type Output = Vector2u64;

    fn mul(self, rhs: Vector2u64) -> Self::Output {
        (*self).mul(rhs)
    }
}

/// Components wise multiplication
/// v1 * v2 = [v1.x * v2.x, v1.y * v2.y]
impl std::ops::Mul<&Vector2u64> for &Vector2u64 {
    type Output = Vector2u64;

    fn mul(self, rhs: &Vector2u64) -> Self::Output {
        (*self).mul(*rhs)
    }
}

/// Multiplies all components of the vector by `rhs`
/// v * f = [v.x * f, v.y * f]
impl std::ops::Mul<u64> for Vector2u64 {
    type Output = Vector2u64;

    fn mul(self, rhs: u64) -> Self::Output {
        Vector2u64 {
            data: self.data * u64x2::from_array([rhs, rhs]),
        }
    }
}

/// Multiplies all components of the vector by `rhs`
/// v * f = [v.x * f, v.y * f]
impl std::ops::Mul<u64> for &Vector2u64 {
    type Output = Vector2u64;

    fn mul(self, rhs: u64) -> Self::Output {
        (*self).mul(rhs)
    }
}

/// Multiplies all components of the vector by `rhs`
/// v * f = [v.x * f, v.y * f]
impl std::ops::Mul<&u64> for Vector2u64 {
    type Output = Vector2u64;

    fn mul(self, rhs: &u64) -> Self::Output {
        self.mul(*rhs)
    }
}

/// Multiplies all components of the vector by `rhs`
/// v * f = [v.x * f, v.y * f]
impl std::ops::Mul<&u64> for &Vector2u64 {
    type Output = Vector2u64;

    fn mul(self, rhs: &u64) -> Self::Output {
        (*self).mul(*rhs)
    }
}

//////////////////////////////////////////////////////////
//////////////     vector divisions     //////////////////
//////////////////////////////////////////////////////////

/// Components wise division
/// v1 / v2 = [v1.x / v2.x, v1.y / v2.y]
impl std::ops::Div<Vector2u64> for Vector2u64 {
    type Output = Vector2u64;

    fn div(self, rhs: Vector2u64) -> Self::Output {
        Vector2u64 {
            data: self.data / rhs.data,
        }
    }
}

/// Components wise division
/// v1 / v2 = [v1.x / v2.x, v1.y / v2.y]
impl std::ops::Div<&Vector2u64> for Vector2u64 {
    type Output = Vector2u64;

    fn div(self, rhs: &Vector2u64) -> Self::Output {
        self.div(*rhs)
    }
}

/// Components wise division
/// v1 / v2 = [v1.x / v2.x, v1.y / v2.y]
impl std::ops::Div<Vector2u64> for &Vector2u64 {
    type Output = Vector2u64;

    fn div(self, rhs: Vector2u64) -> Self::Output {
        (*self).div(rhs)
    }
}

/// Components wise division
/// v1 / v2 = [v1.x / v2.x, v1.y / v2.y]
impl std::ops::Div<&Vector2u64> for &Vector2u64 {
    type Output = Vector2u64;

    fn div(self, rhs: &Vector2u64) -> Self::Output {
        (*self).div(*rhs)
    }
}

/// Divides all components of the vector by `rhs`
/// v / f = [v.x / f, v.y / f]
impl std::ops::Div<u64> for Vector2u64 {
    type Output = Vector2u64;

    fn div(self, rhs: u64) -> Self::Output {
        Vector2u64 {
            data: self.data / u64x2::from_array([rhs, rhs]),
        }
    }
}

/// Divides all components of the vector by `rhs`
/// v / f = [v.x / f, v.y / f]
impl std::ops::Div<u64> for &Vector2u64 {
    type Output = Vector2u64;

    fn div(self, rhs: u64) -> Self::Output {
        (*self).div(rhs)
    }
}

/// Divides all components of the vector by `rhs`
/// v / f = [v.x / f, v.y / f]
impl std::ops::Div<&u64> for Vector2u64 {
    type Output = Vector2u64;

    fn div(self, rhs: &u64) -> Self::Output {
        self.div(*rhs)
    }
}

/// Divides all components of the vector by `rhs`
/// v / f = [v.x / f, v.y / f]
impl std::ops::Div<&u64> for &Vector2u64 {
    type Output = Vector2u64;

    fn div(self, rhs: &u64) -> Self::Output {
        (*self).div(*rhs)
    }
}

//////////////////////////////////////////////////////////
//////////////      vector assign       //////////////////
//////////////////////////////////////////////////////////

/// Components wise addition
/// v1 += v2 <=> v1 = [v1.x + v2.x, v1.y + v2.y]
impl std::ops::AddAssign<Vector2u64> for Vector2u64 {
    fn add_assign(&mut self, rhs: Vector2u64) {
        self.data = self.data + rhs.data;
    }
}

/// Components wise addition
/// v1 += v2 <=> v1 = [v1.x + v2.x, v1.y + v2.y]
impl std::ops::AddAssign<&Vector2u64> for Vector2u64 {
    fn add_assign(&mut self, rhs: &Vector2u64) {
        self.add_assign(*rhs);
    }
}

/// Adds `rhs` to all components of the vector
/// v += f <=> v = [v.x + f, v.y + f]
impl std::ops::AddAssign<u64> for Vector2u64 {
    fn add_assign(&mut self, rhs: u64) {
        self.data = self.data + u64x2::from_array([rhs, rhs]);
    }
}

/// Adds `rhs` to all components of the vector
/// v += f <=> v = [v.x + f, v.y + f]
impl std::ops::AddAssign<&u64> for Vector2u64 {
    fn add_assign(&mut self, rhs: &u64) {
        self.add_assign(*rhs);
    }
}

/// Components wise substraction
/// v1 -= v2 <=> v1 = [v1.x - v2.x, v1.y - v2.y]
impl std::ops::SubAssign<Vector2u64> for Vector2u64 {
    fn sub_assign(&mut self, rhs: Vector2u64) {
        self.data = self.data - rhs.data;
    }
}

/// Components wise substraction
/// v1 -= v2 <=> v1 = [v1.x - v2.x, v1.y - v2.y]
impl std::ops::SubAssign<&Vector2u64> for Vector2u64 {
    fn sub_assign(&mut self, rhs: &Vector2u64) {
        self.sub_assign(*rhs);
    }
}

/// Substracts `rhs` to all components of the vector
/// v -= f <=> v = [v.x - f, v.y - f]
impl std::ops::SubAssign<u64> for Vector2u64 {
    fn sub_assign(&mut self, rhs: u64) {
        self.data = self.data - u64x2::from_array([rhs, rhs]);
    }
}

/// Substracts `rhs` to all components of the vector
/// v -= f <=> v = [v.x - f, v.y - f]
impl std::ops::SubAssign<&u64> for Vector2u64 {
    fn sub_assign(&mut self, rhs: &u64) {
        self.sub_assign(*rhs);
    }
}

/// Components wise multiplication
/// v1 *= v2 <=> v1 = [v1.x * v2.x, v1.y * v2.y]
impl std::ops::MulAssign<Vector2u64> for Vector2u64 {
    fn mul_assign(&mut self, rhs: Vector2u64) {
        self.data = self.data * rhs.data;
    }
}

/// Components wise multiplication
/// v1 *= v2 <=> v1 = [v1.x * v2.x, v1.y * v2.y]
impl std::ops::MulAssign<&Vector2u64> for Vector2u64 {
    fn mul_assign(&mut self, rhs: &Vector2u64) {
        self.mul_assign(*rhs);
    }
}

/// Multiplies all components of the vector by `rhs`
/// v *= f <=> v = [v.x * f, v.y * f]
impl std::ops::MulAssign<u64> for Vector2u64 {
    fn mul_assign(&mut self, rhs: u64) {
        self.data = self.data * u64x2::from_array([rhs, rhs]);
    }
}

/// Multiplies all components of the vector by `rhs`
/// v *= f <=> v = [v.x * f, v.y * f]
impl std::ops::MulAssign<&u64> for Vector2u64 {
    fn mul_assign(&mut self, rhs: &u64) {
        self.mul_assign(*rhs);
    }
}

/// Components wise division
/// v1 /= v2 <=> v1 = [v1.x / v2.x, v1.y / v2.y]
impl std::ops::DivAssign<Vector2u64> for Vector2u64 {
    fn div_assign(&mut self, rhs: Vector2u64) {
        self.data = self.data / rhs.data;
    }
}

/// Components wise division
/// v1 /= v2 <=> v1 = [v1.x / v2.x, v1.y / v2.y]
impl std::ops::DivAssign<&Vector2u64> for Vector2u64 {
    fn div_assign(&mut self, rhs: &Vector2u64) {
        self.div_assign(*rhs);
    }
}

/// Divides all components of the vector by `rhs`
/// v /= f <=> v1 = [v.x / f, v.y / f]
impl std::ops::DivAssign<u64> for Vector2u64 {
    fn div_assign(&mut self, rhs: u64) {
        self.data = self.data / u64x2::from_array([rhs, rhs]);
    }
}

/// Divides all components of the vector by `rhs`
/// v /= f <=> v = [v.x / f, v.y / f]
impl std::ops::DivAssign<&u64> for Vector2u64 {
    fn div_assign(&mut self, rhs: &u64) {
        self.div_assign(*rhs);
    }
}

//////////////////////////////////////////////////////////
///////////////     vector indices     ///////////////////
//////////////////////////////////////////////////////////
impl std::ops::Index<usize> for Vector2u64 {
    type Output = u64;
    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            _ => panic!("Index out of bounds"),
        }
    }
}

impl std::ops::IndexMut<usize> for Vector2u64 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
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

    /// Tests 2 dimensional vector initialization
    #[test]
    fn initialization() {
        let v1 = vec2u64(1, 1);
        let v2 = Vector2u64::ONES;
        assert_eq!(v1, v2);
    }

    /// Tests 2 dimensional vector operators
    #[test]
    fn operators() {
        let v1 = vec2u64(2, 3);
        let v2 = vec2u64(4, 5);
        assert_eq!(v1 + v2, vec2u64(6, 8));
        let mut v3 = v1;
        assert_eq!(v1, v3);
        v3 += v1;
        assert_eq!(v3, vec2u64(4, 6));

        v3 += 1;
        assert_eq!(v3, vec2u64(5, 7));
        assert_eq!(v3 + v3 - v3, v3);

        assert_eq!(v3 * 2, vec2u64(10, 14));
        assert_eq!(v1 * v2, vec2u64(8, 15));

        assert_eq!(v3 / v3, Vector2u64::ONES);
        v3 /= v3;
        assert_eq!(v3, Vector2u64::ONES);
    }

    /// Tests 2 dimensional vector operations
    #[test]
    fn operations() {
        let v1 = vec2u64(1, 2);
        let v2 = vec2u64(4, 3);
        assert_eq!(Vector2u64::dot(&v1, &v2), 10);
        let vx = Vector2u64::X;
        let vy = Vector2u64::Y;
        assert_eq!(Vector2u64::dot(&vx, &vy), 0);

        assert_eq!(vx.length(), 1.);
        assert_eq!(vy.length(), 1.);
        assert_eq!(v1.length(), 5f64.sqrt());
    }

    /// Tests 2 dimensional vector's fields getters and setters
    #[test]
    fn deref() {
        let v1 = vec2u64(5, 2);
        assert_eq!(v1.x, 5);
        assert_eq!(v1.y, 2);
        let mut v2 = Vector2u64::ZEROS;
        assert_eq!(v2.x, 0);
        assert_eq!(v2.y, 0);
        v2.x = 1;
        assert_eq!(v2.x, 1);
        v2.y = 3;
        assert_eq!(v2.y, 3);
        assert_eq!(v2.x, 1);
    }

    /// Tests the formatting of 2 dimensional vectors
    #[test]
    fn format() {
        let v1 = vec2u64(3, 4);
        assert_eq!(v1.to_string(), "(3, 4)");
        assert_eq!(format!("{:?}", v1), "Vector2u64 { x: 3, y: 4 }");
    }

    /// Tests indices access
    #[test]
    fn indices() {
        let mut v1 = vec2u64(2, 3);
        assert_eq!(v1[1], 3);
        v1[0] = 4;
        assert_eq!(v1[0], 4);
    }
}
