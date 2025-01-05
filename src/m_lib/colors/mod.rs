// prettier-ignore
use bevy::color::{
  Color, ColorToComponents,
  palettes::{css::*, tailwind::*},
};

pub fn hex_to_rgb(hex: &str) -> Color {
  let (r, g, b, a) = _hex_to_rgba(hex);
  Color::srgb_u8(r, g, b)
}

pub fn hex_to_rgba(hex: &str) -> Color {
  let (r, g, b, a) = _hex_to_rgba(hex);
  Color::srgba_u8(r, g, b, a)
}

pub fn hex_to_rgba_f32(hex: &str) -> [f32; 4] {
  let (r, g, b, a) = _hex_to_rgba(hex);
  let color = Color::srgba_u8(r, g, b, a);
  color.to_linear().to_f32_array()
}

pub fn hex_to_rgb_f32(hex: &str) -> [f32; 3] {
  let (r, g, b, a) = _hex_to_rgba(hex);
  let color = Color::srgb_u8(r, g, b);
  color.to_linear().to_f32_array_no_alpha()
}

fn _hex_to_rgba(hex: &str) -> (u8, u8, u8, u8) {
  let hex = hex.trim_start_matches('#');
  // if hex.len() != 6 {
  //   return Err("Hex color must be 6 characters long.".into());
  // }

  let mut r: u8 = 0;
  let mut g: u8 = 0;
  let mut b: u8 = 0;
  let mut a: u8 = 0;

  r = u8::from_str_radix(&hex[0..2], 16).unwrap(); // .map_err(|_| "Invalid red component")?;
  g = u8::from_str_radix(&hex[2..4], 16).unwrap(); // .map_err(|_| "Invalid green component")?;
  b = u8::from_str_radix(&hex[4..6], 16).unwrap(); // .map_err(|_| "Invalid blue component")?;

  if hex.len() == 8 {
    a = u8::from_str_radix(&hex[6..8], 16).unwrap(); // .map_err(|_| "Invalid blue component")?;
  }

  (r, g, b, a)
}
