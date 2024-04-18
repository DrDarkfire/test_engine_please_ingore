use util::linear::Pos2D;

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
}