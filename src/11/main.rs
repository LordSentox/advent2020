use std::fs;
use std::mem;
use std::str::FromStr;

pub const DIRECTIONS: [(isize, isize); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

#[derive(Copy, Clone, PartialEq, Eq)]
enum Tile {
    Floor,
    SeatEmpty,
    SeatTaken,
}

#[derive(Clone, PartialEq)]
struct TileMap {
    tiles: Vec<Vec<Tile>>,
}

impl Tile {
    pub fn from_char(c: char) -> Self {
        match c {
            '.' => Self::Floor,
            'L' => Self::SeatEmpty,
            '#' => Self::SeatTaken,
            o => panic!("Character '{}' is not a tile.", o),
        }
    }

    pub fn occupied(&self) -> bool {
        match &self {
            &Self::SeatTaken => true,
            _ => false,
        }
    }
}

impl TileMap {
    pub fn with_size((width, height): (usize, usize)) -> Self {
        Self {
            tiles: vec![vec![Tile::Floor; width]; height],
        }
    }

    pub fn num_occupied(&self) -> usize {
        let mut count = 0;
        for row in &self.tiles {
            for tile in row {
                if tile.occupied() {
                    count += 1;
                }
            }
        }

        count
    }

    pub fn num_adjacent_occupied(&self, (x, y): (usize, usize)) -> u8 {
        assert!(self.get((x, y)).is_some());

        // First, create a collection of all positions that are right next to this one.
        let mut adjacent: Vec<(usize, usize)> = DIRECTIONS
            .iter()
            .filter_map(|(dx, dy)| {
                let x = x as isize + dx;
                let y = y as isize + dy;
                if x >= 0 && y >= 0 {
                    Some((x as usize, y as usize))
                } else {
                    None
                }
            })
            .collect();

        // Get the actual seats at that positions, if they exist and then count how many of them
        // are occupied.
        adjacent
            .into_iter()
            .filter(|&pos| match self.get(pos) {
                Some(seat) => seat.occupied(),
                None => false,
            })
            .count() as u8
    }

    pub fn find_view(&self, (x, y): (usize, usize), (dx, dy): (isize, isize)) -> Option<Tile> {
        let x = x as isize + dx;
        let y = y as isize + dy;
        if x < 0 || y < 0 {
            return None;
        }
        let x = x as usize;
        let y = y as usize;

        match self.get((x, y)) {
            Some(Tile::Floor) => self.find_view((x, y), (dx, dy)),
            other => other,
        }
    }

    pub fn num_los_occupied(&self, (x, y): (usize, usize)) -> u8 {
        let mut count = 0;
        for dir in &DIRECTIONS {
            match self.find_view((x, y), *dir) {
                Some(Tile::SeatTaken) => count += 1,
                _ => {}
            }
        }

        count
    }

    pub fn get(&self, (x, y): (usize, usize)) -> Option<Tile> {
        match self.tiles.get(y) {
            Some(row) => row.get(x).copied(),
            None => None,
        }
    }

    pub fn set(&mut self, (x, y): (usize, usize), tile: Tile) {
        self.tiles[y][x] = tile;
    }

    pub fn size(&self) -> (usize, usize) {
        if !self.tiles.is_empty() {
            (self.tiles[0].len(), self.tiles.len())
        } else {
            (0, 0)
        }
    }
}

impl FromStr for TileMap {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut tiles = Vec::new();
        for line in s.lines() {
            let tile_row = line.chars().map(|c| Tile::from_char(c)).collect();
            tiles.push(tile_row);
        }

        Ok(Self { tiles })
    }
}

fn perform_step_a(source: &TileMap, target: &mut TileMap) {
    let (width, height) = source.size();
    for x in 0..width {
        for y in 0..height {
            let tile = source.get((x, y)).expect("Tile does not exist");
            target.set(
                (x, y),
                match (tile, source.num_adjacent_occupied((x, y))) {
                    (Tile::SeatEmpty, 0) => Tile::SeatTaken,
                    (Tile::SeatTaken, 4..=8) => Tile::SeatEmpty,
                    (tile, _) => tile,
                },
            );
        }
    }
}

fn perform_step_b(source: &TileMap, target: &mut TileMap) {
    let (width, height) = source.size();
    for x in 0..width {
        for y in 0..height {
            let tile = source.get((x, y)).expect("Tile does not exist");
            target.set(
                (x, y),
                match (tile, source.num_los_occupied((x, y))) {
                    (Tile::SeatEmpty, 0) => Tile::SeatTaken,
                    (Tile::SeatTaken, 5..=8) => Tile::SeatEmpty,
                    (tile, _) => tile,
                },
            );
        }
    }
}

fn main() {
    let tile_map = fs::read_to_string("input/11").expect("Unable to read input file");
    let map = TileMap::from_str(&tile_map).expect("Could not parse tile map");

    // Create two maps, where one is the base map for every step and the other is the next one.
    let mut map_one = map.clone();
    let mut map_two = TileMap::with_size(map_one.size());

    // The source and target maps are switched on each step, for now the first map is the source
    // map.
    {
        let mut source_map = &mut map_one;
        let mut target_map = &mut map_two;
        loop {
            perform_step_a(source_map, target_map);

            // When we've reached a stable state, stop
            if *source_map == *target_map {
                break;
            }

            mem::swap(&mut source_map, &mut target_map);
        }
    }

    println!("Number of occupied seats for a) {}", map_one.num_occupied());

    map_one = map;
    {
        let mut source_map = &mut map_one;
        let mut target_map = &mut map_two;
        loop {
            perform_step_b(source_map, target_map);

            // When we've reached a stable state, stop
            if *source_map == *target_map {
                break;
            }

            mem::swap(&mut source_map, &mut target_map);
        }
    }

    println!("Number of occupied seats for b) {}", map_one.num_occupied());
}
