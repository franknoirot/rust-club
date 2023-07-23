use std::{num::ParseIntError, str::FromStr};

fn main() -> Result<(), ParseIntError> {
    let text = std::fs::read_to_string("input.txt")
        .unwrap();

    // Answer 1
    let checksum = get_checksum(&text)?;
    assert_eq!(checksum, 51833); 
    println!("Answer 1 is {}", checksum);

    let divisible_checksum = get_divisible_checksum(&text);
    println!("Answer 2 is {}", divisible_checksum);

    Ok(())
}

type Spreadsheet = Vec<Vec<i32>>;

fn parse_tsv(tsv_text: &str) -> Result<Spreadsheet, ParseIntError> {
    // Parse the input file (TSV) into a 2D vector of i32s
    tsv_text.split('\n')
        .filter(|line| !line.is_empty())
        .map(|line| line.split('\t')
            .map(FromStr::from_str)
            .collect()
        )
        .collect()
}

// Function for the first part of the problem
fn get_checksum(path: &str) -> Result<i32, ParseIntError> {
    let parsed_input = parse_tsv(path)?;

    // Get the difference between the min and max values in each line,
    // then get the sum of those differences
    let differences = parsed_input.iter()
        .map(|line| line.iter().max().unwrap() - line.iter().min().unwrap());

    // Return the sum of the differences
    Ok(differences.sum())
}

// Function for the second part of the problem
fn get_divisible_checksum(path: &str) -> i32 {
    let parsed_input = parse_tsv(path);

    // Sort the items in each row, smallest to largest
    // Then see if each item in the row divides evenly with any others
    // TODO: I don't know how to sort each line and pass that through
    //  the map() functions I want to use
    // let divided_lines = parsed_input.iter()

    // dummy output for now
    return 1
}