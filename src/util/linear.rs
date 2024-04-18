use std::ops::{Add, AddAssign, Mul, Sub, SubAssign};
use std::fmt;
/// Linear
/// Linear Algebra utility structs for both internal engine use and for games dumbed down to handle only what we need.
/// Could we just use someone else's library for some of these things? Yes, but we're trying to learn.
/// 

/// ## Position
/// Primarily for UI/HUD element use. This is for when we only need to deal with integers and don't need incredibly specific coords.
#[derive(Debug, Copy, Clone)]
#[allow(unused)]
pub struct Postion {
    x: u32,
    y: u32,
}

/// ## Pos2D
/// Primary struct for giving nodes accurate positons and movement in 2D games.
#[derive(Debug, Copy, Clone)]
pub struct Pos2D {
    x: f32,
    y: f32,
}

pub const fn pos2d(x: f32, y: f32) -> Pos2D {
    Pos2D::new(x, y)
}

/// ## Pos3D
/// Primary struct for giving nodes accurate positons and movement in 3D games.
#[derive(Debug, Copy, Clone)]
#[allow(unused)]
pub struct Pos3D {
    x: f32,
    y: f32,
    z: f32,
}

/// ## Vec2D
/// struct to manipulate a pos in a 2D space
#[derive(Debug, Copy, Clone)]
pub struct Vec2D {
    dx: f32,
    dy: f32,
}

/// ## Vec3D
/// struct to manipulate a pos in a 3D same 
#[allow(unused)]
#[derive(Debug, Copy, Clone)]
pub struct Vec3D {
    dx: f32,
    dy: f32,
    dz: f32,
}

/// ## Impl blocks for Pos2D and Vec2D
/// 

/// translate a Pos2D by adding a vector
impl Add<Vec2D> for Pos2D {
    type Output = Pos2D;

    fn add(self, rhs: Vec2D) -> Self {
        Pos2D {
            x: self.x + rhs.dx,
            y: self.y + rhs.dy,
        }
    }
}

/// translate and assign a Pos2D by adding a vector
impl AddAssign<Vec2D> for Pos2D {
    fn add_assign(&mut self, rhs: Vec2D) {
        *self = Self {
            x: self.x + rhs.dx(),
            y: self.y + rhs.dy()
        }
    }
}

/// multiplies two points
impl Mul<Pos2D> for Pos2D {
    type Output = Pos2D;

    fn mul(self, rhs: Pos2D) -> Self{
        Pos2D {
            x: self.x.mul(rhs.x),
            y: self.y.mul(rhs.y),
        }
    }
}

impl Pos2D {
    pub const ZERO: Pos2D = Pos2D {x: 0.0, y: 0.0};
    pub const ONE: Pos2D = Pos2D {x: 1.0, y: 1.0};

    pub const fn new(x:f32 , y: f32) -> Self {
        Self { x, y }
    }

    // sorry attempt at a deref
    pub const fn new_from_self(&self) -> Self {
        Self::new(self.x, self.y)
    }

    /// getter for the x data member
    pub fn x(&self) -> f32 {
        self.x
    }

    /// getter for the y data member
    pub fn y(&self) -> f32 {
        self.y
    }

    /// assign a new x value
    pub fn set_x(&mut self, x: f32) {
        self.x = x
    }

    /// assign a new y value
    pub fn set_y(&mut self, y: f32) {
        self.y = y
    }

    /// assign a new x and y value
    pub fn set(&mut self, x: f32, y: f32) {
        self.x = x;
        self.y = y;
    }

    /// returns a new point with the min x and min y between self and another point
    pub fn min(self, rhs: Pos2D) -> Pos2D {
        Self {
            x: self.x.min(rhs.x),
            y: self.y.min(rhs.y),
        }
    }

    /// returns a new point with the max x and max y between self and another point
    pub fn max(self, rhs: Pos2D) -> Pos2D {
        Self {
            x: self.x.max(rhs.x),
            y: self.y.max(rhs.y),
        }
    }

    /// translates a point instantly given an x distance (tx)
    pub fn translate_x(&mut self, tx: f32) {
        self.x += tx
        // while this method is possible it is not necessary
        //*self = *self + Vec2D::new(tx, 0.0);
    }

    /// translates a point instantly given a y distance (ty)
    pub fn translate_y(&mut self, ty: f32) {
        self.y += ty
    }

    /// translates a point instantly given an x and y distance (tx, ty)
    pub fn translate(&mut self, tx: f32, ty: f32) {
        self.x += tx;
        self.y += ty;
    }

    /// lerp covers the states from start to end of the distance between a start point and and end point where t is the % completion 
    /// 
    /// lerp function can be simplified to
    /// 
    /// lerp(a, b, t) = a + (b - a) * t
    /// 
    /// lerp pseudo-code and resource: https://docs.godotengine.org/en/stable/tutorials/math/interpolation.html
    pub fn lerp(start: Pos2D, end: Pos2D, t: f32) -> Pos2D {
        Pos2D {
            x: start.x() + (end.x() - start.x()) * t,
            y: start.y() + (end.y() - start.y()) * t,
        }
    }

    /// debug method for lerp
    /// 
    /// can also be used to help create transitions between two points
    /// 
    /// the returned Vec will contain the start and end pos 
    /// 
    /// steps is inclusive to the end
    pub fn lerp_steps(start: Pos2D, end: Pos2D, steps: u32) -> Vec<Pos2D> {
        let mut v: Vec<Pos2D> = Vec::new();
        for i in 0..steps + 1 {
            v.push(Pos2D::lerp(start, end, i as f32 / steps as f32));
        }
        v
    }

    /// 2D cartesian to polar coords
    pub fn cartesian_to_polar(self) -> (f32, f32) {
        let r = (self.x().powi(2) + self.y().powi(2)).sqrt();
        let theta = f32::atan2(self.y(), self.x());
        (r, theta)
    }
}

impl Pos3D {
    pub const fn new(x:f32 , y: f32, z: f32) -> Self {
        Self { x, y, z }
    }
    pub fn x(&self) -> f32 {
        self.x
    }

    pub fn y(&self) -> f32 {
        self.y
    }

    pub fn z(&self) -> f32 {
        self.z
    }

    /// assign a new x value
    pub fn set_x(&mut self, x: f32) {
        self.x = x
    }

    /// assign a new y value
    pub fn set_y(&mut self, y: f32) {
        self.y = y
    }

    pub fn set_z(&mut self, z: f32) {
        self.z = z
    }

    /// assign a new x and y value
    pub fn set(&mut self, x: f32, y: f32, z: f32) {
        *self = Self {
            x: x,
            y: y,
            z: z,
        }
    }

    /// returns a new point with the min x and min y between the two points
    pub fn min(self, rhs: Pos3D) -> Pos3D {
        Self {
            x: self.x.min(rhs.x),
            y: self.y.min(rhs.y),
            z: self.z.min(rhs.z),
        }
    }

    pub fn max(self, rhs: Pos3D) -> Pos3D {
        Self {
            x: self.x.max(rhs.x),
            y: self.y.max(rhs.y),
            z: self.z.max(rhs.z),
        }
    }

    pub fn translate_x(&mut self, tx: f32) {
        self.x += tx
    }

    pub fn translate_y(&mut self, ty: f32) {
        self.y += ty
    }

    pub fn translate_z(&mut self, tz: f32) {
        self.z += tz
    }

    pub fn translate(&mut self, tx: f32, ty: f32, tz: f32) {
        *self = *self + Vec3D::new(tx, ty, tz)
    }

    pub fn lerp(start: Pos3D, end: Pos3D, t: f32) -> Pos3D {
        Pos3D {
            x: start.x() + (end.x() - start.x()) * t,
            y: start.y() + (end.y() - start.y()) * t,
            z: start.z() + (end.z() - start.z()) * t,
        }
    }
}

impl Add<Vec3D> for Pos3D {
    type Output = Pos3D;

    fn add(self, rhs: Vec3D) -> Self {
        Pos3D {
            x: self.x + rhs.dx,
            y: self.y + rhs.dy,
            z: self.z + rhs.dz,
        }
    }
}

impl AddAssign<Vec3D> for Pos3D {
    fn add_assign(&mut self, rhs: Vec3D) {
        self.x += rhs.dx;
        self.y += rhs.dy;
        self.z += rhs.dz;
    }
}

/// Scalar multiplication for Vec2D
impl Mul<f32> for Vec2D {
    type Output = Vec2D;
    
    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            dx: self.dx * rhs,
            dy: self.dy * rhs,
        }
    }
}

/// Vector addition for Vec2D
impl Add<Vec2D> for Vec2D {
    type Output = Vec2D;

    fn add(self, rhs: Vec2D) -> Self::Output {
        Self {
            dx: self.dx + rhs.dx,
            dy: self.dy + rhs.dy
        }
    }
}

/// Vector addition for Vec2D with assignment
impl AddAssign<Vec2D> for Vec2D {
    fn add_assign(&mut self, rhs: Vec2D) {
        self.dx += rhs.dx;
        self.dy += rhs.dy;
    }
}

impl Sub<Vec2D> for Vec2D {
    type Output = Vec2D;

    fn sub(self, rhs: Vec2D) -> Self::Output {
        Self {
            dx: self.dx - rhs.dx,
            dy: self.dy - rhs.dy
        }
    }
}

impl SubAssign<Vec2D> for Vec2D {
    fn sub_assign(&mut self, rhs: Vec2D) {
        self.dx -= rhs.dx;
        self.dy -= rhs.dy;
    }
}

impl Vec2D {
    pub fn new(dx: f32, dy: f32) -> Vec2D {
        Vec2D {
            dx: dx,
            dy: dy,
        }
    }

    pub fn dx(&self) -> f32 {
        self.dx
    }

    pub fn dy(&self) -> f32 {
        self.dy
    }

    /// Convert Vec2D to Vec3D with a given dz
    pub fn to_vec3d(self, dz: f32) -> Vec3D {
        Vec3D {
            dx: self.dx,
            dy: self.dy,
            dz: dz
        }
    }
}

impl Vec3D {
    pub const ZERO: Pos3D = Pos3D {x: 0.0, y: 0.0, z: 0.0};
    pub const ONE: Pos3D = Pos3D {x: 1.0, y: 1.0, z: 1.0};

    pub fn new(dx: f32, dy: f32, dz: f32) -> Self {
        Self { 
            dx: dx,
            dy: dy,
            dz: dz,
        }
    }
}

// Equality blocks we probably should've just derived them but it's too late now.
impl PartialEq for Pos2D {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }

    fn ne(&self, other: &Self) -> bool {
        self.x != other.x || self.y != other.y
    }
}

impl PartialEq for Pos3D {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }

    fn ne(&self, other: &Self) -> bool {
        self.x != other.x || self.y != other.y || self.z != other.z
    }
}

impl PartialEq for Vec2D {
    fn eq(&self, other: &Self) -> bool {
        self.dx == other.dx && self.dy == other.dy
    }

    fn ne(&self, other: &Self) -> bool {
        self.dx != other.dx || self.dy != other.dy
    }
}

impl PartialEq for Vec3D {
    fn eq(&self, other: &Self) -> bool {
        self.dx == other.dx && self.dy == other.dy && self.dz == other.dz
    }

    fn ne(&self, other: &Self) -> bool {
        self.dx != other.dx || self.dy != other.dy || self.dz != other.dz
    }
}

// Formatting blocks
impl fmt::Display for Pos2D {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Pos2D: {{ x: {}, y: {} }}", self.x, self.y)
    }
}

impl fmt::Display for Pos3D {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Pos3D: {{ x: {}, y: {}, z: {} }}", self.x, self.y, self.z)
    }
}

impl fmt::Display for Vec2D {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Vec2D: {{ dx: {}, dy: {} }}", self.dx, self.dy)
    }
}

impl fmt::Display for Vec3D {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Vec3D: {{ dx: {}, dy: {}, dz: {} }}", self.dx, self.dy, self.dz)
    }
}

// impl fmt::Display for Vec<Pos2D> {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         for v in 0..self.len() {
//             writeln!(f, "Step {}: {}", v, self.get(v))
//         }
//     }
// }