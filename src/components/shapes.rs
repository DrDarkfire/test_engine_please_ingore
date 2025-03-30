use crate::{graphics::window_minifb::Framebuffer, util::{color, linear::Pos2D}};

use super::nodes::node::Camera2D;

/// The shape module will provide basic shapes for the game engine.
/// 
/// in the context of the game engine this will aide in UI construction
/// 
/// Shape trait with functions required of every shape (i.e. draw, inside_eh)
/// This is to separate functions specific to the shape.
/// We will need access functions inside the individual shape's impl blocks for type safety purposes

/// Shape Trait
/// Core functions related to drawing a scene
trait Shape {
    type ReturnType;
    /// draw
    /// sets the pixels the appropriate pixels to the defined color and eventually texture
    fn draw(&self, framebuffer: &mut Framebuffer, color: u32);

    /// inside_eh / is_inside
    /// checks if a point(Pos2D) is inside the given shape.
    fn inside_eh(&self, p: &Pos2D) -> bool;

    /// translate
    /// moves the shape immediately a given x and y
    fn translate(&mut self, tx: f32, ty: f32);

    /// lerp_translate
    /// moves the shape gradually a given x and y with t being % completion
    fn lerp_translate(&mut self, tx: f32, ty: f32, t: f32);


    /// lerp_steps_translate
    /// moves the shape gradually a given x and y with t being # of steps
    fn lerp_steps_translate(&mut self, tx: f32, ty: f32, t: u32);

    /// transform
    /// mutates the shape immediately to the given points
    /// 
    /// will eventually have an elegant way of changing shapes
    fn transform(&mut self, points: Vec<Pos2D>);

    /// lerp_transform
    /// mutates the shape gradually to the given points with t% completion
    ///
    /// will eventually have an elegant way of changing shapes
    fn lerp_transform(&mut self, t: f32, points: Vec<Pos2D>);

    /// lerp_steps_transform
    /// mutates the shape gradually to the given points with t steps
    fn lerp_steps_transform(&mut self, t: f32, points: Vec<Pos2D>);
}

pub struct Triangle {
    a: Pos2D,
    b: Pos2D,
    c: Pos2D,
}

impl Triangle {
    pub fn new(a: Pos2D, b: Pos2D, c: Pos2D) -> Triangle {
        Triangle{ a, b, c }
    }

    /// if edge_function returns positive we are inside the triangle, if not we are outside
    pub fn edge_function(&self, a: &Pos2D, c: &Pos2D, b: &Pos2D) -> f32 {
        // we force passing in references to minimize copying things we don't need to. 
        (c.x() - a.x()) * (b.y()- a.y()) - (c.y() - a.y()) * (b.x() - a.x())
    }
    
    // TRAIT ACCESS FUNCTIONS
    pub fn inside_eh(&self, p: &Pos2D) -> bool {
        <Self as Shape>::inside_eh(self, p)
    }

    // access function for drawing relative to framebuffer
    pub fn draw(&self, framebuffer: &mut Framebuffer, color: u32) {
        <Self as Shape>::draw(self, framebuffer, color)
    }

    // we are using draw_abs to pilot drawing the scene with the camera in mind
    pub fn draw_abs(&self, framebuffer: &mut Framebuffer, color: u32, camera: &Camera2D) {
        let width = framebuffer.width() as f32;
        let height = framebuffer.height() as f32;
        // don't do draw calculations if not in viewport
        // print!("in draw_abs");
        if !self.render_guard(&camera, width, height) {
            return;
        }
        // get the bottom left and top right of our viewport
        let (bl, tr) = camera.viewport(width, height);

        // find our offset
        let offset = Pos2D::new(bl.x(), bl.y());
        // print!("our offset is {}", offset);

        // get the bounding box and and adjust to not care about anything outside of the viewport
        let mut bounds = self.bounds();
        bounds.0 = bounds.0.max(bl);
        bounds.1 = bounds.1.min(tr);

        // we can now start drawing the triangle
        // double for loop uses the bounds as the range and we'll adjust pixels relative to the fb
        for x in (bounds.0.x() as usize)..(bounds.1.x() as usize) {
            for y in (bounds.0.y() as usize)..(bounds.1.y() as usize) {
                // current point we're evaluating
                let p = Pos2D::new(x as f32, y as f32);
                if self.inside_eh(&p) {
                    framebuffer.set_pixel(x + (offset.x() as usize), y + (offset.y() as usize), color)
                }
            }
        }

    }

    // returns a Pos2D bounding box tuple with first being min and second being max ignoring the viewport
    pub fn bounds(&self) -> (Pos2D, Pos2D) {
        let p_min: Pos2D = self.a.min(self.b.min(self.c));
        let p_max: Pos2D = self.a.max(self.b.max(self.c));
        (p_min, p_max)
    }

    // if true we can draw the triangle
    pub fn render_guard(&self, cam: &Camera2D, width: f32, height: f32) -> bool {
        // get our mins and maxes for the box formed by the triangle
        let (p_min, p_max) = self.bounds();
        // adjust width and height to be half for less messy conditionals
        let w = width / 2.0; // 2.0 and not 2 bc vscode type inference was scary looking
        let h = height / 2.0;
        let cx = cam.pos().x();
        let cy = cam.pos().y();
        // check our x vals
        if p_min.x() < (cx + width) || p_max.x() > (cx - width) {
            if p_min.y() < (cy + height) || p_max.y() > (cy - height) {
                return true;
            }
        }

        return false;
    }

    pub fn translate_a(&mut self, tx: f32, ty: f32) {
        self.a.translate(tx, ty)
    }

    pub fn translate_b(&mut self, tx: f32, ty: f32) {
        self.b.translate(tx, ty)
    }

    pub fn translate_c(&mut self, tx: f32, ty: f32) {
        self.c.translate(tx, ty)
    }
}

impl Shape for Triangle {
    type ReturnType = Triangle;

    /// Draw the triangle on the screen on top of all previous pixels relative to the height/width of the fb
    /// 
    /// Later we will have an alpha channel to deal with transparency coloring
    fn draw(&self, framebuffer: &mut Framebuffer, color: u32) {
        let width = framebuffer.width();
        let height = framebuffer.height();

        // create a bounding box over the triangle so we don't perform checks on every coord
        // find the smallest value and if < 0 change it. find largest value and if > 1 change it. Then we convert them to pixels.
        // something to consider later adjusting positions and conditional rendering
        let min = self.a.min(self.b.min(self.c)).max(Pos2D::ZERO) * Pos2D::new(width as f32, height as f32);
        let max = self.a.max(self.b.max(self.c)).min(Pos2D::ONE) * Pos2D::new(width as f32, height as f32);

        for x in (min.x() as usize)..(max.x() as usize) {
            for y in (min.y() as usize)..(max.y() as usize) {
                // create a point for each pixel we are attempting to modify within 0 - 1
                let p = Pos2D::new(x as f32 / width as f32, y as f32 / height as f32);

                if self.inside_eh(&p) {
                    framebuffer.set_pixel(x, y, color);
                }
            }
        }
    }
    // works with CLOCKWISE winding order
    // checks if a point is inside the triangle or not via edge functions
    fn inside_eh(&self, p: &Pos2D) -> bool {
        let a0 = self.edge_function(&self.b, &self.c, p);
        let a1 = self.edge_function(&self.c, &self.a, p);
        let a2 = self.edge_function(&self.a, &self.b, p);
        
        let mut overlaps = true;
        overlaps &= a0 > 0.0;
        overlaps &= a1 > 0.0;
        overlaps &= a2 > 0.0;

        overlaps
    }

    fn translate(&mut self, tx: f32, ty: f32) {
        self.a.translate(tx, ty);
        self.b.translate(tx, ty);
        self.c.translate(tx, ty);
    }

    fn lerp_translate(&mut self, tx: f32, ty: f32, t: f32) {
        
    }

    fn lerp_steps_translate(&mut self, tx: f32, ty: f32, t: u32) {
        
    }

    fn transform(&mut self, points: Vec<Pos2D>) {
        self.a = Pos2D::new_from_other(*points.get(0).expect("missing point"));
        self.b = Pos2D::new_from_other(*points.get(1).expect("missing point"));
        self.c = Pos2D::new_from_other(*points.get(2).expect("missing point"));
    }

    fn lerp_transform(&mut self, t: f32, points: Vec<Pos2D>) {
        
    }

    fn lerp_steps_transform(&mut self, t: f32, points: Vec<Pos2D>) {
        
    }
}

pub struct Rect {
    a: Pos2D,
    b: Pos2D,
    c: Pos2D,
    d: Pos2D,
    length: f32,
    height: f32
}

impl Rect {
    // pub fn new(a: Pos2D, b: Pos2D, c: Pos2D, d: Pos2D) -> Rect {
    //     Rect { a, b, c, d }
    // }
    /// create rect based on initial point(top left), length, & height *without* rotation
    /// we use height to reduce confusion on which is x and y
    pub fn new(point: Pos2D, length: f32, height: f32) -> Rect {
        Rect { 
            a: point, 
            b: Pos2D::new(point.x() + length, point.y()), 
            c: Pos2D::new(point.x() + length, point.y() - height), 
            d: Pos2D::new(point.x(), point.y() - height),
            length: (length),
            height: (height)
        }
    }

    pub fn a(&self) -> Pos2D {
        self.a
    }

    pub fn b(&self) -> Pos2D {
        self.b
    }

    pub fn c(&self) -> Pos2D {
        self.c
    }

    pub fn d(&self) -> Pos2D {
        self.d
    }

    // TRAIT ACCESS FUNCTIONS
    pub fn inside_eh(&self, p: &Pos2D) -> bool {
        <Self as Shape>::inside_eh(self, p)
    }

    pub fn render_guard(&self, cam: &Camera2D, width: f32, height: f32) -> bool {
        true
    }

    pub fn draw(&self, framebuffer: &mut Framebuffer, color: u32) {
        <Self as Shape>::draw(self, framebuffer, color)
    }

    pub fn draw_abs(&self, framebuffer: &mut Framebuffer, color: u32, cam: &Camera2D) {
        let width = framebuffer.width();
        let height = framebuffer.height();

        if !self.render_guard(cam, width as f32, height as f32) {
            return;
        }

        let (mut bl, mut tr) = cam.viewport(width as f32, height as f32);

        let offset = Pos2D::new(bl.x(), bl.y());
        // in a rect, d is min and b is max so we'll adjust bl and tr to be the max of d/br and min of b/tr
        bl = bl.max(self.d);
        tr = tr.min(self.b);

        // we can draw now
        for x in (bl.x() as usize)..(tr.x() as usize) {
            for y in (bl.y() as usize)..(tr.y() as usize) {
                framebuffer.set_pixel(x + offset.x() as usize, y + offset.y() as usize, color)
            }
        }
    }
}

impl Shape for Rect {
    type ReturnType = Rect;
    
    fn draw(&self, framebuffer: &mut Framebuffer, color: u32) {
        let width = framebuffer.width();
        let height = framebuffer.height();

        // correct the min and max
    }

    fn inside_eh(&self, p: &Pos2D) -> bool {
        true
    }

    fn translate(&mut self, tx: f32, ty: f32) {
        
    }

    fn lerp_translate(&mut self, tx: f32, ty: f32, t: f32) {
        
    }

    fn lerp_steps_translate(&mut self, tx: f32, ty: f32, t: u32) {
        
    }

    fn transform(&mut self, points: Vec<Pos2D>) {
        
    }

    fn lerp_transform(&mut self, t: f32, points: Vec<Pos2D>) {
        
    }

    fn lerp_steps_transform(&mut self, t: f32, points: Vec<Pos2D>) {
        
    }
}