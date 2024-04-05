use crate::{graphics::texture::Material, util::linear::{Pos2D, Pos3D, Vec3D}};

pub struct Node {
    parent: Option<Box<Node>>,
    children: Vec<Node>,
    id: String,
    name: String,
    texture: Material,
    scene: Scene,
}

pub struct Camera2D {
    node: Node,
    pos: Pos2D,
}

pub struct Camera3D {
    node: Node,
    pos: Pos3D,
    direction: Direction3D
}

pub struct Direction3D {
    x: f32,
    y: f32,
    z: f32,
}

pub struct Scene {
    parent_node: Box<Node>,
}

impl Node {
    /// Returns parent node if it exists
    // pub fn parent(&self) -> Option<Box<Node>> {
    //     if self.parent.is_some() {
    //         self.parent
    //     } else {
    //         None
    //     }
    // }
    pub fn parent(&self) -> Option<&Node> {
        // as_ref turns it into a borrowed value
        // if as_ref returns some &Box<Node> we unbox the node and return Some &Node or None
        self.parent.as_ref().map(|parent_box| &**parent_box) // deref Option and Box
    }

    /// If children is empty, it returns None.
    /// Else, it returns the list of children.
    pub fn children(&self) -> Option<&Vec<Node>> {
        if self.children.is_empty() {
            None
        } else {
            Some(self.children.as_ref())
        }
    }
}

impl Direction3D {
    pub const NORTH: Direction3D = Direction3D { x: 1.0, y: 0.0, z: 0.0};
    pub const SOUTH: Direction3D = Direction3D { x:-1.0, y: 0.0, z: 0.0};
    pub const EAST:  Direction3D = Direction3D { x: 0.0, y: 0.0, z: 1.0};
    pub const WEST:  Direction3D = Direction3D { x: 0.0, y: 0.0, z:-1.0};
    pub const UP:    Direction3D = Direction3D { x: 0.0, y: 1.0, z: 0.0};
    pub const DOWN:  Direction3D = Direction3D { x: 0.0, y:-1.0, z: 0.0};

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