use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
mod tile;
mod transformation;

use crate::tile::{Tile, TILE_HEIGHT, TILE_WIDTH};

const FILENAME: &str = "input.txt";

fn main() {
    let file = File::open(FILENAME)
        .map(BufReader::new)
        .expect("File I/O Error");
    let mut tiles = tile::extract_tiles(file.lines().map(|s| s.unwrap()));

    let mut tile_map: HashMap<usize, Tile> = HashMap::new();

    while let Some((id, mut tile)) = tiles.pop() {
        for (other_id, other_tile) in tiles.iter_mut() {
            // For each side
            for i in 0..8 {
                // For each side in other tile
                for j in 0..8 {
                    if tile.sides.sides[i] == other_tile.sides.sides[j] {
                        tile.sides.matching[i].insert(*other_id);
                        other_tile.sides.matching[j].insert(id);
                    }
                }
            }
        }
        tile_map.insert(id, tile);
    }

    let ans = tile_map
        .iter()
        .filter(|(id, tile)| {
            tile.sides
                .matching
                .iter()
                .filter(|side| side.is_empty())
                .count()
                == 4
        })
        .map(|(id, _)| id)
        .product::<usize>();

    println!("{}", ans);
    for (id, tile) in tile_map.iter() {
        println!("{}: ", id);
        for side in tile.sides.matching.iter() {
            print!(" ");
            for matched_side in side.iter() {
                print!(" {}", matched_side);
            }
            println!();
            assert!(side.len() == 0 || side.len() == 1);
        }
    }
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

        let mut tiles = tile::extract_tiles(INPUT);

        let mut tile_map: HashMap<usize, Tile> = HashMap::new();

        while let Some((id, mut tile)) = tiles.pop() {
            for (other_id, other_tile) in tiles.iter_mut() {
                // For each side
                for i in 0..8 {
                    // For each side in other tile
                    for j in 0..8 {
                        if tile.sides.sides[i] == other_tile.sides.sides[j] {
                            tile.sides.matching[i].insert(*other_id);
                            other_tile.sides.matching[j].insert(id);
                        }
                    }
                }
            }
            tile_map.insert(id, tile);
        }

        for (id, tile) in tile_map.iter() {
            println!("{}: ", id);
            for side in tile.sides.matching.iter() {
                print!(" ");
                for matched_side in side.iter() {
                    print!(" {}", matched_side);
                }
                println!();
            }
        }
        panic!();
    }
}
