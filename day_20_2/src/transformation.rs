use crate::tile::{TilePrimitive, TileSides, TILE_HEIGHT, TILE_WIDTH};

#[derive(Debug, Clone, Copy)]
pub enum Transform {
    None,
    Flip_H,
    Flip_V,
    Flip_Both,
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

pub fn flip_horizontal(TilePrimitive: &TilePrimitive) -> TilePrimitive {
    let mut out = [false; TILE_HEIGHT * TILE_WIDTH];
    TilePrimitive
        .chunks_exact(TILE_WIDTH)
        .zip(out.chunks_exact_mut(TILE_WIDTH).rev())
        .for_each(|(t, o)| o.copy_from_slice(t));
    out
}

fn flip_sides_horizontal(old: &TileSides) -> TileSides {
    TileSides {
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
        matching: [
            old.matching[4].clone(), // South -> North
            old.matching[5].clone(), // South inv -> North inv
            old.matching[3].clone(), // East inv -> East
            old.matching[2].clone(), // East -> East inv
            old.matching[0].clone(), // North -> South
            old.matching[1].clone(), // North inv -> South inv
            old.matching[7].clone(), // West inv -> West
            old.matching[6].clone(), // West -> West inv
        ],
    }
}

pub fn flip_vertical(TilePrimitive: &TilePrimitive) -> TilePrimitive {
    let mut out = [false; TILE_HEIGHT * TILE_WIDTH];
    TilePrimitive
        .chunks_exact(TILE_WIDTH)
        .zip(out.chunks_exact_mut(TILE_WIDTH))
        .for_each(|(t, o)| t.iter().zip(o.iter_mut().rev()).for_each(|(t, o)| *o = *t));
    out
}

fn flip_sides_vertical(old: &TileSides) -> TileSides {
    TileSides {
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
        matching: [
            old.matching[1].clone(), // North -> North inv
            old.matching[0].clone(), // North inv -> North
            old.matching[6].clone(), // West -> East
            old.matching[7].clone(), // West inv -> East inv
            old.matching[5].clone(), // South -> South inv
            old.matching[4].clone(), // South inv -> South
            old.matching[2].clone(), // East -> West
            old.matching[3].clone(), // East inv -> West inv
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

fn rotate_sides(old: TileSides) -> TileSides {
    TileSides {
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
        matching: [
            old.matching[7].clone(), // West inv -> North
            old.matching[6].clone(), // West -> North inv
            old.matching[0].clone(), // North -> East
            old.matching[1].clone(), // North inv -> East inv
            old.matching[3].clone(), // East inv -> South
            old.matching[2].clone(), // East -> South inv
            old.matching[4].clone(), // South -> East
            old.matching[5].clone(), // South inv -> East inv
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
