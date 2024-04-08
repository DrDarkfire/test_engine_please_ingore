use std::ops::{Add, AddAssign, Mul, Sub, SubAssign};
/// Linear
/// Linear Algebra utility structs for both internal engine use and for games dumbed down to handle only what we need.
/// Could we just use someone else's library for some of these things? Yes, but we're trying to learn.
/// 

/// # Position
/// Primarily for UI/HUD element use. This is for when we only need to deal with integers and don't need incredibly specific coords.
#[derive(Debug, Copy, Clone)]
#[allow(unused)]
pub struct Postion {
    x: u32,
    y: u32,
}

/// # Pos2D
/// Primarily for giving nodes accurate positons in 2D games.
#[derive(Debug, Copy, Clone)]
pub struct Pos2D {
    pub x: f32,
    pub y: f32,
}

pub const fn pos2d(x: f32, y: f32) -> Pos2D {
    Pos2D::new(x, y)
}

/// # Pos3D
/// Primarily for giving nodes accurate positons in 3D games.
#[derive(Debug, Copy, Clone)]
#[allow(unused)]
pub struct Pos3D {
    x: f32,
    y: f32,
    z: f32,
}

#[derive(Debug, Copy, Clone)]
pub struct Vec2D {
    dx: f32,
    dy: f32,
}

#[allow(unused)]
#[derive(Debug, Copy, Clone)]
pub struct Vec3D {
    dx: f32,
    dy: f32,
    dz: f32,
}

/// Impl blocks for Pos2D and Vec2D
impl Add<Vec2D> for Pos2D {
    type Output = Pos2D;

    fn add(self, rhs: Vec2D) -> Self {
        Pos2D {
            x: self.x + rhs.dx,
            y: self.y + rhs.dy,
        }
    }
}

impl AddAssign<Vec2D> for Pos2D {
    fn add_assign(&mut self, rhs: Vec2D) {
        self.x += rhs.dx;
        self.y += rhs.dy;
    }
}

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

    pub fn x(&self) -> f32 {
        self.x
    }

    pub fn y(&self) -> f32 {
        self.y
    }

    /// returns a new point with the min x and min y between the two points
    pub fn min(self, rhs: Pos2D) -> Pos2D {
        Self {
            x: self.x.min(rhs.x),
            y: self.y.min(rhs.y),
        }
    }

    pub fn max(self, rhs: Pos2D) -> Pos2D {
        Self {
            x: self.x.max(rhs.x),
            y: self.y.max(rhs.y),
        }
    }

    pub fn translate_x(&mut self, tx: f32) {
        self.translate(tx, 0.0)
    }

    pub fn translate_y(&mut self, ty: f32) {
        self.translate(0.0, ty)
    }

    pub fn translate(&mut self, x: f32, y: f32) {
        self.x += x;
        self.y += y;
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
        self.translate(tx, 0.0, 0.0)
    }

    pub fn translate_y(&mut self, ty: f32) {
        self.translate(0.0, ty, 0.0)
    }

    pub fn translate_z(&mut self, tz: f32) {
        self.translate(0.0, 0.0, tz)
    }

    pub fn translate(&mut self, x: f32, y: f32, z: f32) {
        self.x += x;
        self.y += y;
        self.z += z;
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