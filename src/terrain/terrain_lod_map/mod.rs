// prettier-ignore
// static T2: [[i8; 13]; 13] = [
//   [ 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0 ],
//   [ 0,16,16,16,16,16,16,16,16,16,16,16, 0 ],
//   [ 0,16, 8, 8, 8, 8, 8, 8, 8, 8, 8,16, 0 ],
//   [ 0,16, 8, 4, 4, 4, 4, 4, 4, 4, 8,16, 0 ],
//   [ 0,16, 8, 4, 2, 2, 2, 2, 2, 4, 8,16, 0 ],
//   [ 0,16, 8, 4, 2, 1, 1, 1, 2, 4, 8,16, 0 ],
//   [ 0,16, 8, 4, 2, 1, 1, 1, 2, 4, 8,16, 0 ],
//   [ 0,16, 8, 4, 2, 1, 1, 1, 2, 4, 8,16, 0 ],
//   [ 0,16, 8, 4, 2, 2, 2, 2, 2, 4, 8,16, 0 ],
//   [ 0,16, 8, 4, 4, 4, 4, 4, 4, 4, 8,16, 0 ],
//   [ 0,16, 8, 8, 8, 8, 8, 8, 8, 8, 8,16, 0 ],
//   [ 0,16,16,16,16,16,16,16,16,16,16,16, 0 ],
//   [ 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0 ],
// ];

// prettier-ignore
// static T2: [[i8; 13]; 13] = [
//   [0,0,0,0,0,0,0,0,0,0,0,0,0],
//   [0,0,0,0,0,0,0,0,0,0,0,0,0],
//   [0,0,8,8,8,8,8,8,8,8,8,0,0],
//   [0,0,8,4,4,4,4,4,4,4,8,0,0],
//   [0,0,8,4,2,2,2,2,2,4,8,0,0],
//   [0,0,8,4,2,1,1,1,2,4,8,0,0],
//   [0,0,8,4,2,1,1,1,2,4,8,0,0],
//   [0,0,8,4,2,1,1,1,2,4,8,0,0],
//   [0,0,8,4,2,2,2,2,2,4,8,0,0],
//   [0,0,8,4,4,4,4,4,4,4,8,0,0],
//   [0,0,8,8,8,8,8,8,8,8,8,0,0],
//   [0,0,0,0,0,0,0,0,0,0,0,0,0],
//   [0,0,0,0,0,0,0,0,0,0,0,0,0],
// ];

// prettier-ignore
// static T2: [[i8; 13]; 13] = [
//   [32,32,32,32,32,32,32,32,32,32,32,32,32],
//   [32,32,32,32,32,32,32,32,32,32,32,32,32],
//   [32,32,16,16,16,16,16,16,16,16,16,32,32],
//   [32,32,16, 8, 8, 8, 8, 8, 8, 8,16,32,32],
//   [32,32,16, 8, 4, 4, 4, 4, 4, 8,16,32,32],
//   [32,32,16, 8, 4, 2, 2, 2, 4, 8,16,32,32],
//   [32,32,16, 8, 4, 2, 1, 2, 4, 8,16,32,32],
//   [32,32,16, 8, 4, 2, 2, 2, 4, 8,16,32,32],
//   [32,32,16, 8, 4, 4, 4, 4, 4, 8,16,32,32],
//   [32,32,16, 8, 8, 8, 8, 8, 8, 8,16,32,32],
//   [32,32,16,16,16,16,16,16,16,16,16,32,32],
//   [32,32,32,32,32,32,32,32,32,32,32,32,32],
//   [32,32,32,32,32,32,32,32,32,32,32,32,32],
// ];

// // prettier-ignore
// static T2: [[i8; 13]; 13] = [
//   [64,64,64,64,64,64,64,64,64,64,64,64,64],
//   [64,64,64,64,64,64,64,64,64,64,64,64,64],
//   [64,64,32,32,32,32,64,32,32,32,32,64,64],
//   [64,64,32,32,16,16,32,16,16,32,32,64,64],
//   [64,64,32,16, 8, 4, 8, 4, 8,16,32,64,64],
//   [64,64,32,16, 4, 1, 1, 1, 4,16,32,64,64],
//   [64,64,64,32, 8, 1, 1, 1, 8,32,64,64,64],
//   [64,64,32,16, 4, 1, 1, 1, 4,16,32,64,64],
//   [64,64,32,16, 8, 4, 8, 4, 8,16,32,64,64],
//   [64,64,32,32,16,16,32,16,16,32,32,64,64],
//   [64,64,32,32,32,32,64,32,32,32,32,64,64],
//   [64,64,64,64,64,64,64,64,64,64,64,64,64],
//   [64,64,64,64,64,64,64,64,64,64,64,64,64],
// ];

//  prettier-ignore
static T2: [[i16; 13]; 13] = [
  [-4,-4,-4,-4,-4,-4,-4,-4,-4,-4,-4,-4,-4],
  [-4,-2,-2,-2,-2,-2,-2,-2,-2,-2,-2,-2,-4],
  [-4,-2,64,64,64,64,64,64,64,64,64,-2,-4],
  [-4,-2,64,32,32,32,32,32,32,32,64,-2,-4],
  [-4,-2,64,32,16,16,16,16,16,32,64,-2,-4],
  [-4,-2,64,32,16, 4, 4, 4,16,32,64,-2,-4],
  [-4,-2,64,32,16, 4, 1, 4,16,32,64,-2,-4],
  [-4,-2,64,32,16, 4, 4, 4,16,32,64,-2,-4],
  [-4,-2,64,32,16,16,16,16,16,32,64,-2,-4],
  [-4,-2,64,32,32,32,32,32,32,32,64,-2,-4],
  [-4,-2,64,64,64,64,64,64,64,64,64,-2,-4],
  [-4,-2,-2,-2,-2,-2,-2,-2,-2,-2,-2,-2,-4],
  [-4,-4,-4,-4,-4,-4,-4,-4,-4,-4,-4,-4,-4],
];

pub fn get_lod() -> [[i16; 13]; 13] {
  T2
}
