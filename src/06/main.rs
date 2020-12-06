use std::fs;

// Only lowercase, standard latin alphabet are allowed.
const NUM_CHARS: usize = 26;

// Count the number of yes answers of a group. Returns also the number of people in that group for
// reference.
fn count_yes_answers(s: &str) -> ([usize; NUM_CHARS], usize) {
    // Trimming is necessary for the last element to be counted properly. Otherwise it thinks there
    // is one more person in the group.
    let s = s.trim_end();

    let mut answers = [0; NUM_CHARS];
    let mut num_people = 1;
    for c in s.chars() {
        match c {
            'a'..='z' => answers[c as usize - 'a' as usize] += 1,
            '\n' => num_people += 1,
            other => panic!("Unexpected character: '{}'", other),
        }
    }

    (answers, num_people)
}

fn main() {
    let input = fs::read_to_string("input/06").expect("Could not read input file");
    let group_inputs = input.split("\n\n");

    let mut num_any_yes = 0;
    let mut num_all_yes = 0;
    for group in group_inputs {
        let (yes_answers, num_people) = count_yes_answers(&group);

        for a in yes_answers.iter() {
            assert!(*a <= num_people);
            if *a >= 1 {
                num_any_yes += 1;
            }
            if *a == num_people {
                num_all_yes += 1;
            }
        }
    }

    println!(
        "Total number of questions anyone answered yes for a) {}",
        num_any_yes
    );

    println!(
        "Total number of questions all answered yes for b) {}",
        num_all_yes
    );
}
