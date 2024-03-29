//use test_engine::graphics::window::Window;
use test_engine::graphics::window_minifb::Window;
fn main() {
    // let mut window = Window::new(1080, 720, "Hello Window");

    // window.init_gl();

    // while !window.should_close() {
    //     unsafe {
    //         gl::ClearColor(0.3, 0.5, 0.3, 1.0);
    //         gl::Clear(gl::COLOR_BUFFER_BIT)
    //     }
    //     window.update();
    // }
    let mut window = Window::new("graphics from scratch!", 512, 512);
    // eventually window will have a param of start_scene to dynamically pass what we initially render
    window.run(true);
}
