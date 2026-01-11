use std::simd::prelude::*;

/// A structure to represent a 2 dimensional u8 vector
#[derive(Clone, Copy, PartialEq)]
#[repr(C)]
pub struct Vector2u8 {
    data: u8x2,
}

/// A structure to be able to create `.x` and `.y` getters and setters
mod private {
    #[repr(C)]
    pub struct CoordsVector2u8 {
        pub x: u8,
        pub y: u8,
    }
}

/// Implements `Deref` to allow accessing `.x` and `.y`
impl std::ops::Deref for Vector2u8 {
    type Target = private::CoordsVector2u8;

    fn deref(&self) -> &Self::Target {
        let value: *const Vector2u8 = self;
        unsafe { &*(value as *const private::CoordsVector2u8) }
    }
}

/// Implements `DerefMut` to allow modifying `.x` and `.y`
impl std::ops::DerefMut for Vector2u8 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        let value: *mut Vector2u8 = self;
        unsafe { &mut *(value as *mut private::CoordsVector2u8) }
    }
}

/// Overrides the debug trait
impl std::fmt::Debug for Vector2u8 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Vector2u8")
            .field("x", &self.x)
            .field("y", &self.y)
            .finish()
    }
}

/// Overrides the display trait
impl std::fmt::Display for Vector2u8 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

/// Sets a 2 dimensional u8 vector to `[0, 0]` as default
impl Default for Vector2u8 {
    fn default() -> Self {
        Self::ZEROS
    }
}

/// Creates a 2 dimensional u8 vector
pub fn vec2u8(x: u8, y: u8) -> Vector2u8 {
    Vector2u8::new(x, y)
}

/// A union to cast simd to array and allow const construction
union UnionCast {
    array: [u8; 2],
    simd: Vector2u8,
}

impl Vector2u8 {
    //////////////////////////////////////////////////////////
    /////////////      vector creation       /////////////////
    //////////////////////////////////////////////////////////

    /// Creates a new vector given its coordinates
    pub const fn new(x: u8, y: u8) -> Self {
        unsafe { UnionCast { array: [x, y] }.simd }
    }

    /// Creates a new vector with all coordinates set to `value`
    const fn splat(value: u8) -> Self {
        Self::new(value, value)
    }

    /// Creates a new vector filled with `values`
    pub const fn filled(value: u8) -> Self {
        Self::splat(value)
    }

    /// Creates a new vector filled with ones
    pub const ONES: Self = Self::splat(1);

    /// Creates a new vector filled with zeros
    pub const ZEROS: Self = Self::splat(0);

    /// Creates a new vector filled with `u8::MIN``
    pub const MIN: Self = Self::splat(u8::MIN);

    /// Creates a new vector filled with `u8::MAX`
    pub const MAX: Self = Self::splat(u8::MAX);

    /// Creates a new vector pointing along the positive X axis
    pub const X: Self = Self::new(1, 0);

    /// Creates a new vector pointing along the positive Y axis
    pub const Y: Self = Self::new(0, 1);

    //////////////////////////////////////////////////////////
    /////////////     vector operations      /////////////////
    //////////////////////////////////////////////////////////

    /// Sums up the element of the vector
    pub fn prefix_sum(self) -> u8 {
        self.x + self.y
    }

    /// Dot product between two vectors
    pub fn dot(v1: &Vector2u8, v2: &Vector2u8) -> u8 {
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
/// v1 + v2 = [v1.x + v2.x, v1.y + v2.y]
impl std::ops::Add<Vector2u8> for Vector2u8 {
    type Output = Vector2u8;

    fn add(self, rhs: Vector2u8) -> Self::Output {
        Vector2u8 {
            data: self.data + rhs.data,
        }
    }
}

/// Components wise addition
/// v1 + v2 = [v1.x + v2.x, v1.y + v2.y]
impl std::ops::Add<&Vector2u8> for Vector2u8 {
    type Output = Vector2u8;

    fn add(self, rhs: &Vector2u8) -> Self::Output {
        self.add(*rhs)
    }
}

/// Components wise addition
/// v1 + v2 = [v1.x + v2.x, v1.y + v2.y]
impl std::ops::Add<Vector2u8> for &Vector2u8 {
    type Output = Vector2u8;

    fn add(self, rhs: Vector2u8) -> Self::Output {
        (*self).add(rhs)
    }
}

/// Components wise addition
/// v1 + v2 = [v1.x + v2.x, v1.y + v2.y]
impl std::ops::Add<&Vector2u8> for &Vector2u8 {
    type Output = Vector2u8;

    fn add(self, rhs: &Vector2u8) -> Self::Output {
        (*self).add(*rhs)
    }
}

/// Adds `rhs` to all components of the vector
/// v + f = [v.x + f, v.y + f]
impl std::ops::Add<u8> for Vector2u8 {
    type Output = Vector2u8;

    fn add(self, rhs: u8) -> Self::Output {
        Vector2u8 {
            data: self.data + u8x2::from_array([rhs, rhs]),
        }
    }
}

/// Adds `rhs` to all components of the vector
/// v + f = [v.x + f, v.y + f]
impl std::ops::Add<u8> for &Vector2u8 {
    type Output = Vector2u8;

    fn add(self, rhs: u8) -> Self::Output {
        (*self).add(rhs)
    }
}

/// Adds `rhs` to all components of the vector
/// v + f = [v.x + f, v.y + f]
impl std::ops::Add<&u8> for Vector2u8 {
    type Output = Vector2u8;

    fn add(self, rhs: &u8) -> Self::Output {
        self.add(*rhs)
    }
}

/// Adds `rhs` to all components of the vector
/// v + f = [v.x + f, v.y + f]
impl std::ops::Add<&u8> for &Vector2u8 {
    type Output = Vector2u8;

    fn add(self, rhs: &u8) -> Self::Output {
        (*self).add(*rhs)
    }
}

//////////////////////////////////////////////////////////
//////////////   vector substractions    /////////////////
//////////////////////////////////////////////////////////

/// Components wise substraction
/// v1 - v2 = [v1.x - v2.x, v1.y - v2.y]
impl std::ops::Sub<Vector2u8> for Vector2u8 {
    type Output = Vector2u8;

    fn sub(self, rhs: Vector2u8) -> Self::Output {
        Vector2u8 {
            data: self.data - rhs.data,
        }
    }
}

/// Components wise substraction
/// v1 - v2 = [v1.x - v2.x, v1.y - v2.y]
impl std::ops::Sub<&Vector2u8> for Vector2u8 {
    type Output = Vector2u8;

    fn sub(self, rhs: &Vector2u8) -> Self::Output {
        self.sub(*rhs)
    }
}

/// Components wise substraction
/// v1 - v2 = [v1.x - v2.x, v1.y - v2.y]
impl std::ops::Sub<Vector2u8> for &Vector2u8 {
    type Output = Vector2u8;

    fn sub(self, rhs: Vector2u8) -> Self::Output {
        (*self).sub(rhs)
    }
}

/// Components wise substraction
/// v1 - v2 = [v1.x - v2.x, v1.y - v2.y]
impl std::ops::Sub<&Vector2u8> for &Vector2u8 {
    type Output = Vector2u8;

    fn sub(self, rhs: &Vector2u8) -> Self::Output {
        (*self).sub(*rhs)
    }
}

/// Substract `rhs` to all components of the vector
/// v - f = [v.x - f, v.y - f]
impl std::ops::Sub<u8> for Vector2u8 {
    type Output = Vector2u8;

    fn sub(self, rhs: u8) -> Self::Output {
        Vector2u8 {
            data: self.data - u8x2::from_array([rhs, rhs]),
        }
    }
}

/// Substracts `rhs` to all components of the vector
/// v - f = [v.x - f, v.y - f]
impl std::ops::Sub<u8> for &Vector2u8 {
    type Output = Vector2u8;

    fn sub(self, rhs: u8) -> Self::Output {
        (*self).sub(rhs)
    }
}

/// Substracts `rhs` to all components of the vector
/// v - f = [v.x - f, v.y - f]
impl std::ops::Sub<&u8> for Vector2u8 {
    type Output = Vector2u8;

    fn sub(self, rhs: &u8) -> Self::Output {
        self.sub(*rhs)
    }
}

/// Substracts `rhs` to all components of the vector
/// v - f = [v.x - f, v.y - f]
impl std::ops::Sub<&u8> for &Vector2u8 {
    type Output = Vector2u8;

    fn sub(self, rhs: &u8) -> Self::Output {
        (*self).sub(*rhs)
    }
}

//////////////////////////////////////////////////////////
//////////////   vector multiplications   ////////////////
//////////////////////////////////////////////////////////

/// Components wise multiplication
/// v1 * v2 = [v1.x * v2.x, v1.y * v2.y]
impl std::ops::Mul<Vector2u8> for Vector2u8 {
    type Output = Vector2u8;

    fn mul(self, rhs: Vector2u8) -> Self::Output {
        Vector2u8 {
            data: self.data * rhs.data,
        }
    }
}

/// Components wise multiplication
/// v1 * v2 = [v1.x * v2.x, v1.y * v2.y]
impl std::ops::Mul<&Vector2u8> for Vector2u8 {
    type Output = Vector2u8;

    fn mul(self, rhs: &Vector2u8) -> Self::Output {
        self.mul(*rhs)
    }
}

/// Components wise multiplication
/// v1 * v2 = [v1.x * v2.x, v1.y * v2.y]
impl std::ops::Mul<Vector2u8> for &Vector2u8 {
    type Output = Vector2u8;

    fn mul(self, rhs: Vector2u8) -> Self::Output {
        (*self).mul(rhs)
    }
}

/// Components wise multiplication
/// v1 * v2 = [v1.x * v2.x, v1.y * v2.y]
impl std::ops::Mul<&Vector2u8> for &Vector2u8 {
    type Output = Vector2u8;

    fn mul(self, rhs: &Vector2u8) -> Self::Output {
        (*self).mul(*rhs)
    }
}

/// Multiplies all components of the vector by `rhs`
/// v * f = [v.x * f, v.y * f]
impl std::ops::Mul<u8> for Vector2u8 {
    type Output = Vector2u8;

    fn mul(self, rhs: u8) -> Self::Output {
        Vector2u8 {
            data: self.data * u8x2::from_array([rhs, rhs]),
        }
    }
}

/// Multiplies all components of the vector by `rhs`
/// v * f = [v.x * f, v.y * f]
impl std::ops::Mul<u8> for &Vector2u8 {
    type Output = Vector2u8;

    fn mul(self, rhs: u8) -> Self::Output {
        (*self).mul(rhs)
    }
}

/// Multiplies all components of the vector by `rhs`
/// v * f = [v.x * f, v.y * f]
impl std::ops::Mul<&u8> for Vector2u8 {
    type Output = Vector2u8;

    fn mul(self, rhs: &u8) -> Self::Output {
        self.mul(*rhs)
    }
}

/// Multiplies all components of the vector by `rhs`
/// v * f = [v.x * f, v.y * f]
impl std::ops::Mul<&u8> for &Vector2u8 {
    type Output = Vector2u8;

    fn mul(self, rhs: &u8) -> Self::Output {
        (*self).mul(*rhs)
    }
}

//////////////////////////////////////////////////////////
//////////////     vector divisions     //////////////////
//////////////////////////////////////////////////////////

/// Components wise division
/// v1 / v2 = [v1.x / v2.x, v1.y / v2.y]
impl std::ops::Div<Vector2u8> for Vector2u8 {
    type Output = Vector2u8;

    fn div(self, rhs: Vector2u8) -> Self::Output {
        Vector2u8 {
            data: self.data / rhs.data,
        }
    }
}

/// Components wise division
/// v1 / v2 = [v1.x / v2.x, v1.y / v2.y]
impl std::ops::Div<&Vector2u8> for Vector2u8 {
    type Output = Vector2u8;

    fn div(self, rhs: &Vector2u8) -> Self::Output {
        self.div(*rhs)
    }
}

/// Components wise division
/// v1 / v2 = [v1.x / v2.x, v1.y / v2.y]
impl std::ops::Div<Vector2u8> for &Vector2u8 {
    type Output = Vector2u8;

    fn div(self, rhs: Vector2u8) -> Self::Output {
        (*self).div(rhs)
    }
}

/// Components wise division
/// v1 / v2 = [v1.x / v2.x, v1.y / v2.y]
impl std::ops::Div<&Vector2u8> for &Vector2u8 {
    type Output = Vector2u8;

    fn div(self, rhs: &Vector2u8) -> Self::Output {
        (*self).div(*rhs)
    }
}

/// Divides all components of the vector by `rhs`
/// v / f = [v.x / f, v.y / f]
impl std::ops::Div<u8> for Vector2u8 {
    type Output = Vector2u8;

    fn div(self, rhs: u8) -> Self::Output {
        Vector2u8 {
            data: self.data / u8x2::from_array([rhs, rhs]),
        }
    }
}

/// Divides all components of the vector by `rhs`
/// v / f = [v.x / f, v.y / f]
impl std::ops::Div<u8> for &Vector2u8 {
    type Output = Vector2u8;

    fn div(self, rhs: u8) -> Self::Output {
        (*self).div(rhs)
    }
}

/// Divides all components of the vector by `rhs`
/// v / f = [v.x / f, v.y / f]
impl std::ops::Div<&u8> for Vector2u8 {
    type Output = Vector2u8;

    fn div(self, rhs: &u8) -> Self::Output {
        self.div(*rhs)
    }
}

/// Divides all components of the vector by `rhs`
/// v / f = [v.x / f, v.y / f]
impl std::ops::Div<&u8> for &Vector2u8 {
    type Output = Vector2u8;

    fn div(self, rhs: &u8) -> Self::Output {
        (*self).div(*rhs)
    }
}

//////////////////////////////////////////////////////////
//////////////      vector assign       //////////////////
//////////////////////////////////////////////////////////

/// Components wise addition
/// v1 += v2 <=> v1 = [v1.x + v2.x, v1.y + v2.y]
impl std::ops::AddAssign<Vector2u8> for Vector2u8 {
    fn add_assign(&mut self, rhs: Vector2u8) {
        self.data = self.data + rhs.data;
    }
}

/// Components wise addition
/// v1 += v2 <=> v1 = [v1.x + v2.x, v1.y + v2.y]
impl std::ops::AddAssign<&Vector2u8> for Vector2u8 {
    fn add_assign(&mut self, rhs: &Vector2u8) {
        self.add_assign(*rhs);
    }
}

/// Adds `rhs` to all components of the vector
/// v += f <=> v = [v.x + f, v.y + f]
impl std::ops::AddAssign<u8> for Vector2u8 {
    fn add_assign(&mut self, rhs: u8) {
        self.data = self.data + u8x2::from_array([rhs, rhs]);
    }
}

/// Adds `rhs` to all components of the vector
/// v += f <=> v = [v.x + f, v.y + f]
impl std::ops::AddAssign<&u8> for Vector2u8 {
    fn add_assign(&mut self, rhs: &u8) {
        self.add_assign(*rhs);
    }
}

/// Components wise substraction
/// v1 -= v2 <=> v1 = [v1.x - v2.x, v1.y - v2.y]
impl std::ops::SubAssign<Vector2u8> for Vector2u8 {
    fn sub_assign(&mut self, rhs: Vector2u8) {
        self.data = self.data - rhs.data;
    }
}

/// Components wise substraction
/// v1 -= v2 <=> v1 = [v1.x - v2.x, v1.y - v2.y]
impl std::ops::SubAssign<&Vector2u8> for Vector2u8 {
    fn sub_assign(&mut self, rhs: &Vector2u8) {
        self.sub_assign(*rhs);
    }
}

/// Substracts `rhs` to all components of the vector
/// v -= f <=> v = [v.x - f, v.y - f]
impl std::ops::SubAssign<u8> for Vector2u8 {
    fn sub_assign(&mut self, rhs: u8) {
        self.data = self.data - u8x2::from_array([rhs, rhs]);
    }
}

/// Substracts `rhs` to all components of the vector
/// v -= f <=> v = [v.x - f, v.y - f]
impl std::ops::SubAssign<&u8> for Vector2u8 {
    fn sub_assign(&mut self, rhs: &u8) {
        self.sub_assign(*rhs);
    }
}

/// Components wise multiplication
/// v1 *= v2 <=> v1 = [v1.x * v2.x, v1.y * v2.y]
impl std::ops::MulAssign<Vector2u8> for Vector2u8 {
    fn mul_assign(&mut self, rhs: Vector2u8) {
        self.data = self.data * rhs.data;
    }
}

/// Components wise multiplication
/// v1 *= v2 <=> v1 = [v1.x * v2.x, v1.y * v2.y]
impl std::ops::MulAssign<&Vector2u8> for Vector2u8 {
    fn mul_assign(&mut self, rhs: &Vector2u8) {
        self.mul_assign(*rhs);
    }
}

/// Multiplies all components of the vector by `rhs`
/// v *= f <=> v = [v.x * f, v.y * f]
impl std::ops::MulAssign<u8> for Vector2u8 {
    fn mul_assign(&mut self, rhs: u8) {
        self.data = self.data * u8x2::from_array([rhs, rhs]);
    }
}

/// Multiplies all components of the vector by `rhs`
/// v *= f <=> v = [v.x * f, v.y * f]
impl std::ops::MulAssign<&u8> for Vector2u8 {
    fn mul_assign(&mut self, rhs: &u8) {
        self.mul_assign(*rhs);
    }
}

/// Components wise division
/// v1 /= v2 <=> v1 = [v1.x / v2.x, v1.y / v2.y]
impl std::ops::DivAssign<Vector2u8> for Vector2u8 {
    fn div_assign(&mut self, rhs: Vector2u8) {
        self.data = self.data / rhs.data;
    }
}

/// Components wise division
/// v1 /= v2 <=> v1 = [v1.x / v2.x, v1.y / v2.y]
impl std::ops::DivAssign<&Vector2u8> for Vector2u8 {
    fn div_assign(&mut self, rhs: &Vector2u8) {
        self.div_assign(*rhs);
    }
}

/// Divides all components of the vector by `rhs`
/// v /= f <=> v1 = [v.x / f, v.y / f]
impl std::ops::DivAssign<u8> for Vector2u8 {
    fn div_assign(&mut self, rhs: u8) {
        self.data = self.data / u8x2::from_array([rhs, rhs]);
    }
}

/// Divides all components of the vector by `rhs`
/// v /= f <=> v = [v.x / f, v.y / f]
impl std::ops::DivAssign<&u8> for Vector2u8 {
    fn div_assign(&mut self, rhs: &u8) {
        self.div_assign(*rhs);
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
        let v1 = vec2u8(1, 1);
        let v2 = Vector2u8::ONES;
        assert_eq!(v1, v2);
    }

    /// Tests 2 dimensional vector operators
    #[test]
    fn operators() {
        let v1 = vec2u8(2, 3);
        let v2 = vec2u8(4, 5);
        assert_eq!(v1 + v2, vec2u8(6, 8));
        let mut v3 = v1;
        assert_eq!(v1, v3);
        v3 += v1;
        assert_eq!(v3, vec2u8(4, 6));

        v3 += 1;
        assert_eq!(v3, vec2u8(5, 7));
        assert_eq!(v3 + v3 - v3, v3);

        assert_eq!(v3 * 2, vec2u8(10, 14));
        assert_eq!(v1 * v2, vec2u8(8, 15));

        assert_eq!(v3 / v3, Vector2u8::ONES);
        v3 /= v3;
        assert_eq!(v3, Vector2u8::ONES);
    }

    /// Tests 2 dimensional vector operations
    #[test]
    fn operations() {
        let v1 = vec2u8(1, 2);
        let v2 = vec2u8(4, 3);
        assert_eq!(Vector2u8::dot(&v1, &v2), 10);
        let vx = Vector2u8::X;
        let vy = Vector2u8::Y;
        assert_eq!(Vector2u8::dot(&vx, &vy), 0);

        assert_eq!(vx.length(), 1.);
        assert_eq!(vy.length(), 1.);
        assert_eq!(v1.length(), 5f32.sqrt());
    }

    /// Tests 2 dimensional vector's fields getters and setters
    #[test]
    fn deref() {
        let v1 = vec2u8(5, 2);
        assert_eq!(v1.x, 5);
        assert_eq!(v1.y, 2);
        let mut v2 = Vector2u8::ZEROS;
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
        let v1 = vec2u8(3, 4);
        assert_eq!(v1.to_string(), "(3, 4)");
        assert_eq!(format!("{:?}", v1), "Vector2u8 { x: 3, y: 4 }");
    }
}
