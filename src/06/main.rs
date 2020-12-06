use std::fs;

// Only lowercase, standard latin alphabet are allowed.
const NUM_CHARS: usize = 26;

fn find_any_yes_answer(s: &str) -> [bool; NUM_CHARS] {
    let mut answers = [false; NUM_CHARS];

    for c in s.chars() {
        match c {
            'a'..='z' => answers[c as usize - 'a' as usize] = true,
            '\n' => {}
            other => panic!("Unexpected character: '{}'", other),
        }
    }

    answers
}

fn main() {
    let input = fs::read_to_string("input/06").expect("Could not read input file");
    let group_inputs = input.split("\n\n");

    let mut num_yes_answers = 0;
    for group in group_inputs {
        let yes_answers = find_any_yes_answer(&group);
        for a in yes_answers.iter() {
            if *a {
                num_yes_answers += 1;
            }
        }
    }

    println!(
        "Total number of questions anyone answered yes for a) {}",
        num_yes_answers
    );
}
