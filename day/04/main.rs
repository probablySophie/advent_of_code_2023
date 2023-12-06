pub use advent_of_code_2023::shared;

//https://adventofcode.com/2023/day/4

static DEBUG: bool = true;

fn main()
{
	let lines: Vec<String> = shared::read_lines("day/04/input.txt");

	let part_1_sum = part_1(lines.clone());

	println!("Part 1: {}!", part_1_sum);
}

fn part_1(lines: Vec<String>) -> i32
{
	let mut score = 0;
	
	for line in lines
	{
		if line.len() > 0
		{
			let num_winners: u32 = num_winners(line) as u32;
		
			// only add to the score if there actually are any winners
			if num_winners > 0
			{
				score += 2_i32.pow(num_winners-1);
			}
		}
	}

	return score;
}

fn num_winners(line: String) -> i32
{
	/*
	 * Split the line into winning numbers and chosen numbers
	 */

	let left: String = line.split("|").nth(0).unwrap().into();
    let right: String = line.split("|").nth(1).unwrap().into();
    
    // Get a string of just the numbers from the left side
    let left_numbers: String = (&left[( left.find(":").unwrap()+1 )..]).into();
    
    // .split_whitespace() splits by ANY amount of whitespace
    // These are also called _numbers, but they're strings
    let winning_numbers = left_numbers.split_whitespace();
    let chosen_numbers = right.split_whitespace(); 
    
	let mut num_winners = 0;

	/*
	 * Are any of our numbers winners?
	 */

    // there's not a chance this is the most efficient way of doing this, but here we are...
    for number in winning_numbers.clone()
    {
        for number2 in chosen_numbers.clone()
        {
            if number == number2
            {
                num_winners += 1;
            }
        }
    }

	return num_winners;
}
