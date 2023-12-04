
// https://adventofcode.com/2023/day/1

fn main() {
    println!("Day 1!\n");

	let lines: Vec<String> = read_lines("day/01/input.txt");

	let mut sum_part_1 = 0;
	let mut sum_part_2 = 0;
	let mut line_num = 0;

	for line in lines.iter()
	{
		//print!("line: {}", line_num);

		if line.len() > 0
		{
			let part_1 = get_digits_from_line(line.to_string());
			let part_2 = part_2(line.to_string());

			sum_part_1 += part_1;
			sum_part_2 += part_2;

			//print!("\t{}\t{}", part_1, part_2);
		}

		line_num += 1;

		//println!("");
	}

	println!("Part 1: {}", sum_part_1);
	println!("Part 2: {}", sum_part_2);
}

/*	PART 1
 *	take all the input lines, get the first and last occurance of any digit and return them as a two digit number
 *  e.g. ak2khrvb5 = 25
 *  and  abcdef1 = 11
 * */

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


/* PART 2
 * The same as part one, but some of the lines have text digits "one", "two", ...
 * */

// the theory here is that if we do a weird location recording thing then we can just search for the number name strings and that'll be fine?
fn part_2(line: String) -> i32
{
	let mut first_num: char = 'z';
	let mut first_num_loc: i32 = 100000; // so anything else will register as the first number

	let mut last_num: char = '0';
	let mut last_num_loc: i32 = -1; // so anything else will register as the last number

	let mut loc = 0;

	for c in line.chars()
	{
		// thank you https://stackoverflow.com/a/29873663
		if c.is_digit(10) // is it a digit?
		{
			if first_num == 'z' // have we already got a first number?
			{
				first_num = c;
				last_num = c; // set both to c as per the example

				first_num_loc = loc;
				last_num_loc = loc;
			}
			else // we already have a first number
			{
				last_num = c; 
				last_num_loc = loc;
			}
		}

		loc += 1;
	}


	let numbers = vec!("zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine");
	let mut num = 0;

	for number in numbers.iter()
	{
		if let Ok(indexes) = string_num_from_line(line.clone().into(), number)
		{
			if indexes.0 < first_num_loc
			{
				first_num_loc = indexes.0;
				first_num = char::from_digit(num as u32, 10).unwrap();
			}
			if indexes.1 > last_num_loc
			{
				last_num_loc = indexes.1;
				last_num = char::from_digit(num as u32, 10).unwrap();
			}
		}

		num += 1;
	}

	return (first_num.to_string() + &last_num.to_string()).parse::<i32>().unwrap();
}

fn string_num_from_line(line: String, num: &str) -> Result<(i32, i32), &'static str>
{
	if let Some(output) = line.find(num)
	{
	    let first = output as i32;
	    let last = line.rfind(num).unwrap() as i32;
	    
	    return Ok((first,last));
	}
	Err("number not found as string")
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
