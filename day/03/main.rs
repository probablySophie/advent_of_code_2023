pub use advent_of_code_2023::shared;

//https://adventofcode.com/2023/day/3

static DEBUG: bool = true;

fn main()
{
	let mut lines: Vec<String> = shared::read_lines("day/03/input.txt");
	
	let mut part_1_sum = 0;
	let mut part_2_sum = 0;

	// add an 'empty' pre & post line to avoid some hassle with the first and last lines later	
	let empty_line: String = ".".repeat(lines.first().unwrap().len());
	
	lines.insert(0, empty_line.clone());
	lines.push(empty_line.clone());

	let mut line_num = 0; 

	lines.iter().for_each(|line|{

		// skip the first and last lines
		if line_num == 0 || line_num == lines.len() - 1
		{
			// do nothing
			if DEBUG
			{
				println!("Skipping line: {}", line_num);
			}
		}
		else 
		{
			part_1_sum = part_1(
				lines.get(line_num-1).unwrap().clone(), 
				line.clone(), 
				lines.get(line_num+1).unwrap().clone()
			);
		}

		line_num += 1;
	});
	
	println!("Part 1: {}!", part_1_sum);
	println!("Part 2: {}!", part_2_sum);
}

fn part_1(prev_line: String, line: String, next_line: String) -> i32
{
	let mut sum: i32 = 0;
		
		// find a number, find the whole number
		//
		// check if there's a symbol to the left
		// check if there's a symbol to the right
		// check if there's a symbol above (and one to the left and right above)
		// check if there's a symbol below (and one to the left and right below)

	return sum;
}
