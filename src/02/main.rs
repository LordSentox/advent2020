use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::Range;
use std::str::FromStr;

/// Structure that describes the policy concerning one specific character inside of a password
/// employing this policy.
struct CharPolicy {
    /// The character this policy concerns itself with
    character: char,
    /// For the first version, this is the minimum number of occurences as well as the maximum
    /// number of occurences of the character in question.
    /// For the second part of the puzzle, this describes the indices that must be checked.
    accepted_occurences: (usize, usize),
}

impl CharPolicy {
    /// Checks if a provided password complies with this policy when interpreted as the policy of
    /// the first version. Returns true if it does, otherwise false.
    pub fn check_password_by_a(&self, password: &str) -> bool {
        // Count the occurences of the important character in the password.
        let occurences = password.chars().filter(|&c| c == self.character).count();

        // Check that the number of occurences is indeed in the desired range.
        occurences >= self.accepted_occurences.0 && occurences <= self.accepted_occurences.1
    }

    /// Checks if a provided password complies with the policy when interpreting it as a policy for
    /// the second part of the puzzle, the 'new' policy.
    pub fn check_password_by_b(&self, password: &str) -> bool {
        let first_is_occ = match password.chars().nth(self.accepted_occurences.0 - 1) {
            Some(c) => self.character == c,
            None => false,
        };
        let second_is_occ = match password.chars().nth(self.accepted_occurences.1 - 1) {
            Some(c) => self.character == c,
            None => false,
        };

        first_is_occ ^ second_is_occ
    }
}

impl FromStr for CharPolicy {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Split the range from the character information part
        let parts: Vec<&str> = s.trim().split_whitespace().collect();
        if parts.len() != 2 {
            return Err(format!(
                "Expected one range and one character part. Found {} parts",
                parts.len()
            ));
        }

        let range_str = parts[0];
        let char_str = parts[1];

        // Check that it's only one char, not more
        if char_str.len() != 1 {
            return Err("Cannot use multiple characters per character policy".to_owned());
        }
        let character = char_str.chars().nth(0).unwrap();

        // Process the string containing the range
        let range_parts: Vec<&str> = range_str.split('-').collect();
        if range_parts.len() != 2 {
            return Err(format!(
                "Range can only be constructed from 2 items, but {} where provided",
                range_parts.len()
            ));
        }

        let accepted_occurences = {
            let min = match range_parts[0].parse() {
                Ok(min) => min,
                Err(err) => return Err(format!("Unable to parse range minimum: {}", err)),
            };
            let max = match range_parts[1].parse() {
                Ok(max) => max,
                Err(err) => return Err(format!("Unable to parse range maximum: {}", err)),
            };

            if min > max {
                return Err("Minimum must be greater than the maximum to create a range".to_owned());
            }

            (min, max)
        };

        Ok(Self {
            character,
            accepted_occurences,
        })
    }
}

fn main() {
    let file = File::open("input/02").expect("Could not open input file");
    let reader = BufReader::new(file);

    let mut num_valid_a = 0;
    let mut num_valid_b = 0;
    for line in reader.lines() {
        let line = line.expect("Failure while reading input file");

        let parts: Vec<&str> = line.split(':').collect();
        if parts.len() != 2 {
            panic!("A line does not contain two parts, a rule and a password");
        }

        let policy = CharPolicy::from_str(parts[0]).expect("Could not construct char policy");

        if policy.check_password_by_a(parts[1].trim()) {
            num_valid_a += 1;
        }
        if policy.check_password_by_b(parts[1].trim()) {
            num_valid_b += 1;
        }
    }

    println!("For part a) {} passwords meet their criteria", num_valid_a);
    println!("For part b) {} passwords meet their criteria", num_valid_b);
}
