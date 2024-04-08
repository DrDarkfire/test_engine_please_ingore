use crate::util::color::Color;

#[allow(unused)]
pub struct Material {
    color: Color,
    texture: String,
    normal_map: String,
    reflectivity: f32,
    transparency: f32,
    specularity: f32,
    emmission: f32,
}