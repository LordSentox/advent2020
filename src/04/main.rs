use std::collections::HashMap;
use std::fs;
use std::str::FromStr;

pub const BIRTH_YEAR: &str = "byr";
pub const ISSUE_YEAR: &str = "iyr";
pub const EXP_YEAR: &str = "eyr";
pub const HEIGHT: &str = "hgt";
pub const HAIRCOLOUR: &str = "hcl";
pub const EYECOLOUR: &str = "ecl";
pub const PASS_ID: &str = "pid";

pub const COUNTRY_ID: &str = "cid";

pub const PASSPORT_REQUIRED_FIELDS: [&str; 7] = [
    BIRTH_YEAR, ISSUE_YEAR, EXP_YEAR, HEIGHT, HAIRCOLOUR, EYECOLOUR, PASS_ID,
];

#[derive(Debug, thiserror::Error)]
enum PassportParseErr {
    #[error("missing fields {0}")]
    MissingField(String),
    #[error("unknown field {0}")]
    WrongFieldFormat(usize),
    #[error("field was found twice: {0}")]
    DoubledField(String),
}

struct Passport {
    fields: HashMap<String, String>,
}

fn check_year(s: &str, min: u32, max: u32) -> bool {
    match s.parse::<u32>() {
        Ok(year) => year >= min && year <= max,
        Err(_) => false,
    }
}

impl Passport {
    /// Checks if all required values are ok according to the rules in b)
    pub fn values_ok(&self) -> bool {
        let birthyear = self.fields.get(BIRTH_YEAR).unwrap();
        if !check_year(birthyear, 1920, 2002) {
            println!("Birthyear invalid: {}", birthyear);
            return false;
        }
        let issue_year = self.fields.get(ISSUE_YEAR).unwrap();
        if !check_year(issue_year, 2010, 2020) {
            println!("Issue year invalid: {}", birthyear);
            return false;
        }
        let exp_year = self.fields.get(EXP_YEAR).unwrap();
        if !check_year(exp_year, 2020, 2030) {
            println!("Expiration year invalid: {}", birthyear);
            return false;
        }

        // Check height
        let mut height = self.fields.get(HEIGHT).unwrap().clone();
        let cm = height.ends_with("cm");
        if !cm && !height.ends_with("in") {
            println!("Height {} does not end in cm or in", height);
            return false;
        }
        height.pop();
        height.pop();
        let height = match height.parse::<u8>() {
            Ok(height) => height,
            Err(_) => {
                println!("Could not parse int from height {}", height);
                return false;
            }
        };
        if cm && height >= 150 && height <= 193 {
        } else if !cm && height >= 59 && height <= 76 {
        } else {
            if cm {
                println!("{}cm not a valid height.", height);
            } else {
                println!("{}in not a valid height.", height);
            }
            return false;
        }

        // Check haircolour
        let haircolour = self.fields.get(HAIRCOLOUR).unwrap();
        if haircolour.len() != 7 || haircolour.chars().nth(0).unwrap() != '#' {
            println!("Not a valid haircolour: {}", haircolour);
            return false;
        }
        for c in haircolour.chars().skip(1) {
            match c {
                '0'..='9' | 'a'..='f' => {}
                o => {
                    println!("{}-character invalid in haircolour.", o);
                    return false;
                }
            }
        }

        // Check eyecolour
        match self.fields.get(EYECOLOUR).unwrap().as_str() {
            "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => {}
            o => {
                println!("{} not a valid eye colour.", o);
                return false;
            }
        }

        // Check passport id
        let passport_id = self.fields.get(PASS_ID).unwrap();
        if passport_id.len() != 9 {
            println!(
                "Invalid passport id length: {} instead of 9",
                passport_id.len()
            );
            return false;
        }
        for c in passport_id.chars() {
            match c {
                '0'..='9' => {}
                o => {
                    println!(
                        "Passport id must consist only of numbers, which {} is not.",
                        o
                    );
                    return false;
                }
            }
        }

        true
    }
}

impl FromStr for Passport {
    type Err = PassportParseErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut fields = HashMap::new();

        let attributes = s.split(&[' ', '\n'][..]);
        for attribute in attributes {
            // Split the attribute (hopefully into two) and add it into the hashmap
            let field: Vec<&str> = attribute.split(':').collect();
            if field.len() != 2 {
                return Err(Self::Err::WrongFieldFormat(field.len()));
            }

            if fields
                .insert(field[0].to_owned(), field[1].to_owned())
                .is_some()
            {
                return Err(Self::Err::DoubledField(field[0].to_owned()));
            }
        }

        // Check if the passport information is complete
        for field in &PASSPORT_REQUIRED_FIELDS {
            if !fields.contains_key(*field) {
                return Err(Self::Err::MissingField(String::from(*field)));
            }
        }

        Ok(Self { fields })
    }
}

fn main() {
    let passports_str = fs::read_to_string("input/04").expect("Could not read passport file");
    let passports_str = passports_str.split("\n\n");

    let mut passports = Vec::new();
    for passport in passports_str {
        match Passport::from_str(passport) {
            Ok(pass) => passports.push(pass),
            Err(e) => println!("Passport invalid: {}", e),
        }
    }

    println!("Number of valid passports for a) {}", passports.len());

    let mut num_valid = 0;
    for passport in passports {
        if passport.values_ok() {
            num_valid += 1;
        }
    }

    println!("Number of valid passports for b) {}", num_valid);
}
