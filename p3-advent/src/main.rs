fn main() {
    // Answer 1
    let checksum = get_checksum();
    assert_eq!(checksum, 51833); 
    println!("Answer 1 is {}", checksum);

    let divisible_checksum = get_divisible_checksum();
    println!("Answer 2 is {}", divisible_checksum);
}

fn load_parse_txt_as_tsv(path: &str) -> Vec<Vec<i32>> {
    // Load in the input TSV file
    let raw_input = std::fs::read_to_string(path)
        .unwrap();

    // Parse the input file (TSV) into a 2D vector of i32s
    let parsed_input: Vec<Vec<i32>> = raw_input.split("\n")
        .filter(|line| line.is_empty() == false)
        .map(|line| line.split("\t"))
        .map(|line| line.map(|num| num.parse::<i32>().unwrap()))
        .map(|line| line.collect())
        .collect();

    return parsed_input
}

// Function for the first part of the problem
fn get_checksum() -> i32 {
    let parsed_input = load_parse_txt_as_tsv("input.txt");

    // Get the difference between the min and max values in each line,
    // then get the sum of those differences
    let differences = parsed_input.iter()
        .map(|line| line.iter().max().unwrap() - line.iter().min().unwrap());

    // Return the sum of the differences
    differences.sum()
}

// Function for the second part of the problem
fn get_divisible_checksum() -> i32 {
    let parsed_input = load_parse_txt_as_tsv("input.txt");

    // Sort the items in each row, smallest to largest
    // Then see if each item in the row divides evenly with any others
    // TODO: I don't know how to sort each line and pass that through
    //  the map() functions I want to use
    let divided_lines = parsed_input.iter()
        .for_each(|&mut line| line.sort());

    // dummy output for now
    return 1
}