#[allow(unused)]
use crate::{graphics::texture::Material, util::linear::{Pos2D, Pos3D, Vec3D}};

#[allow(unused)]
pub struct Node {
    parent: Option<Box<Node>>,
    children: Vec<Node>,
    id: String,
    name: String,
    texture: Material,
    scene: Scene,
}

#[allow(unused)]
pub struct Camera2D {
    node: Node,
    pos: Pos2D,
}

#[allow(unused)]
pub struct Camera3D {
    node: Node,
    pos: Pos3D,
    direction: Direction3D
}

#[allow(unused)]
pub struct Direction3D {
    x: f32,
    y: f32,
    z: f32,
}

#[allow(unused)]
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

impl Camera3D {
    pub fn translate_x(&mut self, tx: f32) {
        self.pos.translate_x(tx);
    }

    pub fn translate_y(&mut self, ty: f32) {
        self.pos.translate_y(ty);
    }

    pub fn translate_z(&mut self, tz: f32) {
        self.pos.translate_z(tz);
    }

    pub fn translate(&mut self, x: f32, y: f32, z: f32) {
        self.pos.translate(x, y, z);
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