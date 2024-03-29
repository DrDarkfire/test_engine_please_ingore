use std::ops::{Add, Mul};
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
}