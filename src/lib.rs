use crate::util::linear::*;

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