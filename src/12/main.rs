use std::f64::consts::PI;
use std::fs;
use std::str::FromStr;

#[derive(Copy, Clone)]
pub enum CourseCommand {
    DeltaLatitude(i32),
    DeltaLongitude(i32),
    Turn(f64),
    Forward(u16),
}

pub struct Ship {
    latitude: i32,
    longitude: i32,
    heading: f64,
}

impl Ship {
    pub fn pos(&self) -> (i32, i32) {
        (self.latitude, self.longitude)
    }

    pub fn set_course(&mut self, command: CourseCommand) {
        match command {
            CourseCommand::DeltaLatitude(dlat) => self.latitude += dlat,
            CourseCommand::DeltaLongitude(dlong) => self.longitude += dlong,
            CourseCommand::Turn(rad) => self.heading += rad,
            CourseCommand::Forward(amount) => {
                self.latitude -= (self.heading.cos() * amount as f64) as i32;
                self.longitude += (self.heading.sin() * amount as f64) as i32;
            }
        }
    }
}

impl Default for Ship {
    fn default() -> Self {
        Self {
            latitude: 0,
            longitude: 0,
            heading: 1.5 * PI,
        }
    }
}

impl FromStr for CourseCommand {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (word, attribute) = s.split_at(1);
        let attribute = attribute
            .parse::<u16>()
            .expect("Unable to read value for command");

        match word {
            "N" => Ok(Self::DeltaLatitude(-(attribute as i32))),
            "S" => Ok(Self::DeltaLatitude(attribute as i32)),
            "E" => Ok(Self::DeltaLongitude(-(attribute as i32))),
            "W" => Ok(Self::DeltaLongitude(attribute as i32)),
            "L" => Ok(Self::Turn(attribute as f64 / 360. * 2. * PI)),
            "R" => Ok(Self::Turn(-(attribute as f64 / 360. * 2. * PI))),
            "F" => Ok(Self::Forward(attribute)),
            o => panic!("{} is not a valid course word", o),
        }
    }
}

#[inline]
pub fn manhattan_distance(pos1: (i32, i32), pos2: (i32, i32)) -> i32 {
    (pos1.0 - pos2.0).abs() + (pos1.1 - pos2.0).abs()
}

fn main() {
    let commands = fs::read_to_string("input/12").expect("Could not read input");
    let commands: Vec<CourseCommand> = commands
        .lines()
        .map(|line| CourseCommand::from_str(line).unwrap())
        .collect();

    let mut ship = Ship::default();
    let start_pos = ship.pos();
    for command in commands {
        ship.set_course(command);
    }

    println!(
        "Manhattan distance from start to end for a) {}",
        manhattan_distance(ship.pos(), start_pos)
    );
}
