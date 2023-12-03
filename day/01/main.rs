
fn main() {
    println!("Hello, world!");

	let lines: Vec<String> = read_lines("day/01/input.txt");

	let mut sum = 0;
	let mut line_num = 0;

	for line in lines.iter()
	{
		//print!("line: {}", line_num);

		if line.len() > 0
		{
			sum += get_digits_from_line(line.to_string());
		}

		line_num += 1;

		//println!("");
	}

	println!("{}", sum);
}

fn get_digits_from_line(input: String) -> i32
{
	let mut first_num: char = 'z';
	let mut last_num: char = '0';

	// thank you https://stackoverflow.com/a/22894333
	for c in input.chars()
	{
		// thank you https://stackoverflow.com/a/29873663
		if c.is_digit(10) // is it a digit?
		{
			if first_num == 'z' // have we already got a first number?
			{
				first_num = c;
				last_num = c; // set both to c as per the example
			}
			else // we already have a first number
			{
				last_num = c; 
			}
		}
	}

	return (first_num.to_string() + &last_num.to_string()).parse::<i32>().unwrap();
}

// thank you https://doc.rust-lang.org/stable/rust-by-example/std_misc/file/read_lines.html
use std::fs::read_to_string;

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename) 
        .unwrap()  // panic on possible file-reading errors
        .lines()  // split the string into an iterator of string slices
        .map(String::from)  // make each slice into a string
        .collect()  // gather them together into a vector
}
