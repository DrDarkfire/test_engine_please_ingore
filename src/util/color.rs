struct Color(u8, u8, u8, u8);

/// creates a single rbg from a separated rbg
pub fn from_u8_rgb(r: u8, g: u8, b: u8) -> u32 {
    let (r, g, b) = (r as u32, g as u32, b as u32);
    (r << 16 ) | (g << 8) | b
}

pub fn from_u8_rgba(r: u8, g: u8, b: u8, a: u8) -> u32 {
    let (r, g, b, a) = (r as u32, g as u32, b as u32, a as u32);
    (r << 24 ) | (g << 16) | (b << 8) | a
}