use std::cmp;
use std::fs;
use std::ops::Deref;
use std::str::FromStr;

const NUM_ROWS: u8 = 128;
const NUM_COLS: u8 = 8;
const NUM_IDS: usize = NUM_ROWS as usize * 8;

struct BoardingInfo(Vec<Direction>);

impl Deref for BoardingInfo {
    type Target = [Direction];
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

enum Direction {
    // Negative x direction
    Left,
    // Positive x direction
    Right,
    // Negative z direction
    Front,
    // Positive z direction
    Back,
}

impl FromStr for BoardingInfo {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut info = Vec::with_capacity(s.len());
        for c in s.chars() {
            let dir = match c {
                'L' => Direction::Left,
                'R' => Direction::Right,
                'F' => Direction::Front,
                'B' => Direction::Back,
                o => return Err(format!("{} is not a valid direction", o)),
            };
            info.push(dir);
        }

        Ok(BoardingInfo(info))
    }
}

fn binary_search_seat(info: &[Direction]) -> (u8, u8) {
    let mut min_x = 0;
    let mut min_z = 0;
    let mut max_x = NUM_COLS;
    let mut max_z = NUM_ROWS;

    for dir in info {
        let mid_x = (min_x + max_x) / 2;
        let mid_z = (min_z + max_z) / 2;
        match dir {
            Direction::Left => max_x = mid_x,
            Direction::Right => min_x = mid_x,
            Direction::Front => max_z = mid_z,
            Direction::Back => min_z = mid_z,
        }
    }

    (min_x, min_z)
}

fn main() {
    let input = fs::read_to_string("input/05").expect("Unable to read input file");

    let mut max_seat_id = 0;
    let mut available_seats = [true; NUM_IDS];
    for line in input.lines() {
        let boarding_info = BoardingInfo::from_str(line).expect("Could not read boarding info");
        let (seat_x, seat_z) = binary_search_seat(&boarding_info);

        let seat_id = seat_z as u32 * 8 + seat_x as u32;
        max_seat_id = cmp::max(max_seat_id, seat_id);
        available_seats[seat_id as usize] = false;
    }

    println!("Maximum seat id for a) {}", max_seat_id);

    /* The very first and the very last seats that appear available are just missing. Mark them as
     * unavailable as well
     */
    // First for the front
    for available in available_seats.iter_mut() {
        // Abort with the first seat unavailable
        if !*available {
            break;
        }
        *available = false;
    }
    // Then in the back too
    for available in available_seats.iter_mut().rev() {
        // Abort with the first seat unavailable
        if !*available {
            break;
        }
        *available = false;
    }

    /* Find the one seat not totally at the front and not totally in the back that is not taken
     * starting with the second row and ending with the second to last row.
     */
    let mut own_seat_id = None;
    for (i, &available) in available_seats.iter().enumerate() {
        if available {
            own_seat_id = Some(i);
            break;
        }
    }

    match own_seat_id {
        Some(id) => println!("Own seat id for b) {}", id),
        None => panic!("Could not find an open seat"),
    }
}
