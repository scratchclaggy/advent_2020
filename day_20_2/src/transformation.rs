use crate::tile::{Tile, TilePrimitive, TILE_HEIGHT, TILE_WIDTH};

#[derive(Debug, Clone, Copy)]
pub enum Transform {
    None,
    FlipH,
    FlipV,
    FlipBoth,
}

/*
    0: North
    1: North inv
    2: East
    3: East inv
    4: South
    5: South inv
    6: West
    7: West inv
*/
pub fn flip_horizontal(tile_primitive: &TilePrimitive) -> TilePrimitive {
    let mut out = [false; TILE_HEIGHT * TILE_WIDTH];
    tile_primitive
        .chunks_exact(TILE_WIDTH)
        .zip(out.chunks_exact_mut(TILE_WIDTH).rev())
        .for_each(|(t, o)| o.copy_from_slice(t));
    out
}

fn flip_sides_horizontal(old: &Tile) -> Tile {
    Tile {
        sides: [
            old.sides[4], // South -> North
            old.sides[5], // South inv -> North inv
            old.sides[3], // East inv -> East
            old.sides[2], // East -> East inv
            old.sides[0], // North -> South
            old.sides[1], // North inv -> South inv
            old.sides[7], // West inv -> West
            old.sides[6], // West -> West inv
        ],
        insides: old.insides,
        matching_tiles: [
            old.matching_tiles[4], // South -> North
            old.matching_tiles[5], // South inv -> North inv
            old.matching_tiles[3], // East inv -> East
            old.matching_tiles[2], // East -> East inv
            old.matching_tiles[0], // North -> South
            old.matching_tiles[1], // North inv -> South inv
            old.matching_tiles[7], // West inv -> West
            old.matching_tiles[6], // West -> West inv
        ],
    }
}

pub fn flip_vertical(tile_primitive: &TilePrimitive) -> TilePrimitive {
    let mut out = [false; TILE_HEIGHT * TILE_WIDTH];
    tile_primitive
        .chunks_exact(TILE_WIDTH)
        .zip(out.chunks_exact_mut(TILE_WIDTH))
        .for_each(|(t, o)| t.iter().zip(o.iter_mut().rev()).for_each(|(t, o)| *o = *t));
    out
}

fn flip_sides_vertical(old: &Tile) -> Tile {
    Tile {
        sides: [
            old.sides[1], // North -> North inv
            old.sides[0], // North inv -> North
            old.sides[6], // West -> East
            old.sides[7], // West inv -> East inv
            old.sides[5], // South -> South inv
            old.sides[4], // South inv -> South
            old.sides[2], // East -> West
            old.sides[3], // East inv -> West inv
        ],
        insides: old.insides,
        matching_tiles: [
            old.matching_tiles[1], // North -> North inv
            old.matching_tiles[0], // North inv -> North
            old.matching_tiles[6], // West -> East
            old.matching_tiles[7], // West inv -> East inv
            old.matching_tiles[5], // South -> South inv
            old.matching_tiles[4], // South inv -> South
            old.matching_tiles[2], // East -> West
            old.matching_tiles[3], // East inv -> West inv
        ],
    }
}

fn rotate_internals(tile: &TilePrimitive) -> TilePrimitive {
    let mut out = [false; TILE_HEIGHT * TILE_WIDTH];
    out.chunks_exact_mut(TILE_WIDTH)
        .enumerate()
        .for_each(|(x, row)| {
            row.iter_mut()
                .rev()
                .enumerate()
                .for_each(|(y, out)| *out = tile[(y * TILE_WIDTH) + x])
        });
    out
}

fn rotate_sides(old: Tile) -> Tile {
    Tile {
        sides: [
            old.sides[7], // West inv -> North
            old.sides[6], // West -> North inv
            old.sides[0], // North -> East
            old.sides[1], // North inv -> East inv
            old.sides[3], // East inv -> South
            old.sides[2], // East -> South inv
            old.sides[4], // North -> East
            old.sides[5], // North inv -> East inv
        ],
        insides: old.insides,
        matching_tiles: [
            old.matching_tiles[7], // West inv -> North
            old.matching_tiles[6], // West -> North inv
            old.matching_tiles[0], // North -> East
            old.matching_tiles[1], // North inv -> East inv
            old.matching_tiles[3], // East inv -> South
            old.matching_tiles[2], // East -> South inv
            old.matching_tiles[4], // South -> East
            old.matching_tiles[5], // South inv -> East inv
        ],
    }
}

pub fn rotate_tile(tile: &TilePrimitive, steps: usize) -> TilePrimitive {
    let mut out = tile.clone();
    for _ in 0..steps {
        out = rotate_internals(&out);
    }
    out
}
