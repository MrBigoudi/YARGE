use std::simd::prelude::*;

/// A structure to represent a 2 dimensional usize vector
#[derive(Clone, Copy, PartialEq)]
#[repr(C)]
pub struct Vector2usize {
    data: usizex2,
}

/// A structure to be able to create `.x` and `.y` getters and setters
mod private {
    #[repr(C)]
    pub struct CoordsVector2usize {
        pub x: usize,
        pub y: usize,
    }
}

/// Implements `Deref` to allow accessing `.x` and `.y`
impl std::ops::Deref for Vector2usize {
    type Target = private::CoordsVector2usize;

    fn deref(&self) -> &Self::Target {
        let value: *const Vector2usize = self;
        unsafe { &*(value as *const private::CoordsVector2usize) }
    }
}

/// Implements `DerefMut` to allow modifying `.x` and `.y`
impl std::ops::DerefMut for Vector2usize {
    fn deref_mut(&mut self) -> &mut Self::Target {
        let value: *mut Vector2usize = self;
        unsafe { &mut *(value as *mut private::CoordsVector2usize) }
    }
}

/// Overrides the debug trait
impl std::fmt::Debug for Vector2usize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Vector2usize")
            .field("x", &self.x)
            .field("y", &self.y)
            .finish()
    }
}

/// Overrides the display trait
impl std::fmt::Display for Vector2usize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

/// Sets a 2 dimensional usize vector to `[0, 0]` as default
impl Default for Vector2usize {
    fn default() -> Self {
        Self::ZEROS
    }
}

/// Creates a 2 dimensional usize vector
pub const fn vec2usize(x: usize, y: usize) -> Vector2usize {
    Vector2usize::new(x, y)
}

/// A union to cast simd to array and allow const construction
union UnionCast {
    array: [usize; 2],
    simd: Vector2usize,
}

impl Vector2usize {
    //////////////////////////////////////////////////////////
    /////////////      vector creation       /////////////////
    //////////////////////////////////////////////////////////

    /// Creates a new vector given its coordinates
    pub const fn new(x: usize, y: usize) -> Self {
        unsafe { UnionCast { array: [x, y] }.simd }
    }

    /// Creates a new vector with all coordinates set to `value`
    const fn splat(value: usize) -> Self {
        Self::new(value, value)
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
    pub const X: Self = Self::new(1, 0);

    /// Creates a new vector pointing along the positive Y axis
    pub const Y: Self = Self::new(0, 1);

    //////////////////////////////////////////////////////////
    /////////////     vector operations      /////////////////
    //////////////////////////////////////////////////////////

    /// Sums up the element of the vector
    pub fn prefix_sum(self) -> usize {
        self.x + self.y
    }

    /// Dot product between two vectors
    pub fn dot(v1: &Vector2usize, v2: &Vector2usize) -> usize {
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
}

//////////////////////////////////////////////////////////
//////////////     vector additions     //////////////////
//////////////////////////////////////////////////////////

/// Components wise addition
/// v1 + v2 = [v1.x + v2.x, v1.y + v2.y]
impl std::ops::Add<Vector2usize> for Vector2usize {
    type Output = Vector2usize;

    fn add(self, rhs: Vector2usize) -> Self::Output {
        Vector2usize {
            data: self.data + rhs.data,
        }
    }
}

/// Components wise addition
/// v1 + v2 = [v1.x + v2.x, v1.y + v2.y]
impl std::ops::Add<&Vector2usize> for Vector2usize {
    type Output = Vector2usize;

    fn add(self, rhs: &Vector2usize) -> Self::Output {
        self.add(*rhs)
    }
}

/// Components wise addition
/// v1 + v2 = [v1.x + v2.x, v1.y + v2.y]
impl std::ops::Add<Vector2usize> for &Vector2usize {
    type Output = Vector2usize;

    fn add(self, rhs: Vector2usize) -> Self::Output {
        (*self).add(rhs)
    }
}

/// Components wise addition
/// v1 + v2 = [v1.x + v2.x, v1.y + v2.y]
impl std::ops::Add<&Vector2usize> for &Vector2usize {
    type Output = Vector2usize;

    fn add(self, rhs: &Vector2usize) -> Self::Output {
        (*self).add(*rhs)
    }
}

/// Adds `rhs` to all components of the vector
/// v + f = [v.x + f, v.y + f]
impl std::ops::Add<usize> for Vector2usize {
    type Output = Vector2usize;

    fn add(self, rhs: usize) -> Self::Output {
        Vector2usize {
            data: self.data + usizex2::from_array([rhs, rhs]),
        }
    }
}

/// Adds `rhs` to all components of the vector
/// v + f = [v.x + f, v.y + f]
impl std::ops::Add<usize> for &Vector2usize {
    type Output = Vector2usize;

    fn add(self, rhs: usize) -> Self::Output {
        (*self).add(rhs)
    }
}

/// Adds `rhs` to all components of the vector
/// v + f = [v.x + f, v.y + f]
impl std::ops::Add<&usize> for Vector2usize {
    type Output = Vector2usize;

    fn add(self, rhs: &usize) -> Self::Output {
        self.add(*rhs)
    }
}

/// Adds `rhs` to all components of the vector
/// v + f = [v.x + f, v.y + f]
impl std::ops::Add<&usize> for &Vector2usize {
    type Output = Vector2usize;

    fn add(self, rhs: &usize) -> Self::Output {
        (*self).add(*rhs)
    }
}

//////////////////////////////////////////////////////////
//////////////   vector substractions    /////////////////
//////////////////////////////////////////////////////////

/// Components wise substraction
/// v1 - v2 = [v1.x - v2.x, v1.y - v2.y]
impl std::ops::Sub<Vector2usize> for Vector2usize {
    type Output = Vector2usize;

    fn sub(self, rhs: Vector2usize) -> Self::Output {
        Vector2usize {
            data: self.data - rhs.data,
        }
    }
}

/// Components wise substraction
/// v1 - v2 = [v1.x - v2.x, v1.y - v2.y]
impl std::ops::Sub<&Vector2usize> for Vector2usize {
    type Output = Vector2usize;

    fn sub(self, rhs: &Vector2usize) -> Self::Output {
        self.sub(*rhs)
    }
}

/// Components wise substraction
/// v1 - v2 = [v1.x - v2.x, v1.y - v2.y]
impl std::ops::Sub<Vector2usize> for &Vector2usize {
    type Output = Vector2usize;

    fn sub(self, rhs: Vector2usize) -> Self::Output {
        (*self).sub(rhs)
    }
}

/// Components wise substraction
/// v1 - v2 = [v1.x - v2.x, v1.y - v2.y]
impl std::ops::Sub<&Vector2usize> for &Vector2usize {
    type Output = Vector2usize;

    fn sub(self, rhs: &Vector2usize) -> Self::Output {
        (*self).sub(*rhs)
    }
}

/// Substract `rhs` to all components of the vector
/// v - f = [v.x - f, v.y - f]
impl std::ops::Sub<usize> for Vector2usize {
    type Output = Vector2usize;

    fn sub(self, rhs: usize) -> Self::Output {
        Vector2usize {
            data: self.data - usizex2::from_array([rhs, rhs]),
        }
    }
}

/// Substracts `rhs` to all components of the vector
/// v - f = [v.x - f, v.y - f]
impl std::ops::Sub<usize> for &Vector2usize {
    type Output = Vector2usize;

    fn sub(self, rhs: usize) -> Self::Output {
        (*self).sub(rhs)
    }
}

/// Substracts `rhs` to all components of the vector
/// v - f = [v.x - f, v.y - f]
impl std::ops::Sub<&usize> for Vector2usize {
    type Output = Vector2usize;

    fn sub(self, rhs: &usize) -> Self::Output {
        self.sub(*rhs)
    }
}

/// Substracts `rhs` to all components of the vector
/// v - f = [v.x - f, v.y - f]
impl std::ops::Sub<&usize> for &Vector2usize {
    type Output = Vector2usize;

    fn sub(self, rhs: &usize) -> Self::Output {
        (*self).sub(*rhs)
    }
}

//////////////////////////////////////////////////////////
//////////////   vector multiplications   ////////////////
//////////////////////////////////////////////////////////

/// Components wise multiplication
/// v1 * v2 = [v1.x * v2.x, v1.y * v2.y]
impl std::ops::Mul<Vector2usize> for Vector2usize {
    type Output = Vector2usize;

    fn mul(self, rhs: Vector2usize) -> Self::Output {
        Vector2usize {
            data: self.data * rhs.data,
        }
    }
}

/// Components wise multiplication
/// v1 * v2 = [v1.x * v2.x, v1.y * v2.y]
impl std::ops::Mul<&Vector2usize> for Vector2usize {
    type Output = Vector2usize;

    fn mul(self, rhs: &Vector2usize) -> Self::Output {
        self.mul(*rhs)
    }
}

/// Components wise multiplication
/// v1 * v2 = [v1.x * v2.x, v1.y * v2.y]
impl std::ops::Mul<Vector2usize> for &Vector2usize {
    type Output = Vector2usize;

    fn mul(self, rhs: Vector2usize) -> Self::Output {
        (*self).mul(rhs)
    }
}

/// Components wise multiplication
/// v1 * v2 = [v1.x * v2.x, v1.y * v2.y]
impl std::ops::Mul<&Vector2usize> for &Vector2usize {
    type Output = Vector2usize;

    fn mul(self, rhs: &Vector2usize) -> Self::Output {
        (*self).mul(*rhs)
    }
}

/// Multiplies all components of the vector by `rhs`
/// v * f = [v.x * f, v.y * f]
impl std::ops::Mul<usize> for Vector2usize {
    type Output = Vector2usize;

    fn mul(self, rhs: usize) -> Self::Output {
        Vector2usize {
            data: self.data * usizex2::from_array([rhs, rhs]),
        }
    }
}

/// Multiplies all components of the vector by `rhs`
/// v * f = [v.x * f, v.y * f]
impl std::ops::Mul<usize> for &Vector2usize {
    type Output = Vector2usize;

    fn mul(self, rhs: usize) -> Self::Output {
        (*self).mul(rhs)
    }
}

/// Multiplies all components of the vector by `rhs`
/// v * f = [v.x * f, v.y * f]
impl std::ops::Mul<&usize> for Vector2usize {
    type Output = Vector2usize;

    fn mul(self, rhs: &usize) -> Self::Output {
        self.mul(*rhs)
    }
}

/// Multiplies all components of the vector by `rhs`
/// v * f = [v.x * f, v.y * f]
impl std::ops::Mul<&usize> for &Vector2usize {
    type Output = Vector2usize;

    fn mul(self, rhs: &usize) -> Self::Output {
        (*self).mul(*rhs)
    }
}

//////////////////////////////////////////////////////////
//////////////     vector divisions     //////////////////
//////////////////////////////////////////////////////////

/// Components wise division
/// v1 / v2 = [v1.x / v2.x, v1.y / v2.y]
impl std::ops::Div<Vector2usize> for Vector2usize {
    type Output = Vector2usize;

    fn div(self, rhs: Vector2usize) -> Self::Output {
        Vector2usize {
            data: self.data / rhs.data,
        }
    }
}

/// Components wise division
/// v1 / v2 = [v1.x / v2.x, v1.y / v2.y]
impl std::ops::Div<&Vector2usize> for Vector2usize {
    type Output = Vector2usize;

    fn div(self, rhs: &Vector2usize) -> Self::Output {
        self.div(*rhs)
    }
}

/// Components wise division
/// v1 / v2 = [v1.x / v2.x, v1.y / v2.y]
impl std::ops::Div<Vector2usize> for &Vector2usize {
    type Output = Vector2usize;

    fn div(self, rhs: Vector2usize) -> Self::Output {
        (*self).div(rhs)
    }
}

/// Components wise division
/// v1 / v2 = [v1.x / v2.x, v1.y / v2.y]
impl std::ops::Div<&Vector2usize> for &Vector2usize {
    type Output = Vector2usize;

    fn div(self, rhs: &Vector2usize) -> Self::Output {
        (*self).div(*rhs)
    }
}

/// Divides all components of the vector by `rhs`
/// v / f = [v.x / f, v.y / f]
impl std::ops::Div<usize> for Vector2usize {
    type Output = Vector2usize;

    fn div(self, rhs: usize) -> Self::Output {
        Vector2usize {
            data: self.data / usizex2::from_array([rhs, rhs]),
        }
    }
}

/// Divides all components of the vector by `rhs`
/// v / f = [v.x / f, v.y / f]
impl std::ops::Div<usize> for &Vector2usize {
    type Output = Vector2usize;

    fn div(self, rhs: usize) -> Self::Output {
        (*self).div(rhs)
    }
}

/// Divides all components of the vector by `rhs`
/// v / f = [v.x / f, v.y / f]
impl std::ops::Div<&usize> for Vector2usize {
    type Output = Vector2usize;

    fn div(self, rhs: &usize) -> Self::Output {
        self.div(*rhs)
    }
}

/// Divides all components of the vector by `rhs`
/// v / f = [v.x / f, v.y / f]
impl std::ops::Div<&usize> for &Vector2usize {
    type Output = Vector2usize;

    fn div(self, rhs: &usize) -> Self::Output {
        (*self).div(*rhs)
    }
}

//////////////////////////////////////////////////////////
//////////////      vector assign       //////////////////
//////////////////////////////////////////////////////////

/// Components wise addition
/// v1 += v2 <=> v1 = [v1.x + v2.x, v1.y + v2.y]
impl std::ops::AddAssign<Vector2usize> for Vector2usize {
    fn add_assign(&mut self, rhs: Vector2usize) {
        self.data = self.data + rhs.data;
    }
}

/// Components wise addition
/// v1 += v2 <=> v1 = [v1.x + v2.x, v1.y + v2.y]
impl std::ops::AddAssign<&Vector2usize> for Vector2usize {
    fn add_assign(&mut self, rhs: &Vector2usize) {
        self.add_assign(*rhs);
    }
}

/// Adds `rhs` to all components of the vector
/// v += f <=> v = [v.x + f, v.y + f]
impl std::ops::AddAssign<usize> for Vector2usize {
    fn add_assign(&mut self, rhs: usize) {
        self.data = self.data + usizex2::from_array([rhs, rhs]);
    }
}

/// Adds `rhs` to all components of the vector
/// v += f <=> v = [v.x + f, v.y + f]
impl std::ops::AddAssign<&usize> for Vector2usize {
    fn add_assign(&mut self, rhs: &usize) {
        self.add_assign(*rhs);
    }
}

/// Components wise substraction
/// v1 -= v2 <=> v1 = [v1.x - v2.x, v1.y - v2.y]
impl std::ops::SubAssign<Vector2usize> for Vector2usize {
    fn sub_assign(&mut self, rhs: Vector2usize) {
        self.data = self.data - rhs.data;
    }
}

/// Components wise substraction
/// v1 -= v2 <=> v1 = [v1.x - v2.x, v1.y - v2.y]
impl std::ops::SubAssign<&Vector2usize> for Vector2usize {
    fn sub_assign(&mut self, rhs: &Vector2usize) {
        self.sub_assign(*rhs);
    }
}

/// Substracts `rhs` to all components of the vector
/// v -= f <=> v = [v.x - f, v.y - f]
impl std::ops::SubAssign<usize> for Vector2usize {
    fn sub_assign(&mut self, rhs: usize) {
        self.data = self.data - usizex2::from_array([rhs, rhs]);
    }
}

/// Substracts `rhs` to all components of the vector
/// v -= f <=> v = [v.x - f, v.y - f]
impl std::ops::SubAssign<&usize> for Vector2usize {
    fn sub_assign(&mut self, rhs: &usize) {
        self.sub_assign(*rhs);
    }
}

/// Components wise multiplication
/// v1 *= v2 <=> v1 = [v1.x * v2.x, v1.y * v2.y]
impl std::ops::MulAssign<Vector2usize> for Vector2usize {
    fn mul_assign(&mut self, rhs: Vector2usize) {
        self.data = self.data * rhs.data;
    }
}

/// Components wise multiplication
/// v1 *= v2 <=> v1 = [v1.x * v2.x, v1.y * v2.y]
impl std::ops::MulAssign<&Vector2usize> for Vector2usize {
    fn mul_assign(&mut self, rhs: &Vector2usize) {
        self.mul_assign(*rhs);
    }
}

/// Multiplies all components of the vector by `rhs`
/// v *= f <=> v = [v.x * f, v.y * f]
impl std::ops::MulAssign<usize> for Vector2usize {
    fn mul_assign(&mut self, rhs: usize) {
        self.data = self.data * usizex2::from_array([rhs, rhs]);
    }
}

/// Multiplies all components of the vector by `rhs`
/// v *= f <=> v = [v.x * f, v.y * f]
impl std::ops::MulAssign<&usize> for Vector2usize {
    fn mul_assign(&mut self, rhs: &usize) {
        self.mul_assign(*rhs);
    }
}

/// Components wise division
/// v1 /= v2 <=> v1 = [v1.x / v2.x, v1.y / v2.y]
impl std::ops::DivAssign<Vector2usize> for Vector2usize {
    fn div_assign(&mut self, rhs: Vector2usize) {
        self.data = self.data / rhs.data;
    }
}

/// Components wise division
/// v1 /= v2 <=> v1 = [v1.x / v2.x, v1.y / v2.y]
impl std::ops::DivAssign<&Vector2usize> for Vector2usize {
    fn div_assign(&mut self, rhs: &Vector2usize) {
        self.div_assign(*rhs);
    }
}

/// Divides all components of the vector by `rhs`
/// v /= f <=> v1 = [v.x / f, v.y / f]
impl std::ops::DivAssign<usize> for Vector2usize {
    fn div_assign(&mut self, rhs: usize) {
        self.data = self.data / usizex2::from_array([rhs, rhs]);
    }
}

/// Divides all components of the vector by `rhs`
/// v /= f <=> v = [v.x / f, v.y / f]
impl std::ops::DivAssign<&usize> for Vector2usize {
    fn div_assign(&mut self, rhs: &usize) {
        self.div_assign(*rhs);
    }
}

//////////////////////////////////////////////////////////
///////////////     vector indices     ///////////////////
//////////////////////////////////////////////////////////
impl std::ops::Index<usize> for Vector2usize {
    type Output = usize;
    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            _ => panic!("Index out of bounds"),
        }
    }
}

impl std::ops::IndexMut<usize> for Vector2usize {
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
        let v1 = vec2usize(1, 1);
        let v2 = Vector2usize::ONES;
        assert_eq!(v1, v2);
    }

    /// Tests 2 dimensional vector operators
    #[test]
    fn operators() {
        let v1 = vec2usize(2, 3);
        let v2 = vec2usize(4, 5);
        assert_eq!(v1 + v2, vec2usize(6, 8));
        let mut v3 = v1;
        assert_eq!(v1, v3);
        v3 += v1;
        assert_eq!(v3, vec2usize(4, 6));

        v3 += 1;
        assert_eq!(v3, vec2usize(5, 7));
        assert_eq!(v3 + v3 - v3, v3);

        assert_eq!(v3 * 2, vec2usize(10, 14));
        assert_eq!(v1 * v2, vec2usize(8, 15));

        assert_eq!(v3 / v3, Vector2usize::ONES);
        v3 /= v3;
        assert_eq!(v3, Vector2usize::ONES);
    }

    /// Tests 2 dimensional vector operations
    #[test]
    fn operations() {
        let v1 = vec2usize(1, 2);
        let v2 = vec2usize(4, 3);
        assert_eq!(Vector2usize::dot(&v1, &v2), 10);
        let vx = Vector2usize::X;
        let vy = Vector2usize::Y;
        assert_eq!(Vector2usize::dot(&vx, &vy), 0);

        assert_eq!(vx.length(), 1.);
        assert_eq!(vy.length(), 1.);
        assert_eq!(v1.length(), 5f32.sqrt());
    }

    /// Tests 2 dimensional vector's fields getters and setters
    #[test]
    fn deref() {
        let v1 = vec2usize(5, 2);
        assert_eq!(v1.x, 5);
        assert_eq!(v1.y, 2);
        let mut v2 = Vector2usize::ZEROS;
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
        let v1 = vec2usize(3, 4);
        assert_eq!(v1.to_string(), "(3, 4)");
        assert_eq!(format!("{:?}", v1), "Vector2usize { x: 3, y: 4 }");
    }

    /// Tests indices access
    #[test]
    fn indices() {
        let mut v1 = vec2usize(2, 3);
        assert_eq!(v1[1], 3);
        v1[0] = 4;
        assert_eq!(v1[0], 4);
    }
}
