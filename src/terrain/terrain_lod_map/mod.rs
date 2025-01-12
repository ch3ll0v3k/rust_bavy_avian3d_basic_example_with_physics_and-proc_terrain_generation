const Z: i16 = 2; // 2;
const A: i16 = 2; // 2;
const B: i16 = 0; // 4;
const C: i16 = 0; // 8;
const D: i16 = 0; // 16;
const E: i16 = 0; // 32;
const F: i16 = 0; // 64;
const G: i16 = 0; // 128;

pub const BASE_LOD_SCALE: i16 = Z;
pub const TERRAIN_LOD_MAP_SIZE: usize = 13 + 2;

// prettier-ignore
static T2: [[i16; TERRAIN_LOD_MAP_SIZE]; TERRAIN_LOD_MAP_SIZE] = [
  [G, G, G, G, G, G, G, G, G, G, G, G, G, G, G],
  [G, F, F, F, F, F, F, F, F, F, F, F, F, F, G],
  [G, F, E, E, E, E, E, E, E, E, E, E, E, F, G],
  [G, F, E, D, D, D, D, D, D, D, D, D, E, F, G],
  [G, F, E, D, C, C, C, C, C, C, C, D, E, F, G],
  [G, F, E, D, C, B, B, B, B, B, C, D, E, F, G],
  [G, F, E, D, C, B, A, A, A, B, C, D, E, F, G],
  [G, F, E, D, C, B, A, Z, A, B, C, D, E, F, G],
  [G, F, E, D, C, B, A, A, A, B, C, D, E, F, G],
  [G, F, E, D, C, B, B, B, B, B, C, D, E, F, G],
  [G, F, E, D, C, C, C, C, C, C, C, D, E, F, G],
  [G, F, E, D, D, D, D, D, D, D, D, D, E, F, G],
  [G, F, E, E, E, E, E, E, E, E, E, E, E, F, G],
  [G, F, F, F, F, F, F, F, F, F, F, F, F, F, G],
  [G, G, G, G, G, G, G, G, G, G, G, G, G, G, G],
];

pub fn get_lod() -> [[i16; TERRAIN_LOD_MAP_SIZE]; TERRAIN_LOD_MAP_SIZE] {
  T2
}
