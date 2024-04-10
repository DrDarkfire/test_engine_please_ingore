use crate::{graphics::window_minifb::Framebuffer, util::linear::Pos2D};

/// The shape mod will provide basic shapes for the game engine.
/// 
/// Eventually there will be a shape trait with functions required of every shape (i.e. draw)
/// This is to separate functions specific to the shape

pub struct Triangle {
    a: Pos2D,
    b: Pos2D,
    c: Pos2D,
}

impl Triangle {
    pub fn new(a: Pos2D, b: Pos2D, c: Pos2D) -> Triangle {
        Triangle{ a, b, c }
    }

    /// Draw the triangle on the screen on top of all previous pixels.
    /// 
    /// Later we will have an alpha channel to deal with transparency.
    pub fn draw(&self, framebuffer: &mut Framebuffer, color: u32) {
        // create a bounding box over the triangle so we don't perform checks on every coord
        let width = framebuffer.width();
        let height = framebuffer.height();
        // find the smallest value and if < 0 change it. find largest value and if > 1 change it. Then we convert them to pixels
        let min = self.a.min(self.b.min(self.c)).max(Pos2D::ZERO) * Pos2D::new(width as f32, height as f32);
        let max = self.a.max(self.b.max(self.c)).min(Pos2D::ONE) * Pos2D::new(width as f32, height as f32);

        for x in (min.x() as usize)..(max.x() as usize) {
            for y in (min.y() as usize)..(max.y() as usize) {
                // create a point for each pixel we are attempting to modify within 0 - 1
                let p = Pos2D::new(x as f32 / width as f32, y as f32 / height as f32);

                if self.inside_triangle(&p) {
                    framebuffer.set_pixel(x, y, color);
                }
            }
        }
    }

    /// if edge_function returns positive we are inside the triangle, if not we are outside
    pub fn edge_function(&self, a: &Pos2D, c: &Pos2D, b: &Pos2D) -> f32 {
        // we force passing in references to minimize copying things we don't need to. 
        (c.x() - a.x()) * (b.y()- a.y()) - (c.y() - a.y()) * (b.x() - a.x())
    }

    // works with CLOCKWISE winding order
    pub fn inside_triangle(&self, p: &Pos2D) -> bool {
        let a0 = self.edge_function(&self.b, &self.c, p);
        let a1 = self.edge_function(&self.c, &self.a, p);
        let a2 = self.edge_function(&self.a, &self.b, p);
        
        let mut overlaps = true;
        overlaps &= a0 > 0.0;
        overlaps &= a1 > 0.0;
        overlaps &= a2 > 0.0;

        overlaps
    }
}