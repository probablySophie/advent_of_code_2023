fn main() {
    println!("Hello, world!");

	let mut lines: Vec<String> = Vec::new();

	let mut sum = 0;

	for line in lines.iter()
	{
		sum += get_digits_from_line(line.to_string());
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
				last_num = c;
			}
			else // no
			{
				first_num = c;
				last_num = c; // set both to c as per the example
			}
		}
	}

	return (first_num.to_string() + &last_num.to_string()).parse::<i32>().unwrap();
}
