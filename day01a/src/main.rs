use std::fs;

fn process_file(s: &str) -> u32 {
    let mut line_sum = 0;
    for line in s.lines(){
        let mut digits = line.chars().filter_map(|c| c.to_digit(10));
        let first_digit = digits.next().unwrap();
        let last_digit = digits.last().unwrap_or(first_digit);
        line_sum += first_digit * 10 + last_digit;
        println!("Line: {}", line);
    }
    line_sum
}

fn main() {
    let file_name = "input.txt";
    println!("Reading from file: {}", file_name);

    let file_contents = fs::read_to_string(file_name).unwrap();

    let line_sum = process_file(&file_contents);
    println!("line_sum: {}", line_sum);
    // println!("Result: {}", result);
}

#[test]
fn advent_of_code_example() {
    let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
    let result = process_file(input);
    assert_eq!(result, 142);
}
