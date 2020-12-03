use std::fs;
use std::str::FromStr;

/// Terrain with a certain height, but with infinite width in positive integer direction.
struct Terrain {
    /// A two-dimensional vector that contains bools that are true wherever a tree is located and
    /// false where no tree is. The internal order is outer is y, inner is x.
    data: Vec<Vec<bool>>,
}

impl Terrain {
    pub fn height(&self) -> usize {
        self.data.len()
    }

    /// Check if there is a tree on this position. If the index is out of bounds returns false.
    pub fn has_tree(&self, (x, y): (usize, usize)) -> bool {
        if let Some(line) = self.data.get(y) {
            line[x % line.len()]
        } else {
            false
        }
    }

    /// Count the trees on this terrain when following a certain course.
    pub fn trees_on_course(&self, (delta_x, delta_y): (usize, usize)) -> usize {
        let mut x = 0;
        let mut y = 0;
        let mut num_trees = 0;
        while y < self.height() {
            if self.has_tree((x, y)) {
                num_trees += 1;
            }

            x += delta_x;
            y += delta_y;
        }

        num_trees
    }
}

#[derive(thiserror::Error, Debug)]
enum TerrainParseError {
    #[error("unknown token {0}")]
    UnknownToken(char),
}
impl FromStr for Terrain {
    type Err = TerrainParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut data = Vec::new();
        for line in s.lines() {
            let mut line_data = Vec::with_capacity(line.len());
            for c in line.chars() {
                line_data.push(match c {
                    '.' => false,
                    '#' => true,
                    other => return Err(Self::Err::UnknownToken(other)),
                });
            }

            data.push(line_data);
        }

        Ok(Self { data })
    }
}

fn main() {
    let terrain = fs::read_to_string("input/03").expect("Could not read terrain file");
    let terrain = Terrain::from_str(&terrain).expect("Could not parse terrain from string");

    // Plot the courses for both exercises (the first one is for a)
    let courses = [(3, 1), (1, 1), (5, 1), (7, 1), (1, 2)];

    let trees_on_courses: Vec<usize> = courses
        .iter()
        .map(|&course| terrain.trees_on_course(course))
        .collect();

    println!("Trees on course for a) {}", trees_on_courses[0]);

    // The number of trees encountered for each course multiplied is the solution for  b)
    let multiplied: usize = trees_on_courses.iter().product();
    println!("Multiplied trees for b) {}", multiplied);
}
