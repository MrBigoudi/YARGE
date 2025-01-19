use std::simd::prelude::*;

/// A structure to represent a 2 dimensional i64 vector
#[derive(Clone, Copy, PartialEq)]
#[repr(C)]
pub struct LVector2 {
    data: i64x2,
}

/// A structure to be able to create `.x` and `.y` getters and setters
mod private {
    #[repr(C)]
    pub struct CoordsLVector2 {
        pub x: i64,
        pub y: i64,
    }
}

/// Implement `Deref` to allow accessing `.x` and `.y`
impl std::ops::Deref for LVector2 {
    type Target = private::CoordsLVector2;

    fn deref(&self) -> &Self::Target {
        unsafe { &*(self as *const LVector2 as *const private::CoordsLVector2) }
    }
}

/// Implement `DerefMut` to allow modifying `.x` and `.y`
impl std::ops::DerefMut for LVector2 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *(self as *mut LVector2 as *mut private::CoordsLVector2) }
    }
}

/// Override the debug trait
impl std::fmt::Debug for LVector2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LVector2")
            .field("x", &self.x)
            .field("y", &self.y)
            .finish()
    }
}

/// Override the display trait
impl std::fmt::Display for LVector2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

/// Set a 2 dimensional i64 vector to `[0., 0.]` as default
impl Default for LVector2 {
    fn default() -> Self {
        Self::ZEROS
    }
}

/// Create a 2 dimensional i64 vector
pub fn ivec2(x: i64, y: i64) -> LVector2 {
    LVector2::new(x, y)
}

/// A union to cast simd to array and allow const construction
union UnionCast {
    array: [i64; 2],
    simd: LVector2,
}

impl LVector2 {
    //////////////////////////////////////////////////////////
    /////////////      vector creation       /////////////////
    //////////////////////////////////////////////////////////

    /// Create a new vector given its coordinates
    pub const fn new(x: i64, y: i64) -> Self {
        unsafe {
            UnionCast {
                array: [x, y],
            }
            .simd
        }
    }

    /// Create a new vector with all coordinates set to `value`
    const fn splat(value: i64) -> Self {
        Self::new(value, value)
    }

    /// Create a new vector filled with `values`
    pub const fn filled(value: i64) -> Self {
        Self::splat(value)
    }

    /// Create a new vector filled with ones
    pub const ONES: Self = Self::splat(1);

    /// Create a new vector filled with negative ones
    pub const NEG_ONES: Self = Self::splat(-1);

    /// Create a new vector filled with zeros
    pub const ZEROS: Self = Self::splat(0);

    /// Create a new vector filled with `i64::MIN``
    pub const MIN: Self = Self::splat(i64::MIN);

    /// Create a new vector filled with `i64::MAX`
    pub const MAX: Self = Self::splat(i64::MAX);

    /// Create a new vector pointing along the positive X axis
    pub const X: Self = Self::new(1, 0);

    /// Create a new vector pointing along the positive Y axis
    pub const Y: Self = Self::new(0, 1);

    /// Create a new vector pointing along the negative X axis
    pub const NEG_X: Self = Self::new(-1, 0);

    /// Create a new vector pointing along the negative Y axis
    pub const NEG_Y: Self = Self::new(0, -1);

    //////////////////////////////////////////////////////////
    /////////////     vector operations      /////////////////
    //////////////////////////////////////////////////////////

    /// Sum the element of the vector
    pub fn prefix_sum(self) -> i64 {
        self.x + self.y
    }

    /// Dot products between vectors
    pub fn dot(v1: &LVector2, v2: &LVector2) -> i64 {
        (v1*v2).prefix_sum()
    } 

    /// Length of the vector
    pub fn length(&self) -> f32 {
        (Self::dot(self, self) as f32).sqrt()
    }
}



//////////////////////////////////////////////////////////
//////////////     vector additions     //////////////////
//////////////////////////////////////////////////////////

/// Components wise addition
/// v1 + v2 = [v1.x + v2.x, v1.y + v2.y]
impl std::ops::Add<LVector2> for LVector2 {
    type Output = LVector2;

    fn add(self, rhs: LVector2) -> Self::Output {
        LVector2 {
            data: self.data + rhs.data,
        }
    }
}

/// Components wise addition
/// v1 + v2 = [v1.x + v2.x, v1.y + v2.y]
impl std::ops::Add<&LVector2> for LVector2 {
    type Output = LVector2;

    fn add(self, rhs: &LVector2) -> Self::Output {
        self.add(*rhs)
    }
}

/// Components wise addition
/// v1 + v2 = [v1.x + v2.x, v1.y + v2.y]
impl std::ops::Add<LVector2> for &LVector2 {
    type Output = LVector2;

    fn add(self, rhs: LVector2) -> Self::Output {
        (*self).add(rhs)
    }
}

/// Components wise addition
/// v1 + v2 = [v1.x + v2.x, v1.y + v2.y]
impl std::ops::Add<&LVector2> for &LVector2 {
    type Output = LVector2;

    fn add(self, rhs: &LVector2) -> Self::Output {
        (*self).add(*rhs)
    }
}

/// Add `rhs` to all components of the vector 
/// v1 + f = [v1.x + f, v1.y + f]
impl std::ops::Add<i64> for LVector2 {
    type Output = LVector2;

    fn add(self, rhs: i64) -> Self::Output {
        LVector2 {
            data: self.data + i64x2::from_array([rhs, rhs])
        }
    }
}

/// Add `rhs` to all components of the vector 
/// v1 + f = [v1.x + f, v1.y + f]
impl std::ops::Add<i64> for &LVector2 {
    type Output = LVector2;

    fn add(self, rhs: i64) -> Self::Output {
        (*self).add(rhs)
    }
}

/// Add `rhs` to all components of the vector 
/// v1 + f = [v1.x + f, v1.y + f]
impl std::ops::Add<&i64> for LVector2 {
    type Output = LVector2;

    fn add(self, rhs: &i64) -> Self::Output {
        self.add(*rhs)
    }
}

/// Add `rhs` to all components of the vector 
/// v1 + f = [v1.x + f, v1.y + f]
impl std::ops::Add<&i64> for &LVector2 {
    type Output = LVector2;

    fn add(self, rhs: &i64) -> Self::Output {
        (*self).add(*rhs)
    }
}


//////////////////////////////////////////////////////////
//////////////   vector substractions    /////////////////
//////////////////////////////////////////////////////////

/// Components wise substraction
/// v1 - v2 = [v1.x - v2.x, v1.y - v2.y]
impl std::ops::Sub<LVector2> for LVector2 {
    type Output = LVector2;

    fn sub(self, rhs: LVector2) -> Self::Output {
        LVector2 {
            data: self.data - rhs.data,
        }
    }
}

/// Components wise substraction
/// v1 - v2 = [v1.x - v2.x, v1.y - v2.y]
impl std::ops::Sub<&LVector2> for LVector2 {
    type Output = LVector2;

    fn sub(self, rhs: &LVector2) -> Self::Output {
        self.sub(*rhs)
    }
}

/// Components wise substraction
/// v1 - v2 = [v1.x - v2.x, v1.y - v2.y]
impl std::ops::Sub<LVector2> for &LVector2 {
    type Output = LVector2;

    fn sub(self, rhs: LVector2) -> Self::Output {
        (*self).sub(rhs)
    }
}

/// Components wise substraction
/// v1 - v2 = [v1.x - v2.x, v1.y - v2.y]
impl std::ops::Sub<&LVector2> for &LVector2 {
    type Output = LVector2;

    fn sub(self, rhs: &LVector2) -> Self::Output {
        (*self).sub(*rhs)
    }
}

/// Substract `rhs` to all components of the vector 
/// v1 - f = [v1.x - f, v1.y - f]
impl std::ops::Sub<i64> for LVector2 {
    type Output = LVector2;

    fn sub(self, rhs: i64) -> Self::Output {
        LVector2 {
            data: self.data - i64x2::from_array([rhs, rhs])
        }
    }
}

/// Substract `rhs` to all components of the vector 
/// v1 - f = [v1.x - f, v1.y - f]
impl std::ops::Sub<i64> for &LVector2 {
    type Output = LVector2;

    fn sub(self, rhs: i64) -> Self::Output {
        (*self).sub(rhs)
    }
}

/// Substract `rhs` to all components of the vector 
/// v1 - f = [v1.x - f, v1.y - f]
impl std::ops::Sub<&i64> for LVector2 {
    type Output = LVector2;

    fn sub(self, rhs: &i64) -> Self::Output {
        self.sub(*rhs)
    }
}

/// Substract `rhs` to all components of the vector 
/// v1 - f = [v1.x - f, v1.y - f]
impl std::ops::Sub<&i64> for &LVector2 {
    type Output = LVector2;

    fn sub(self, rhs: &i64) -> Self::Output {
        (*self).sub(*rhs)
    }
}



//////////////////////////////////////////////////////////
//////////////   vector multiplications   ////////////////
//////////////////////////////////////////////////////////

/// Components wise multiplication
/// v1 * v2 = [v1.x * v2.x, v1.y * v2.y]
impl std::ops::Mul<LVector2> for LVector2 {
    type Output = LVector2;

    fn mul(self, rhs: LVector2) -> Self::Output {
        LVector2 {
            data: self.data * rhs.data,
        }
    }
}

/// Components wise multiplication
/// v1 * v2 = [v1.x * v2.x, v1.y * v2.y]
impl std::ops::Mul<&LVector2> for LVector2 {
    type Output = LVector2;

    fn mul(self, rhs: &LVector2) -> Self::Output {
        self.mul(*rhs)
    }
}

/// Components wise multiplication
/// v1 * v2 = [v1.x * v2.x, v1.y * v2.y]
impl std::ops::Mul<LVector2> for &LVector2 {
    type Output = LVector2;

    fn mul(self, rhs: LVector2) -> Self::Output {
        (*self).mul(rhs)
    }
}

/// Components wise multiplication
/// v1 * v2 = [v1.x * v2.x, v1.y * v2.y]
impl std::ops::Mul<&LVector2> for &LVector2 {
    type Output = LVector2;

    fn mul(self, rhs: &LVector2) -> Self::Output {
        (*self).mul(*rhs)
    }
}

/// Multiply all components of the vector by `rhs` 
/// v1 * f = [v1.x * f, v1.y * f]
impl std::ops::Mul<i64> for LVector2 {
    type Output = LVector2;

    fn mul(self, rhs: i64) -> Self::Output {
        LVector2 {
            data: self.data * i64x2::from_array([rhs, rhs])
        }
    }
}

/// Multiply all components of the vector by `rhs` 
/// v1 * f = [v1.x * f, v1.y * f]
impl std::ops::Mul<i64> for &LVector2 {
    type Output = LVector2;

    fn mul(self, rhs: i64) -> Self::Output {
        (*self).mul(rhs)
    }
}

/// Multiply all components of the vector by `rhs` 
/// v1 * f = [v1.x * f, v1.y * f]
impl std::ops::Mul<&i64> for LVector2 {
    type Output = LVector2;

    fn mul(self, rhs: &i64) -> Self::Output {
        self.mul(*rhs)
    }
}

/// Multiply all components of the vector by `rhs` 
/// v1 * f = [v1.x * f, v1.y * f]
impl std::ops::Mul<&i64> for &LVector2 {
    type Output = LVector2;

    fn mul(self, rhs: &i64) -> Self::Output {
        (*self).mul(*rhs)
    }
}



//////////////////////////////////////////////////////////
//////////////     vector divisions     //////////////////
//////////////////////////////////////////////////////////

/// Components wise division
/// v1 / v2 = [v1.x / v2.x, v1.y / v2.y]
impl std::ops::Div<LVector2> for LVector2 {
    type Output = LVector2;

    fn div(self, rhs: LVector2) -> Self::Output {
        LVector2 {
            data: self.data / rhs.data,
        }
    }
}

/// Components wise division
/// v1 / v2 = [v1.x / v2.x, v1.y / v2.y]
impl std::ops::Div<&LVector2> for LVector2 {
    type Output = LVector2;

    fn div(self, rhs: &LVector2) -> Self::Output {
        self.div(*rhs)
    }
}

/// Components wise division
/// v1 / v2 = [v1.x / v2.x, v1.y / v2.y]
impl std::ops::Div<LVector2> for &LVector2 {
    type Output = LVector2;

    fn div(self, rhs: LVector2) -> Self::Output {
        (*self).div(rhs)
    }
}

/// Components wise division
/// v1 / v2 = [v1.x / v2.x, v1.y / v2.y]
impl std::ops::Div<&LVector2> for &LVector2 {
    type Output = LVector2;

    fn div(self, rhs: &LVector2) -> Self::Output {
        (*self).div(*rhs)
    }
}

/// Divide all components of the vector by `rhs` 
/// v1 / f = [v1.x / f, v1.y / f]
impl std::ops::Div<i64> for LVector2 {
    type Output = LVector2;

    fn div(self, rhs: i64) -> Self::Output {
        LVector2 {
            data: self.data / i64x2::from_array([rhs, rhs])
        }
    }
}

/// Divide all components of the vector by `rhs` 
/// v1 / f = [v1.x / f, v1.y / f]
impl std::ops::Div<i64> for &LVector2 {
    type Output = LVector2;

    fn div(self, rhs: i64) -> Self::Output {
        (*self).div(rhs)
    }
}

/// Divide all components of the vector by `rhs` 
/// v1 / f = [v1.x / f, v1.y / f]
impl std::ops::Div<&i64> for LVector2 {
    type Output = LVector2;

    fn div(self, rhs: &i64) -> Self::Output {
        self.div(*rhs)
    }
}

/// Divide all components of the vector by `rhs` 
/// v1 / f = [v1.x / f, v1.y / f]
impl std::ops::Div<&i64> for &LVector2 {
    type Output = LVector2;

    fn div(self, rhs: &i64) -> Self::Output {
        (*self).div(*rhs)
    }
}



//////////////////////////////////////////////////////////
//////////////      vector assign       //////////////////
//////////////////////////////////////////////////////////
 
/// Components wise addition
/// v1 += v2 <=> v1 = [v1.x + v2.x, v1.y + v2.y]
impl std::ops::AddAssign<LVector2> for LVector2 {
    fn add_assign(&mut self, rhs: LVector2) {
        self.data = self.data + rhs.data;
    }
}

/// Components wise addition
/// v1 += v2 <=> v1 = [v1.x + v2.x, v1.y + v2.y]
impl std::ops::AddAssign<&LVector2> for LVector2 {
    fn add_assign(&mut self, rhs: &LVector2) {
        self.add_assign(*rhs);
    }
}

/// Add `rhs` to all components of the vector 
/// v += f <=> v = [v*x + f, v*y + f]
impl std::ops::AddAssign<i64> for LVector2 {
    fn add_assign(&mut self, rhs: i64) {
        self.data = self.data + i64x2::from_array([rhs, rhs]);
    }
}

/// Add `rhs` to all components of the vector 
/// v += f <=> v = [v*x + f, v*y + f]
impl std::ops::AddAssign<&i64> for LVector2 {
    fn add_assign(&mut self, rhs: &i64) {
        self.add_assign(*rhs);
    }
}

/// Components wise substraction
/// v1 -= v2 <=> v1 = [v1.x - v2.x, v1.y - v2.y]
impl std::ops::SubAssign<LVector2> for LVector2 {
    fn sub_assign(&mut self, rhs: LVector2) {
        self.data = self.data - rhs.data;
    }
}

/// Components wise substraction
/// v1 -= v2 <=> v1 = [v1.x - v2.x, v1.y - v2.y]
impl std::ops::SubAssign<&LVector2> for LVector2 {
    fn sub_assign(&mut self, rhs: &LVector2) {
        self.sub_assign(*rhs);
    }
}

/// Substract `rhs` to all components of the vector 
/// v -= f <=> v = [v*x - f, v*y - f]
impl std::ops::SubAssign<i64> for LVector2 {
    fn sub_assign(&mut self, rhs: i64) {
        self.data = self.data - i64x2::from_array([rhs, rhs]);
    }
}

/// Substract `rhs` to all components of the vector 
/// v -= f <=> v = [v*x - f, v*y - f]
impl std::ops::SubAssign<&i64> for LVector2 {
    fn sub_assign(&mut self, rhs: &i64) {
        self.sub_assign(*rhs);
    }
}

/// Components wise multiplication
/// v1 *= v2 <=> v1 = [v1.x * v2.x, v1.y * v2.y]
impl std::ops::MulAssign<LVector2> for LVector2 {
    fn mul_assign(&mut self, rhs: LVector2) {
        self.data = self.data * rhs.data;
    }
}

/// Components wise multiplication
/// v1 *= v2 <=> v1 = [v1.x * v2.x, v1.y * v2.y]
impl std::ops::MulAssign<&LVector2> for LVector2 {
    fn mul_assign(&mut self, rhs: &LVector2) {
        self.mul_assign(*rhs);
    }
}

/// Multiply all components of the vector by `rhs` 
/// v *= f <=> v = [v*x / f, v*y / f]
impl std::ops::MulAssign<i64> for LVector2 {
    fn mul_assign(&mut self, rhs: i64) {
        self.data = self.data * i64x2::from_array([rhs, rhs]);
    }
}

/// Multiply all components of the vector by `rhs` 
/// v *= f <=> v = [v*x / f, v*y / f]
impl std::ops::MulAssign<&i64> for LVector2 {
    fn mul_assign(&mut self, rhs: &i64) {
        self.mul_assign(*rhs);
    }
}

/// Components wise division
/// v1 /= v2 <=> v1 = [v1.x / v2.x, v1.y / v2.y]
impl std::ops::DivAssign<LVector2> for LVector2 {
    fn div_assign(&mut self, rhs: LVector2) {
        self.data = self.data / rhs.data;
    }
}

/// Components wise division
/// v1 /= v2 <=> v1 = [v1.x / v2.x, v1.y / v2.y]
impl std::ops::DivAssign<&LVector2> for LVector2 {
    fn div_assign(&mut self, rhs: &LVector2) {
        self.div_assign(*rhs);
    }
}

/// Divide all components of the vector by `rhs` 
/// v /= f <=> v1 = [v.x / f, v.y / f]
impl std::ops::DivAssign<i64> for LVector2 {
    fn div_assign(&mut self, rhs: i64) {
        self.data = self.data / i64x2::from_array([rhs, rhs]);
    }
}

/// Divide all components of the vector by `rhs` 
/// v /= f <=> v = [v.x / f, v.y / f]
impl std::ops::DivAssign<&i64> for LVector2 {
    fn div_assign(&mut self, rhs: &i64) {
        self.div_assign(*rhs);
    }
}


//////////////////////////////////////////////////////////
///////////////     vector tests      ////////////////////
//////////////////////////////////////////////////////////
#[cfg(test)]
mod tests {
    use super::*;

    /// Test of 2 dimensional vector initialization
    #[test]
    fn initialization() {
        let v1 = ivec2(1, 1);
        let v2 = LVector2::ONES;
        assert_eq!(v1, v2);
    }

    /// Test of 2 dimensional vector operators
    #[test]
    fn operators() {
        let v1 = ivec2(2, 3);
        let v2 = ivec2(4, 5);
        assert_eq!(v1 + v2, ivec2(6, 8));
        let mut v3 = v1;
        assert_eq!(v1, v3);
        v3 += v1;
        assert_eq!(v3, ivec2(4, 6));

        v3 += 1;
        assert_eq!(v3, ivec2(5, 7));
        assert_eq!(v3+v3-v3, v3);

        assert_eq!(v3*2, ivec2(10, 14));
        assert_eq!(v1*v2, ivec2(8, 15));

        assert_eq!(v3/v3, LVector2::ONES);
        v3 /= v3;
        assert_eq!(v3, LVector2::ONES);
    }

    /// Test of 2 dimensional vector operations
    #[test]
    fn operations(){
        let v1 = ivec2(1, 2);
        let v2 = ivec2(4, 3);
        assert_eq!(LVector2::dot(&v1, &v2), 10);
        let vx = LVector2::X;
        let vy = LVector2::Y;
        assert_eq!(LVector2::dot(&vx, &vy), 0);

        assert_eq!(vx.length(), 1.);
        assert_eq!(vy.length(), 1.);
        assert_eq!(v1.length(), 5f32.sqrt());
    }

    /// Test of 2 dimensional vector's fields getters and setters
    #[test]
    fn deref() {
        let v1 = ivec2(5, 2);
        assert_eq!(v1.x, 5);
        assert_eq!(v1.y, 2);
        let mut v2 = LVector2::ZEROS;
        assert_eq!(v2.x, 0);
        assert_eq!(v2.y, 0);
        v2.x = 1;
        assert_eq!(v2.x, 1);
        v2.y = 3;
        assert_eq!(v2.y, 3);
        assert_eq!(v2.x, 1);
    }

    /// Test the formatting of 2 dimensional vectors
    #[test]
    fn format() {
        let v1 = ivec2(3, 4);
        assert_eq!(v1.to_string(), "(3, 4)");
        assert_eq!(format!("{:?}", v1), "LVector2 { x: 3, y: 4 }");
    }
}
