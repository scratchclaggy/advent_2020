use std::default::Default;

pub const TILE_WIDTH: usize = 10;
pub const TILE_HEIGHT: usize = 10;

pub type TilePrimitive = [bool; TILE_WIDTH * TILE_HEIGHT];

pub struct Tile {
    pub sides: [u16; 8],
    pub insides: [bool; (TILE_WIDTH - 2) * (TILE_HEIGHT - 2)],
    pub matching_tiles: [Option<usize>; 8],
}

impl Tile {
    pub fn new(input: &TilePrimitive) -> Tile {
        // Sides
        let mut north = 0u16;
        for i in 0..TILE_WIDTH {
            north <<= 1;
            if input[i] {
                north |= 1;
            }
        }
        let north_inverse = north.reverse_bits() >> 6;

        let mut east = 0u16;
        let mut west = 0u16;
        for row in input.chunks_exact(10) {
            east <<= 1;
            west <<= 1;
            west |= row[0] as u16;
            east |= row[TILE_WIDTH - 1] as u16;
        }
        let east_inverse = east.reverse_bits() >> 6;
        let west_inverse = west.reverse_bits() >> 6;

        let bottom_row = &input[(TILE_HEIGHT - 1) * TILE_WIDTH..];
        let south = bottom_row
            .iter()
            .fold(0u16, |acc, &arg| (acc << 1) | arg as u16);
        let south_inverse = south.reverse_bits() >> 6;

        // Insides
        let mut insides = [false; (TILE_WIDTH - 2) * (TILE_HEIGHT - 2)];
        for (src, dst) in input
            .chunks_exact(TILE_WIDTH)
            .skip(1)
            .zip(insides.chunks_exact_mut(TILE_WIDTH - 2))
        {
            dst.copy_from_slice(&src[1..TILE_WIDTH - 1]);
        }

        Tile {
            sides: [
                north,
                north_inverse,
                east,
                east_inverse,
                south,
                south_inverse,
                west,
                west_inverse,
            ],
            insides,
            matching_tiles: Default::default(),
        }
    }
}
pub fn extract_tiles(lines: impl IntoIterator<Item = impl AsRef<str>>) -> Vec<(usize, Tile)> {
    let mut tiles = vec![];
    let mut lines = lines.into_iter();

    loop {
        let line = lines.next().unwrap();
        let tile_info = line.as_ref();
        let tile_num = tile_info
            .strip_prefix("Tile ")
            .and_then(|s| s.strip_suffix(":"))
            .unwrap();
        let tile_num = tile_num.parse().unwrap();

        let mut tile = [false; TILE_WIDTH * TILE_HEIGHT];
        tile.chunks_exact_mut(TILE_WIDTH).for_each(|data| {
            let row_data = lines.next().unwrap();
            data.iter_mut()
                .zip(row_data.as_ref().chars())
                .for_each(|(d, c)| {
                    *d = match c {
                        '.' => false,
                        '#' => true,
                        _ => panic!("bad character {}", c),
                    }
                });
        });
        let tile = Tile::new(&tile);

        tiles.push((tile_num, tile));
        if lines.next().is_none() {
            break;
        }
    }

    tiles
}

// fn print_tile(tile: &TilePrimitive) {
//     for row in tile.chunks_exact(TILE_WIDTH) {
//         for &bit in row {
//             if bit {
//                 print!("#");
//             } else {
//                 print!(".");
//             }
//         }
//         println!("");
//     }
// }

// impl Tile {
//     pub fn north(&self) -> u16 {
//         self.sides[0]
//     }
//     pub fn north_inverse(&self) -> u16 {
//         self.sides[1]
//     }
//     pub fn east(&self) -> u16 {
//         self.sides[2]
//     }
//     pub fn east_inverse(&self) -> u16 {
//         self.sides[3]
//     }
//     pub fn south(&self) -> u16 {
//         self.sides[4]
//     }
//     pub fn south_inverse(&self) -> u16 {
//         self.sides[5]
//     }
//     pub fn west(&self) -> u16 {
//         self.sides[6]
//     }
//     pub fn west_inverse(&self) -> u16 {
//         self.sides[7]
//     }
// }
