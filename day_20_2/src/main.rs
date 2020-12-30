use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
mod tile;
mod transformation;

use crate::tile::Tile;

const FILENAME: &str = "input.txt";

fn main() {
    let file = File::open(FILENAME)
        .map(BufReader::new)
        .expect("File I/O Error");
    let mut tiles_no_matches = tile::extract_tiles(file.lines().map(|s| s.unwrap()));

    let mut tile_map: HashMap<usize, Tile> = HashMap::new();

    // Find the other matching tile for each edge of each tile
    while let Some((id, mut tile)) = tiles_no_matches.pop() {
        for (other_id, other_tile) in tiles_no_matches.iter_mut() {
            // For each side
            for i in 0..8 {
                // For each side in other tile
                for j in 0..8 {
                    if tile.sides[i] == other_tile.sides[j] {
                        tile.matching_tiles[i] = Some(*other_id);
                        other_tile.matching_tiles[j] = Some(id);
                    }
                }
            }
        }
        tile_map.insert(id, tile);
    }

    let ans = tile_map
        .iter()
        .filter(|(_, tile)| {
            tile.matching_tiles
                .iter()
                .filter(|side| match side {
                    Some(_) => true,
                    None => false,
                })
                .count()
                == 4
        })
        .map(|(id, _)| id)
        .product::<usize>();

    // println!("{}", ans);
    // for (id, tile) in tile_map.iter() {
    //     println!("{}: ", id);
    //     for side in tile.sides.matching.iter() {
    //         print!(" ");
    //         for matched_side in side.iter() {
    //             print!(" {}", matched_side);
    //         }
    //         println!();
    //     }
    // }

    println!("{}", ans);
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &[&'static str] = &[
        "Tile 2311:",
        "..##.#..#.",
        "##..#.....",
        "#...##..#.",
        "####.#...#",
        "##.##.###.",
        "##...#.###",
        ".#.#.#..##",
        "..#....#..",
        "###...#.#.",
        "..###..###",
        "",
        "Tile 1951:",
        "#.##...##.",
        "#.####...#",
        ".....#..##",
        "#...######",
        ".##.#....#",
        ".###.#####",
        "###.##.##.",
        ".###....#.",
        "..#.#..#.#",
        "#...##.#..",
        "",
        "Tile 1171:",
        "####...##.",
        "#..##.#..#",
        "##.#..#.#.",
        ".###.####.",
        "..###.####",
        ".##....##.",
        ".#...####.",
        "#.##.####.",
        "####..#...",
        ".....##...",
        "",
        "Tile 1427:",
        "###.##.#..",
        ".#..#.##..",
        ".#.##.#..#",
        "#.#.#.##.#",
        "....#...##",
        "...##..##.",
        "...#.#####",
        ".#.####.#.",
        "..#..###.#",
        "..##.#..#.",
        "",
        "Tile 1489:",
        "##.#.#....",
        "..##...#..",
        ".##..##...",
        "..#...#...",
        "#####...#.",
        "#..#.#.#.#",
        "...#.#.#..",
        "##.#...##.",
        "..##.##.##",
        "###.##.#..",
        "",
        "Tile 2473:",
        "#....####.",
        "#..#.##...",
        "#.##..#...",
        "######.#.#",
        ".#...#.#.#",
        ".#########",
        ".###.#..#.",
        "########.#",
        "##...##.#.",
        "..###.#.#.",
        "",
        "Tile 2971:",
        "..#.#....#",
        "#...###...",
        "#.#.###...",
        "##.##..#..",
        ".#####..##",
        ".#..####.#",
        "#..#.#..#.",
        "..####.###",
        "..#.#.###.",
        "...#.#.#.#",
        "",
        "Tile 2729:",
        "...#.#.#.#",
        "####.#....",
        "..#.#.....",
        "....#..#.#",
        ".##..##.#.",
        ".#.####...",
        "####.#.#..",
        "##.####...",
        "##..#.##..",
        "#.##...##.",
        "",
        "Tile 3079:",
        "#.#.#####.",
        ".#..######",
        "..#.......",
        "######....",
        "####.#..#.",
        ".#...#.##.",
        "#.#####.##",
        "..#.###...",
        "..#.......",
        "..#.###...",
    ];

    #[test]
    fn test() {
        use super::*;

        let mut tiles_no_matches = tile::extract_tiles(INPUT);

        let mut tile_map: HashMap<usize, Tile> = HashMap::new();

        while let Some((id, mut tile)) = tiles_no_matches.pop() {
            for (other_id, other_tile) in tiles_no_matches.iter_mut() {
                // For each side
                for i in 0..8 {
                    // For each side in other tile
                    for j in 0..8 {
                        if tile.sides[i] == other_tile.sides[j] {
                            tile.matching_tiles[i] = Some(*other_id);
                            other_tile.matching_tiles[j] = Some(id);
                        }
                    }
                }
            }
            tile_map.insert(id, tile);
        }

        for (id, tile) in tile_map.iter() {
            print!("{}: ", id);
            for side in tile.matching_tiles.iter() {
                match side {
                    Some(matching_side) => println!("{}", matching_side),
                    None => println!(),
                }
            }
        }
        panic!();
    }
}
