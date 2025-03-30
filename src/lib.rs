use std::time::SystemTime;

use components::shapes::{Rect, Triangle};

use crate::{components::nodes::node::Camera2D, util::linear::*};

pub mod graphics;
pub mod util;
pub mod components;

#[test]
fn test_pos2d() {
    // make new Pos3Ds and test fmt::Display
    let mut pos1 = Pos2D::new(2.0, 4.0);
    let mut pos2 = Pos2D::new(5.0, 2.0);
    println!("{}", pos1);
    println!("{}", pos2);

    // translation
    pos1.translate_x(3.0);
    pos2.translate_y(-1.0);
    assert_eq!(pos1, Pos2D::new(5.0, 4.0));
    assert_eq!(pos2, Pos2D::new(5.0, 1.0));

    // translation with vector additons
    let v = Vec2D::new(1.0, 3.0);
    pos1 = pos1 + v;
    pos2 += v;
    assert_eq!(pos1, Pos2D::new(6.0, 7.0));
    assert_eq!(pos2, Pos2D::new(6.0, 4.0));

    // lerp tests
    let pos3 = Pos2D::ZERO;
    let pos4 = Pos2D::new(100.0, 100.0);
    assert_eq!(Pos2D::lerp(pos3, pos4, 0.5), Pos2D::new(50.0, 50.0));

    let lerp_vec = Pos2D::lerp_steps(pos3, pos4, 20);
    //println!("{}", lerp_vec);
    // debug bool var
    let print_vec = false;
    if print_vec {
        for i in 0..lerp_vec.len() {
            println!("Step {}: {}", i, lerp_vec.get(i).unwrap());
        }
    }

    // min/max tests
    assert_eq!(pos3.min(pos4), pos3);
    assert_eq!(pos3.max(pos4), pos4);

    // set tests
    pos1.set_x(0.0);
    pos1.set_y(0.0);
    pos2.set(0.0,0.0);
    assert_eq!(pos1, Pos2D::ZERO);
    assert_eq!(pos2, Pos2D::ZERO);
}

#[test]
fn test_pos3d() {
    // make new Pos3Ds and test fmt::Display
    let mut pos1 = Pos3D::new(2.0, 4.0, 2.0);
    let mut pos2 = Pos3D::new(5.0, 2.0, 4.0);
    println!("{}", pos1);
    println!("{}", pos2);

    // translation
    pos1.translate_x(3.0);
    pos2.translate_y(-1.0);
    pos2.translate_z(10.0);
    assert_eq!(pos1, Pos3D::new(5.0, 4.0, 2.0));
    assert_eq!(pos2, Pos3D::new(5.0, 1.0, 14.0));

    // translation with vector additons
    let v = Vec3D::new(1.0, 3.0, -3.0);
    pos1 = pos1 + v;
    pos2 += v;
    assert_eq!(pos1, Pos3D::new(6.0, 7.0, -1.0));
    assert_eq!(pos2, Pos3D::new(6.0, 4.0, 11.0));

    // lerp tests
    let pos3 = Pos3D::ZERO;
    let pos4 = Pos3D::new(100.0, 100.0, 100.0);
    assert_eq!(Pos3D::lerp(pos3, pos4, 0.5), Pos3D::new(50.0, 50.0, 50.0));

    let lerp_vec = Pos3D::lerp_steps(pos3, pos4, 20);
    //println!("{}", lerp_vec);
    // debug bool var
    let print_vec = true;
    if print_vec {
        for i in 0..lerp_vec.len() {
            println!("Step {}: {}", i, lerp_vec.get(i).unwrap());
        }
    }

    // min/max tests
    assert_eq!(pos3.min(pos4), pos3);
    assert_eq!(pos3.max(pos4), pos4);

    // set tests
    pos1.set_x(0.0);
    pos1.set_y(0.0);
    pos1.set_z(0.0);
    pos2.set(0.0,0.0, 0.0);
    assert_eq!(pos1, Pos3D::ZERO);
    assert_eq!(pos2, Pos3D::ZERO);
}

#[test]
fn test_vec2d() {
    // creation and fmt::Display tests
    let mut v1 = Vec2D::new(1.0, 2.0);
    let mut v2 = Vec2D::new(3.0, -1.0);
    println!("{}", v1);
    println!("{}", v2);

    // vector addition
    println!("{}", v1 + v2);
    assert_eq!(v1 + v2, Vec2D::new(4.0, 1.0));

    // scalar multiplication
    let v3 = Vec2D::new(2.0, 4.0);
    assert_eq!(v1 * 2.0, v3);

    // setters
    v1.set_dx(3.0);
    v2.set_dy(2.0);
    assert_eq!(v1, v2);
    v1.set(2.0, 4.0);
    assert_eq!(v1, v3);
}

#[test]
fn test_triangle() {
    let tp1: Pos2D = Pos2D::new(-100.0, -140.0);//(-0.5, -0.5)
    let tp2: Pos2D = Pos2D::new(-180.0, -140.0);//(0, 0.5)
    let tp3: Pos2D = Pos2D::new(-100.0, -180.0);//(0.5, 0.5)

    let tp4: Pos2D = Pos2D::new(0.5, 0.7);
    let tp5: Pos2D = Pos2D::new(0.9, 0.7);
    let tp6: Pos2D = Pos2D::new(0.5, 0.9);

    let tp7: Pos2D = Pos2D::new(100.0, 140.0);
    let tp8: Pos2D = Pos2D::new(180.0, 140.0);
    let tp9: Pos2D = Pos2D::new(100.0, 180.0);

    let t1 = Triangle::new(tp1, tp2, tp3);
    let t2 = Triangle::new(tp4, tp5, tp6);
    let t3 = Triangle::new(tp7, tp8, tp9);
    let p1 = Pos2D::ZERO;
    let p2 = Pos2D::new(0.6, 0.75);
    let p3 = Pos2D::new(-120.0, -141.0);
    let p4 = Pos2D::new(120.0, 141.0);
    assert_eq!(t1.inside_eh(&p1), false);
    assert_eq!(t1.inside_eh(&p3), true);
    assert_eq!(t2.inside_eh(&p1), false);
    assert_eq!(t2.inside_eh(&p2), true);

    // test integrity of draw_abs
    let cam = Camera2D::new(Pos2D::new(0.0, 0.0));

    let width: f32 = 512.0;
    let height: f32 = 512.0;
    
    // check if we are allowed to draw
    assert_eq!(t3.render_guard(&cam, width, height), true);

    // check our view bounds
    let mut bl = Pos2D::new_from_other(cam.pos());
    bl.translate(-(width / 2.0), -(height / 2.0));
    println!("bl: {}", bl);
    assert_eq!(Pos2D::new(-256.0, -256.0), bl);
    let mut tr = Pos2D::new_from_other(cam.pos());
    tr.translate(width / 2.0, height / 2.0);
    println!("tr: {}", tr);
    assert_eq!(Pos2D::new(256.0, 256.0), tr);

    // offset checks
    let offset = Pos2D::new(-bl.x(), -bl.y());
    println!("our offset is {}", offset);
    println!("bl with offset is {}, {}", bl.x() + offset.x(), bl.y() + offset.y());
    // get the bounding box and and adjust to not care about anything outside of the viewport
    let mut bounds = t3.bounds();

    bounds.0 = bounds.0.max(bl);
    println!("bounds.0 {}", bounds.0);

    bounds.1 = bounds.1.min(tr);
    println!("bounds.1 {}", bounds.1);
    
    println!("drawing at: {}", bounds.0.x() + offset.x());

}

#[test]
fn test_rect() {
    let r1 = Rect::new(Pos2D::new(100.0, 100.0), 100.0, 200.0);
    println!("a: {},\nb: {},\nc: {},\nd: {}", r1.a(), r1.b(), r1.c(), r1.d());
}

#[test]
fn test_logger() {
    
}