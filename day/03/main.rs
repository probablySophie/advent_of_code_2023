pub use advent_of_code_2023::shared;

//https://adventofcode.com/2023/day/3

static DEBUG: bool = true;

fn main()
{
	let lines: Vec<String> = shared::read_lines("day/03/input.txt");

	let mut line_num = 0;
	
	for line in lines.clone()
	{
		// find a number, find the whole number
		//
		// check if there's a symbol to the left
		// check if there's a symbol to the right
		// check if there's a symbol above (and one to the left and right above)
		// check if there's a symbol below (and one to the left and right below)

		line_num += 1;
	}
}
