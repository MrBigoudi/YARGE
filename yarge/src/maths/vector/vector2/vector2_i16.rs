use std::simd::prelude::*;

/// A structure to represent a 2 dimensional i16 vector
#[derive(Clone, Copy, PartialEq)]
#[repr(C)]
pub struct Vector2i16 {
    data: i16x2,
}

/// A structure to be able to create `.x` and `.y` getters and setters
mod private {
    #[repr(C)]
    pub struct CoordsVector2i16 {
        pub x: i16,
        pub y: i16,
    }
}

/// Implements `Deref` to allow accessing `.x` and `.y`
impl std::ops::Deref for Vector2i16 {
    type Target = private::CoordsVector2i16;

    fn deref(&self) -> &Self::Target {
        let value: *const Vector2i16 = self;
        unsafe { &*(value as *const private::CoordsVector2i16) }
    }
}

/// Implements `DerefMut` to allow modifying `.x` and `.y`
impl std::ops::DerefMut for Vector2i16 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        let value: *mut Vector2i16 = self;
        unsafe { &mut *(value as *mut private::CoordsVector2i16) }
    }
}

/// Overrides the debug trait
impl std::fmt::Debug for Vector2i16 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Vector2i16")
            .field("x", &self.x)
            .field("y", &self.y)
            .finish()
    }
}

/// Overrides the display trait
impl std::fmt::Display for Vector2i16 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

/// Sets a 2 dimensional i16 vector to `[0, 0]` as default
impl Default for Vector2i16 {
    fn default() -> Self {
        Self::ZEROS
    }
}

/// Creates a 2 dimensional i16 vector
pub fn vec2i16(x: i16, y: i16) -> Vector2i16 {
    Vector2i16::new(x, y)
}

/// A union to cast simd to array and allow const construction
union UnionCast {
    array: [i16; 2],
    simd: Vector2i16,
}

impl Vector2i16 {
    //////////////////////////////////////////////////////////
    /////////////      vector creation       /////////////////
    //////////////////////////////////////////////////////////

    /// Creates a new vector given its coordinates
    pub const fn new(x: i16, y: i16) -> Self {
        unsafe { UnionCast { array: [x, y] }.simd }
    }

    /// Creates a new vector with all coordinates set to `value`
    const fn splat(value: i16) -> Self {
        Self::new(value, value)
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
    pub const X: Self = Self::new(1, 0);

    /// Creates a new vector pointing along the positive Y axis
    pub const Y: Self = Self::new(0, 1);

    /// Creates a new vector pointing along the negative X axis
    pub const NEG_X: Self = Self::new(-1, 0);

    /// Creates a new vector pointing along the negative Y axis
    pub const NEG_Y: Self = Self::new(0, -1);

    //////////////////////////////////////////////////////////
    /////////////     vector operations      /////////////////
    //////////////////////////////////////////////////////////

    /// Sums up the element of the vector
    pub fn prefix_sum(self) -> i16 {
        self.x + self.y
    }

    /// Dot product between two vectors
    pub fn dot(v1: &Vector2i16, v2: &Vector2i16) -> i16 {
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
impl std::ops::Add<Vector2i16> for Vector2i16 {
    type Output = Vector2i16;

    fn add(self, rhs: Vector2i16) -> Self::Output {
        Vector2i16 {
            data: self.data + rhs.data,
        }
    }
}

/// Components wise addition
/// v1 + v2 = [v1.x + v2.x, v1.y + v2.y]
impl std::ops::Add<&Vector2i16> for Vector2i16 {
    type Output = Vector2i16;

    fn add(self, rhs: &Vector2i16) -> Self::Output {
        self.add(*rhs)
    }
}

/// Components wise addition
/// v1 + v2 = [v1.x + v2.x, v1.y + v2.y]
impl std::ops::Add<Vector2i16> for &Vector2i16 {
    type Output = Vector2i16;

    fn add(self, rhs: Vector2i16) -> Self::Output {
        (*self).add(rhs)
    }
}

/// Components wise addition
/// v1 + v2 = [v1.x + v2.x, v1.y + v2.y]
impl std::ops::Add<&Vector2i16> for &Vector2i16 {
    type Output = Vector2i16;

    fn add(self, rhs: &Vector2i16) -> Self::Output {
        (*self).add(*rhs)
    }
}

/// Adds `rhs` to all components of the vector
/// v + f = [v.x + f, v.y + f]
impl std::ops::Add<i16> for Vector2i16 {
    type Output = Vector2i16;

    fn add(self, rhs: i16) -> Self::Output {
        Vector2i16 {
            data: self.data + i16x2::from_array([rhs, rhs]),
        }
    }
}

/// Adds `rhs` to all components of the vector
/// v + f = [v.x + f, v.y + f]
impl std::ops::Add<i16> for &Vector2i16 {
    type Output = Vector2i16;

    fn add(self, rhs: i16) -> Self::Output {
        (*self).add(rhs)
    }
}

/// Adds `rhs` to all components of the vector
/// v + f = [v.x + f, v.y + f]
impl std::ops::Add<&i16> for Vector2i16 {
    type Output = Vector2i16;

    fn add(self, rhs: &i16) -> Self::Output {
        self.add(*rhs)
    }
}

/// Adds `rhs` to all components of the vector
/// v + f = [v.x + f, v.y + f]
impl std::ops::Add<&i16> for &Vector2i16 {
    type Output = Vector2i16;

    fn add(self, rhs: &i16) -> Self::Output {
        (*self).add(*rhs)
    }
}

//////////////////////////////////////////////////////////
//////////////   vector substractions    /////////////////
//////////////////////////////////////////////////////////

/// Components wise negation
/// -v = [-v.x, -v.y]
impl std::ops::Neg for Vector2i16 {
    type Output = Vector2i16;

    fn neg(self) -> Self::Output {
        Vector2i16 { data: -self.data }
    }
}

/// Components wise substraction
/// v1 - v2 = [v1.x - v2.x, v1.y - v2.y]
impl std::ops::Sub<Vector2i16> for Vector2i16 {
    type Output = Vector2i16;

    fn sub(self, rhs: Vector2i16) -> Self::Output {
        Vector2i16 {
            data: self.data - rhs.data,
        }
    }
}

/// Components wise substraction
/// v1 - v2 = [v1.x - v2.x, v1.y - v2.y]
impl std::ops::Sub<&Vector2i16> for Vector2i16 {
    type Output = Vector2i16;

    fn sub(self, rhs: &Vector2i16) -> Self::Output {
        self.sub(*rhs)
    }
}

/// Components wise substraction
/// v1 - v2 = [v1.x - v2.x, v1.y - v2.y]
impl std::ops::Sub<Vector2i16> for &Vector2i16 {
    type Output = Vector2i16;

    fn sub(self, rhs: Vector2i16) -> Self::Output {
        (*self).sub(rhs)
    }
}

/// Components wise substraction
/// v1 - v2 = [v1.x - v2.x, v1.y - v2.y]
impl std::ops::Sub<&Vector2i16> for &Vector2i16 {
    type Output = Vector2i16;

    fn sub(self, rhs: &Vector2i16) -> Self::Output {
        (*self).sub(*rhs)
    }
}

/// Substract `rhs` to all components of the vector
/// v - f = [v.x - f, v.y - f]
impl std::ops::Sub<i16> for Vector2i16 {
    type Output = Vector2i16;

    fn sub(self, rhs: i16) -> Self::Output {
        Vector2i16 {
            data: self.data - i16x2::from_array([rhs, rhs]),
        }
    }
}

/// Substracts `rhs` to all components of the vector
/// v - f = [v.x - f, v.y - f]
impl std::ops::Sub<i16> for &Vector2i16 {
    type Output = Vector2i16;

    fn sub(self, rhs: i16) -> Self::Output {
        (*self).sub(rhs)
    }
}

/// Substracts `rhs` to all components of the vector
/// v - f = [v.x - f, v.y - f]
impl std::ops::Sub<&i16> for Vector2i16 {
    type Output = Vector2i16;

    fn sub(self, rhs: &i16) -> Self::Output {
        self.sub(*rhs)
    }
}

/// Substracts `rhs` to all components of the vector
/// v - f = [v.x - f, v.y - f]
impl std::ops::Sub<&i16> for &Vector2i16 {
    type Output = Vector2i16;

    fn sub(self, rhs: &i16) -> Self::Output {
        (*self).sub(*rhs)
    }
}

//////////////////////////////////////////////////////////
//////////////   vector multiplications   ////////////////
//////////////////////////////////////////////////////////

/// Components wise multiplication
/// v1 * v2 = [v1.x * v2.x, v1.y * v2.y]
impl std::ops::Mul<Vector2i16> for Vector2i16 {
    type Output = Vector2i16;

    fn mul(self, rhs: Vector2i16) -> Self::Output {
        Vector2i16 {
            data: self.data * rhs.data,
        }
    }
}

/// Components wise multiplication
/// v1 * v2 = [v1.x * v2.x, v1.y * v2.y]
impl std::ops::Mul<&Vector2i16> for Vector2i16 {
    type Output = Vector2i16;

    fn mul(self, rhs: &Vector2i16) -> Self::Output {
        self.mul(*rhs)
    }
}

/// Components wise multiplication
/// v1 * v2 = [v1.x * v2.x, v1.y * v2.y]
impl std::ops::Mul<Vector2i16> for &Vector2i16 {
    type Output = Vector2i16;

    fn mul(self, rhs: Vector2i16) -> Self::Output {
        (*self).mul(rhs)
    }
}

/// Components wise multiplication
/// v1 * v2 = [v1.x * v2.x, v1.y * v2.y]
impl std::ops::Mul<&Vector2i16> for &Vector2i16 {
    type Output = Vector2i16;

    fn mul(self, rhs: &Vector2i16) -> Self::Output {
        (*self).mul(*rhs)
    }
}

/// Multiplies all components of the vector by `rhs`
/// v * f = [v.x * f, v.y * f]
impl std::ops::Mul<i16> for Vector2i16 {
    type Output = Vector2i16;

    fn mul(self, rhs: i16) -> Self::Output {
        Vector2i16 {
            data: self.data * i16x2::from_array([rhs, rhs]),
        }
    }
}

/// Multiplies all components of the vector by `rhs`
/// v * f = [v.x * f, v.y * f]
impl std::ops::Mul<i16> for &Vector2i16 {
    type Output = Vector2i16;

    fn mul(self, rhs: i16) -> Self::Output {
        (*self).mul(rhs)
    }
}

/// Multiplies all components of the vector by `rhs`
/// v * f = [v.x * f, v.y * f]
impl std::ops::Mul<&i16> for Vector2i16 {
    type Output = Vector2i16;

    fn mul(self, rhs: &i16) -> Self::Output {
        self.mul(*rhs)
    }
}

/// Multiplies all components of the vector by `rhs`
/// v * f = [v.x * f, v.y * f]
impl std::ops::Mul<&i16> for &Vector2i16 {
    type Output = Vector2i16;

    fn mul(self, rhs: &i16) -> Self::Output {
        (*self).mul(*rhs)
    }
}

//////////////////////////////////////////////////////////
//////////////     vector divisions     //////////////////
//////////////////////////////////////////////////////////

/// Components wise division
/// v1 / v2 = [v1.x / v2.x, v1.y / v2.y]
impl std::ops::Div<Vector2i16> for Vector2i16 {
    type Output = Vector2i16;

    fn div(self, rhs: Vector2i16) -> Self::Output {
        Vector2i16 {
            data: self.data / rhs.data,
        }
    }
}

/// Components wise division
/// v1 / v2 = [v1.x / v2.x, v1.y / v2.y]
impl std::ops::Div<&Vector2i16> for Vector2i16 {
    type Output = Vector2i16;

    fn div(self, rhs: &Vector2i16) -> Self::Output {
        self.div(*rhs)
    }
}

/// Components wise division
/// v1 / v2 = [v1.x / v2.x, v1.y / v2.y]
impl std::ops::Div<Vector2i16> for &Vector2i16 {
    type Output = Vector2i16;

    fn div(self, rhs: Vector2i16) -> Self::Output {
        (*self).div(rhs)
    }
}

/// Components wise division
/// v1 / v2 = [v1.x / v2.x, v1.y / v2.y]
impl std::ops::Div<&Vector2i16> for &Vector2i16 {
    type Output = Vector2i16;

    fn div(self, rhs: &Vector2i16) -> Self::Output {
        (*self).div(*rhs)
    }
}

/// Divides all components of the vector by `rhs`
/// v / f = [v.x / f, v.y / f]
impl std::ops::Div<i16> for Vector2i16 {
    type Output = Vector2i16;

    fn div(self, rhs: i16) -> Self::Output {
        Vector2i16 {
            data: self.data / i16x2::from_array([rhs, rhs]),
        }
    }
}

/// Divides all components of the vector by `rhs`
/// v / f = [v.x / f, v.y / f]
impl std::ops::Div<i16> for &Vector2i16 {
    type Output = Vector2i16;

    fn div(self, rhs: i16) -> Self::Output {
        (*self).div(rhs)
    }
}

/// Divides all components of the vector by `rhs`
/// v / f = [v.x / f, v.y / f]
impl std::ops::Div<&i16> for Vector2i16 {
    type Output = Vector2i16;

    fn div(self, rhs: &i16) -> Self::Output {
        self.div(*rhs)
    }
}

/// Divides all components of the vector by `rhs`
/// v / f = [v.x / f, v.y / f]
impl std::ops::Div<&i16> for &Vector2i16 {
    type Output = Vector2i16;

    fn div(self, rhs: &i16) -> Self::Output {
        (*self).div(*rhs)
    }
}

//////////////////////////////////////////////////////////
//////////////      vector assign       //////////////////
//////////////////////////////////////////////////////////

/// Components wise addition
/// v1 += v2 <=> v1 = [v1.x + v2.x, v1.y + v2.y]
impl std::ops::AddAssign<Vector2i16> for Vector2i16 {
    fn add_assign(&mut self, rhs: Vector2i16) {
        self.data = self.data + rhs.data;
    }
}

/// Components wise addition
/// v1 += v2 <=> v1 = [v1.x + v2.x, v1.y + v2.y]
impl std::ops::AddAssign<&Vector2i16> for Vector2i16 {
    fn add_assign(&mut self, rhs: &Vector2i16) {
        self.add_assign(*rhs);
    }
}

/// Adds `rhs` to all components of the vector
/// v += f <=> v = [v.x + f, v.y + f]
impl std::ops::AddAssign<i16> for Vector2i16 {
    fn add_assign(&mut self, rhs: i16) {
        self.data = self.data + i16x2::from_array([rhs, rhs]);
    }
}

/// Adds `rhs` to all components of the vector
/// v += f <=> v = [v.x + f, v.y + f]
impl std::ops::AddAssign<&i16> for Vector2i16 {
    fn add_assign(&mut self, rhs: &i16) {
        self.add_assign(*rhs);
    }
}

/// Components wise substraction
/// v1 -= v2 <=> v1 = [v1.x - v2.x, v1.y - v2.y]
impl std::ops::SubAssign<Vector2i16> for Vector2i16 {
    fn sub_assign(&mut self, rhs: Vector2i16) {
        self.data = self.data - rhs.data;
    }
}

/// Components wise substraction
/// v1 -= v2 <=> v1 = [v1.x - v2.x, v1.y - v2.y]
impl std::ops::SubAssign<&Vector2i16> for Vector2i16 {
    fn sub_assign(&mut self, rhs: &Vector2i16) {
        self.sub_assign(*rhs);
    }
}

/// Substracts `rhs` to all components of the vector
/// v -= f <=> v = [v.x - f, v.y - f]
impl std::ops::SubAssign<i16> for Vector2i16 {
    fn sub_assign(&mut self, rhs: i16) {
        self.data = self.data - i16x2::from_array([rhs, rhs]);
    }
}

/// Substracts `rhs` to all components of the vector
/// v -= f <=> v = [v.x - f, v.y - f]
impl std::ops::SubAssign<&i16> for Vector2i16 {
    fn sub_assign(&mut self, rhs: &i16) {
        self.sub_assign(*rhs);
    }
}

/// Components wise multiplication
/// v1 *= v2 <=> v1 = [v1.x * v2.x, v1.y * v2.y]
impl std::ops::MulAssign<Vector2i16> for Vector2i16 {
    fn mul_assign(&mut self, rhs: Vector2i16) {
        self.data = self.data * rhs.data;
    }
}

/// Components wise multiplication
/// v1 *= v2 <=> v1 = [v1.x * v2.x, v1.y * v2.y]
impl std::ops::MulAssign<&Vector2i16> for Vector2i16 {
    fn mul_assign(&mut self, rhs: &Vector2i16) {
        self.mul_assign(*rhs);
    }
}

/// Multiplies all components of the vector by `rhs`
/// v *= f <=> v = [v.x * f, v.y * f]
impl std::ops::MulAssign<i16> for Vector2i16 {
    fn mul_assign(&mut self, rhs: i16) {
        self.data = self.data * i16x2::from_array([rhs, rhs]);
    }
}

/// Multiplies all components of the vector by `rhs`
/// v *= f <=> v = [v.x * f, v.y * f]
impl std::ops::MulAssign<&i16> for Vector2i16 {
    fn mul_assign(&mut self, rhs: &i16) {
        self.mul_assign(*rhs);
    }
}

/// Components wise division
/// v1 /= v2 <=> v1 = [v1.x / v2.x, v1.y / v2.y]
impl std::ops::DivAssign<Vector2i16> for Vector2i16 {
    fn div_assign(&mut self, rhs: Vector2i16) {
        self.data = self.data / rhs.data;
    }
}

/// Components wise division
/// v1 /= v2 <=> v1 = [v1.x / v2.x, v1.y / v2.y]
impl std::ops::DivAssign<&Vector2i16> for Vector2i16 {
    fn div_assign(&mut self, rhs: &Vector2i16) {
        self.div_assign(*rhs);
    }
}

/// Divides all components of the vector by `rhs`
/// v /= f <=> v1 = [v.x / f, v.y / f]
impl std::ops::DivAssign<i16> for Vector2i16 {
    fn div_assign(&mut self, rhs: i16) {
        self.data = self.data / i16x2::from_array([rhs, rhs]);
    }
}

/// Divides all components of the vector by `rhs`
/// v /= f <=> v = [v.x / f, v.y / f]
impl std::ops::DivAssign<&i16> for Vector2i16 {
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

    /// Tests 2 dimensional vector initialization
    #[test]
    fn initialization() {
        let v1 = vec2i16(1, 1);
        let v2 = Vector2i16::ONES;
        assert_eq!(v1, v2);
    }

    /// Tests 2 dimensional vector operators
    #[test]
    fn operators() {
        let v1 = vec2i16(2, 3);
        let v2 = vec2i16(4, 5);
        assert_eq!(v1 + v2, vec2i16(6, 8));
        let mut v3 = v1;
        assert_eq!(v1, v3);
        v3 += v1;
        assert_eq!(v3, vec2i16(4, 6));

        v3 += 1;
        assert_eq!(v3, vec2i16(5, 7));
        assert_eq!(v3 + v3 - v3, v3);

        assert_eq!(v3 * 2, vec2i16(10, 14));
        assert_eq!(v1 * v2, vec2i16(8, 15));

        assert_eq!(v3 / v3, Vector2i16::ONES);
        v3 /= v3;
        assert_eq!(v3, Vector2i16::ONES);
    }

    /// Tests 2 dimensional vector operations
    #[test]
    fn operations() {
        let v1 = vec2i16(1, 2);
        let v2 = vec2i16(4, 3);
        assert_eq!(Vector2i16::dot(&v1, &v2), 10);
        let vx = Vector2i16::X;
        let vy = Vector2i16::Y;
        assert_eq!(Vector2i16::dot(&vx, &vy), 0);

        assert_eq!(vx.length(), 1.);
        assert_eq!(vy.length(), 1.);
        assert_eq!(v1.length(), 5f32.sqrt());
    }

    /// Tests 2 dimensional vector's fields getters and setters
    #[test]
    fn deref() {
        let v1 = vec2i16(5, 2);
        assert_eq!(v1.x, 5);
        assert_eq!(v1.y, 2);
        let mut v2 = Vector2i16::ZEROS;
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
        let v1 = vec2i16(3, 4);
        assert_eq!(v1.to_string(), "(3, 4)");
        assert_eq!(format!("{:?}", v1), "Vector2i16 { x: 3, y: 4 }");
    }
}
