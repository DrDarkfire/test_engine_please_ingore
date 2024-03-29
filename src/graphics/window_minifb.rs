use crate::util::{color, linear::Pos2D};
use crate::components::shapes::Triangle;

/// # Window
/// based on https://github.com/GameDevGraphics/software-graphics and his videos
/// modifications based on our engine's needs
pub struct Window {
    window: minifb::Window,
    framebuffer: Framebuffer,
}

pub struct Framebuffer {
    data: Vec<u32>,
    width: usize,
    height: usize,
}

static POINTS: &[Pos2D] = &[
    Pos2D::new(0.3, 0.3),
    Pos2D::new(0.7, 0.3),
    Pos2D::new(0.5, 0.7),

    Pos2D::new(0.1, 0.3),
    Pos2D::new(0.5, 0.1),
    Pos2D::new(0.2, 0.6),

    Pos2D::new(0.5, 0.7),
    Pos2D::new(0.9, 0.7),
    Pos2D::new(0.5, 0.9),
];

impl Window {
    pub fn new(name: &str, width: usize, height: usize) -> Self {
        let options = minifb::WindowOptions {
            resize: true,
            ..Default::default()
        };

        let window = minifb::Window::new(
            name,
            width,
            height,
            options
        ).expect("Failed to crate window");

        Window {
            window,
            framebuffer: Framebuffer::new(width, height)
        }
    }

    pub fn should_close(&self) -> bool {
        !self.window.is_open()
    }

    pub fn display(&mut self) {
        self.window.update_with_buffer(
            &self.framebuffer.data,
            self.framebuffer.width,
            self.framebuffer.height
        ).expect("Failed to display pixels.");

        // if the window resizes, we update the framebuffer's size
        let (width, height) = self.window.get_size();
        if width != self.framebuffer.width() || height != self.framebuffer.height() {
            self.framebuffer = Framebuffer::new(width, height);
        }
    }

    pub fn framebuffer(&mut self) -> &mut Framebuffer{
        &mut self.framebuffer
    }

    /// Test function with the later purpose of letting devs subscribe game actions to keybinds.
    fn key_press(&self, keys: Vec<minifb::Key>) {
        keys.iter().for_each(|key| 
            println!("{:?} was pressed", key)
        );
    }

    fn draw(&mut self) {
        let fb = self.framebuffer();

        // place down our background color first in each frame eventually should be specified to hold a color or image.
        fb.clear(color::from_u8_rgb(20, 20, 20));

        for i in 0..(POINTS.len() / 3) {
            let t = Triangle::new(
                POINTS[i * 3],
                POINTS[i * 3 + 1],
                POINTS[i * 3 + 2]
            ).draw_triangle(fb, color::from_u8_rgb((i * 100 + 100) as u8, 100, 50));
        }

        // DEPRECATED FROM ORIGINAL BG COLOR DRAWING
        // for x in 0..fb.width() {
        //     for y in 0..fb.height() {
        //         fb.set_pixel(x, y, color::from_u8_rgb(20, 20, 20)); // later we need to figure out opacity
        //     }
        // }
    }

    /// When the game is set up and ready to start, run() is to be called. This is currently the game loop.
    pub fn run(&mut self, debug: bool) {
        while !self.should_close() && !self.window.is_key_down(minifb::Key::Escape) {
            self.draw();
            self.display();
            if debug {
                self.key_press(self.window.get_keys_pressed(minifb::KeyRepeat::No));
            }
        }
    }
}

impl Framebuffer {
    pub fn new(width: usize, height: usize) -> Self {
        Framebuffer {
            data: vec![0; width * height],
            width,
            height
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, value: u32) {
        self.data[x + y * self.width] = value
    }

    pub fn clear(&mut self, value: u32) {
        for i in 0..self.data.len() {
            self.data[i] = value;
        }
    }
}