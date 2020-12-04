use std::collections::HashMap;
use std::fs;
use std::str::FromStr;

pub const PASSPORT_REQUIRED_FIELDS: [&str; 7] = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

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

impl Passport {
    pub fn get(&self, field: &str) -> Option<&String> {
        self.fields.get(field)
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
}
