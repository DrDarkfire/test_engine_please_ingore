use util::linear::Pos2D;
use crate::util::linear::Vec2D;

pub mod graphics;
pub mod util;
pub mod components;

#[test]
fn test_pos2d() {
    let mut pos1 = Pos2D::new(2.0, 4.0);
    let mut pos2 = Pos2D::new(5.0, 2.0);
    println!("{}", pos1);
    println!("{}", pos2);

    // translation
    pos1.translate_x(3.0);
    pos2.translate_y(-1.0);
    assert_eq!(pos1, Pos2D::new(5.0, 4.0));
    assert_eq!(pos2, Pos2D::new(5.0, 1.0));

    let v = Vec2D::new(1.0, 3.0);
    pos1 = pos1 + v;
    pos2 += v;
    assert_eq!(pos1, Pos2D::new(6.0, 7.0));
    assert_eq!(pos2, Pos2D::new(6.0, 4.0));

    let pos3 = Pos2D::ZERO;
    let pos4 = Pos2D::new(100.0, 100.0);
    assert_eq!(Pos2D::lerp(pos3, pos4, 0.5), Pos2D::new(50.0, 50.0));

    let lerp_vec = Pos2D::lerp_steps(pos3, pos4, 20);
    //println!("{}", lerp_vec);
    // debug bool var
    let print_vec = true;
    if print_vec {
        for i in 0..lerp_vec.len() {
            println!("Step {}: {}", i, lerp_vec.get(i).unwrap());
        }
    }
}