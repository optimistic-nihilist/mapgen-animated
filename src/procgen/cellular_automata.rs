use crate::{fundamentals::*, maptools::*, render_map, utils::*, DrawTextureParams};
use macroquad::prelude::*;
use std::collections::HashMap;

const DEATH_LIMIT: i32 = 3;
const BIRTH_LIMIT: i32 = 4;

pub fn randomize_map_seal_edges(m: &mut Map) {
    for y in 0..ROWS {
        for x in 0..COLS {
            if (x == 0) | (x == COLS - 1) | (y == 0) | (y == ROWS - 1) {
                m[y as usize][x as usize] = TileType::Wall as i32;
            } else {
                m[y as usize][x as usize] = match randr(0..100) {
                    0..=32 => TileType::Wall as i32,
                    _ => TileType::Floor as i32,
                };
            }
        }
    }
}

pub async fn evolve_map(
    m: &mut Map,
    tiles: &HashMap<TileType, DrawTextureParams>,
    texture: Texture2D,
) {
    render_map(&tiles, texture, &m);
    for y in 0..ROWS {
        for x in 0..COLS {
            let neighbor_count = count_alive_neighbors(m, x, y);
            if m[y as usize][x as usize] == TileType::Wall as i32 {
                if neighbor_count < DEATH_LIMIT {
                    m[y as usize][x as usize] = TileType::Floor as i32
                }
            } else {
                if BIRTH_LIMIT < neighbor_count {
                    m[y as usize][x as usize] = TileType::Wall as i32
                }
            }
        }
    }
    std::thread::sleep(std::time::Duration::from_millis(500));
    next_frame().await
}

fn count_alive_neighbors(m: &Map, x: i32, y: i32) -> i32 {
    let mut count = 0;
    for i in -1..2 {
        for j in -1..2 {
            let nx = x + i;
            let ny = y + j;
            if (nx < 0) | (nx >= COLS) | (ny < 0) | (ny >= ROWS) {
                count += 1;
            } else if m[ny as usize][nx as usize] == TileType::Wall as i32 {
                count += 1;
            }
        }
    }
    count
}
