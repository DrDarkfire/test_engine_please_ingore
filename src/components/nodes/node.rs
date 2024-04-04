use crate::util::linear::{Pos2D, Pos3D, Vec3D};

pub struct Node {
    parent: Option<Node>,
    children: Vec<Node>,
    id: String,
    name: String,
    texture: Material,
    scene: Scene
}

pub struct Camera2D {
    node: Node,
    pos: Pos2D,
}

pub struct Camera3D {
    node: Node,
    pos: Pos3D,
    direction: Direction
}

pub struct Direction3D {
    x: f32,
    y: f32,
    z: f32,
}

impl Node {
    /// Returns parent node if it exists
    pub fn parent(&self) -> Option<Node> {
        if Some(self.parent) {
            self.parent
        } else {
            None
        }
    }

    /// If children is empty, it returns None.
    /// Else, it returns the list of children.
    pub fn children(&self) -> Option<Vec<Node>> {
        if self.children.is_empty() {
            None
        } else {
            Some(self.children)
        }
    }
}

impl Direction3D {
    pub const NORTH: Direction3D = Direction3D::new(1, 0, 0);
    pub const SOUTH: Direction3D = Direction3D::new(-1, 0, 0);
    pub const EAST:  Direction3D = Direction3D::new(0, 0, 1);
    pub const WEST:  Direction3D = Direction3D::new(0, 0, -1);
    pub const UP:    Direction3D = Direction3D::new(0, 1, 0);
    pub const DOWN:  Direction3D = Direction3D::new(0, -1, 0);

    pub fn new(x: f32, y: f32, z: f32) -> Self {
        // normalize the vector in case we're given funny numbers
        let mag = (x.powi(2) + y.powi(2) + z.powi(2)).sqrt();
        Self {
            x: x / mag,
            y: y / mag,
            z: z / mag,
        }
    }
}