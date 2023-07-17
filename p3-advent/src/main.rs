fn main() {
    // Load in the input TSV file
    let raw_input = std::fs::read_to_string("input.txt")
        .unwrap();

    // Parse the input file (TSV) into a 2D vector of i32s
    let parsed_input: Vec<Vec<i32>> = raw_input.split("\n")
        .filter(|line| line.is_empty() == false)
        .map(|line| line.split("\t"))
        .map(|line| line.map(|num| num.parse::<i32>().unwrap()))
        .map(|line| line.collect())
        .collect();

    // Get the difference between the min and max values in each line
    let differences = parsed_input.iter()
        .map(|line| line.iter().max().unwrap() - line.iter().min().unwrap());

    // Sum the differences
    let sum : i32 = differences.sum();

    // Print the sum
    println!("{}", sum);
}
