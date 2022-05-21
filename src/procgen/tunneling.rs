use crate::fundamentals::*;
use crate::maptools::*;
use crate::utils::*;

pub struct TunnelingGenerator {}
impl TunnelingGenerator {
    pub fn generate_step(&self, m: &mut Map, r: &mut Vec<Room>) -> bool {
        let w = randr(TUNNELING_ROOM_SIZE_MIN..TUNNELING_ROOM_SIZE_MAX);
        let h = randr(TUNNELING_ROOM_SIZE_MIN..TUNNELING_ROOM_SIZE_MAX);
        let x = randr(1..COLS - w);
        let y = randr(1..ROWS - h);

        let curr_room = Room::new(x, y, w, h);
        let mut overlaps = false;

        for room in r.clone() {
            if curr_room.overlaps(room) {
                overlaps = true;
                return false;
            }
        }

        if !overlaps {
            curr_room.carve(m);

            let (curr_x, curr_y) = curr_room.center();

            if !r.is_empty() {
                let prev_room = r[r.len() - 1];
                let (prev_x, prev_y) = prev_room.center();

                if randr(0..1) == 1 {
                    carve_horz_tunnel(m, curr_x, prev_x, curr_y);
                    carve_vert_tunnel(m, curr_y, prev_y, prev_x);
                } else {
                    carve_vert_tunnel(m, curr_y, prev_y, curr_x);
                    carve_horz_tunnel(m, curr_x, prev_x, prev_y);
                }
            }

            r.push(curr_room);
        }
        true
    }
}
