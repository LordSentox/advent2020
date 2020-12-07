use std::fs;
use std::str::{FromStr, SplitWhitespace};

#[derive(PartialEq, Eq, Clone)]
struct BagColour {
    special: String,
    base: String,
}

#[derive(Clone)]
struct BagRule {
    container_colour: BagColour,
    contains: Vec<(BagColour, usize)>,
}

impl BagColour {
    pub fn new(special: String, base: String) -> Self {
        Self { special, base }
    }
}

impl BagRule {
    pub fn can_contain(&self, (bag_colour, amount): &(BagColour, usize)) -> bool {
        for (bc, a) in &self.contains {
            if bc == bag_colour && a >= amount {
                return true;
            }
        }

        false
    }

    pub fn container_colour(&self) -> &BagColour {
        &self.container_colour
    }
}

#[derive(Debug, thiserror::Error)]
enum BagRuleParseErr {
    #[error("expected token {0}, found {1}")]
    UnexpectedToken(String, String),
    #[error("expected token of type {0}, which token {1} is not")]
    TokenType(String, String),
    #[error("ended in invalid parsing state, string too short")]
    UnexpectedEOF,
}

impl FromStr for BagRule {
    type Err = BagRuleParseErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut words = s.split_whitespace();

        // Helper function to read a token when there must be one. Returns that token or a
        // returnable error if there is no string given.
        fn read_token<'a>(words: &mut SplitWhitespace<'a>) -> Result<&'a str, BagRuleParseErr> {
            match words.next() {
                Some(s) => Ok(s),
                None => Err(BagRuleParseErr::UnexpectedEOF),
            }
        }

        // Helper to read the next token and check that it is indeed the expected token. Useful
        // when there can be only one token next but it must exist.
        fn expect_token<'a>(
            words: &mut SplitWhitespace<'a>,
            expected: &str,
        ) -> Result<(), BagRuleParseErr> {
            let token = read_token(words)?;
            if token == expected {
                Ok(())
            } else {
                Err(BagRuleParseErr::UnexpectedToken(
                    expected.to_owned(),
                    token.to_owned(),
                ))
            }
        }

        // The first colour will have no number identifier but must exist
        let container_colour = BagColour::new(
            read_token(&mut words)?.to_owned(),
            read_token(&mut words)?.to_owned(),
        );

        expect_token(&mut words, "bags")?;
        expect_token(&mut words, "contain")?;

        let mut contains = Vec::new();
        loop {
            let amount = read_token(&mut words)?;
            // Token that declares no other bags will be contained. Check for correct format
            if amount == "no" {
                expect_token(&mut words, "other")?;
                expect_token(&mut words, "bags.")?;
                break;
            }

            // Read information about the bag type that can be contained.
            let amount = match amount.parse::<usize>() {
                Ok(amount) => amount,
                Err(_) => return Err(Self::Err::TokenType("usize".to_owned(), amount.to_owned())),
            };
            let bag_colour = BagColour::new(
                read_token(&mut words)?.to_owned(),
                read_token(&mut words)?.to_owned(),
            );

            contains.push((bag_colour, amount));

            // Check if there will be other bag types coming or not and abort if not.
            match (amount, read_token(&mut words)?) {
                (1, "bag,") | (_, "bags,") => {}     // continue on comma
                (1, "bag.") | (_, "bags.") => break, // abort on period
                (_, other) => {
                    return Err(Self::Err::UnexpectedToken(
                        "bag continuation".to_owned(),
                        other.to_owned(),
                    ))
                }
            }
        }

        Ok(Self {
            container_colour,
            contains,
        })
    }
}

fn main() {
    let input = fs::read_to_string("input/07").expect("Unable to read input file");
    let rule_strings = input.split('\n');

    let mut rules = Vec::new();
    for rule in rule_strings {
        // Ignore empty lines
        if rule.trim().is_empty() {
            continue;
        }

        rules.push(match BagRule::from_str(rule) {
            Ok(rule) => rule,
            Err(err) => {
                println!("Unable to parse rule from: {}", rule);
                panic!("{}", err);
            }
        });
    }

    let own_bag = (
        BagColour {
            special: "shiny".to_owned(),
            base: "gold".to_owned(),
        },
        1,
    );

    // Mark all rules that can immediately contain our own bag
    let mut possible_rules: Vec<bool> = rules
        .iter()
        .map(|rule| rule.can_contain(&own_bag))
        .collect();

    let mut added_rules = true;
    while added_rules {
        added_rules = false;

        // Add all rules that can contain any of the bags that are able to contain at least one
        // shiny gold bag.
        for (i, rule) in rules.iter().enumerate() {
            // Ignore rules that already have been added
            if possible_rules[i] {
                continue;
            }

            // Check if any of the rules that have been confirmed by the algorithm to be able to
            // contain our own bag are contained in the current rule.
            if possible_rules.iter().enumerate().any(|(i, &possible)| {
                possible && rule.can_contain(&(rules[i].container_colour().clone(), 1))
            }) {
                added_rules = true;
                possible_rules[i] = true;
            }
        }
    }

    let num_possible = possible_rules.iter().filter(|&possible| *possible).count();

    println!("Number of possible containers for a) {}", num_possible);
}
