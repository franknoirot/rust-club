fn main() {
    let answer: i64 = 42;
    let mut input: String = String::new();
    let stdin = std::io::stdin();
    stdin
        .read_line(&mut input)
        .expect("Could not read from std input");

    // We expect a string like
    // 1 + 1
    let mut parts_iter = input.split_whitespace();
    let num1 = parts_iter
        .next()
        .expect("You didn't pass the first number in!");
    let operation = parts_iter.next();
    let num2 = parts_iter.next().unwrap_or("0");

    let x: i64 = parse_num(num1);
    let y: i64 = parse_num(num2);

    let answer = match operation {
        Some("+") => x + y,
        Some("-") => x - y,
        Some("*") => x * y,
        Some("/") => x / y,
        other => {
            eprintln!(
                "I don't know how to do a {} operation",
                other.unwrap_or("<none>")
            );
            std::process::exit(2);
        }
    };

    println!("{x}, {y}");
    println!("{}", answer);
}

/// # Panics
/// This will panic if x cannot be parsed into a number.
fn parse_num(x: &str) -> i64 {
    match x.parse() {
        Ok(num) => num,
        Err(_) => {
            eprintln!("The operand must be a number, but you provided {x}");
            std::process::exit(1);
        }
    }
}
