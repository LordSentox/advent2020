use std::fs;
use std::ops::Range;

const CHECK_LEN: usize = 25;

fn num_two_sums(numbers: &[u64], sum: u64) -> usize {
    let mut count_two_sums = 0;
    for i in 0..numbers.len() {
        for j in i + 1..numbers.len() {
            if numbers[i] != numbers[j] && numbers[i] + numbers[j] == sum {
                count_two_sums += 1;
            }
        }
    }

    count_two_sums
}

fn main() {
    let input = fs::read_to_string("input/09").expect("Unable to open input file");
    let input: Vec<u64> = input
        .lines()
        .map(|line| {
            line.parse::<u64>()
                .expect("Could not parse a u64 from a line")
        })
        .collect();

    let mut first_broken = None;
    for i in CHECK_LEN..input.len() {
        if num_two_sums(&input[(i - CHECK_LEN)..i], input[i]) == 0 {
            first_broken = Some(input[i]);
            break;
        }
    }

    let first_broken = first_broken.expect("No broken number was found");
    println!("First broken number for a) {}", first_broken);

    let mut set_size = 2;
    let mut found_set: Option<Range<usize>> = None;
    'algo: while set_size < input.len() {
        for i in 0..(input.len() - set_size) {
            if input[i..i + set_size].iter().sum::<u64>() == first_broken {
                println!("Found set: {:?}", &input[i..i + set_size]);
                found_set = Some(i..i + set_size);
                break 'algo;
            }
        }

        set_size += 1;
    }

    let found_set = found_set.expect("Could not find a matching set");
    let max = input[found_set.clone()].iter().max().unwrap();
    let min = input[found_set].iter().min().unwrap();

    println!("Found set with size {}", set_size);
    println!(
        "Summing set lowest and biggest together for b) {}",
        min + max
    );
}
