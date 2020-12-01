use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("input/01").expect("Could not open input file");
    let reader = BufReader::new(file);

    // Read the file line by line into unsigned integer values as puzzle input.

    let expenses: Vec<u32> = reader
        .lines()
        .map(|line| {
            let line = line.expect("Failure while reading file");
            line.trim()
                .parse::<u32>()
                .expect("Could not parse puzzle input into u32")
        })
        .collect();

    // Try all possible combinations of two elements in the vector ------------

    {
        let mut result = None;
        for i in 0..expenses.len() {
            for j in i + 1..expenses.len() {
                if expenses[i] + expenses[j] == 2020 {
                    result = Some(expenses[i] * expenses[j]);
                }
            }
        }

        let result = result.expect("Could not find a result for the first exercise.");
        println!("The result for 01.a is: {}", result);
    }

    // Do the same but for three numbers in part b) ---------------------------

    {
        let mut result = None;
        for i in 0..expenses.len() {
            for j in i + 1..expenses.len() {
                for k in j + 1..expenses.len() {
                    if expenses[i] + expenses[j] + expenses[k] == 2020 {
                        result = Some(expenses[i] * expenses[j] * expenses[k]);
                    }
                }
            }
        }

        let result = result.expect("Could not find a result for 01.b.");
        println!("The result for 01.b is: {}", result);
    }
}
