pub use advent_of_code_2023::shared;

//https://adventofcode.com/2023/day/2

static DEBUG: bool = false;

fn main()
{
	let lines: Vec<String> = shared::read_lines("day/02/input.txt");
	let mut line_num = 0;

	let mut part_1_sum = 0;
	let mut part_2_sum = 0;

	for line in lines.iter()
	{
		if line.len() > 0 // skip empty lines, as a LOT of the code will panic if a line is empty
		{
			let part_1_result = part_1(line.clone());
			let part_2_result = part_2(line.clone());

			part_1_sum += part_1_result;
			part_2_sum += part_2_result;
		
			if DEBUG
			{
				println!("{}\t{}\t{}", line_num, part_1_result, part_2_result);
			}
		}
		line_num += 1;
	}

	println!("Part 1: {}!", part_1_sum);
	println!("Part 2: {}!", part_2_sum);
}


/* PART 1
 *
 * Lines are received in the form Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
 *
 * We need to confirm whether a bag containing 12 red cubes, 13 green cubes, and 14 blue cubes could feasibly have those cubes drawn from it
 * */
fn part_1(line: String) -> i32
{
	let max_red = 12;
	let max_green = 13;
	let max_blue = 14;

	// Get a substring from "Game " to ":" & then parse that as an i32
	let game_id: i32 = (&line[5..line.find(":").unwrap()]).parse::<i32>().unwrap();

	// just the rounds of the game
	let line_without_game: String = (&line[ (line.find(":").unwrap()+1).. ]).into();

	let rounds = line_without_game.split(";");

	for round in rounds
	{
		let cubes = round.split(",");

		for cube_colour in cubes
		{
			// skip the first " " then find the right most " " and convert what's inbetween into a number
			let num: i32 = (&cube_colour[1..cube_colour.rfind(" ").unwrap()]).parse::<i32>().unwrap();

			// get the colour
			let colour = &cube_colour[(cube_colour.rfind(" ").unwrap() + 1)..];
           
			// if the number of cubes is larger than the max allowable number of cubes of that colour, return 0
            match colour
            {
                "red" => 
				{
					if num > max_red	{return 0};
				},
                "green" => 
				{
					if num > max_green	{return 0};
				},
                "blue" => 
				{
					if num > max_blue	{return 0};
				},
                _ => 
				{
					println!("uh oh")
				},
            }
		}
	}


	// 
	
	//
	//
	
	return game_id; // assuming we got to this point, 
}

fn part_2(line: String) -> i32
{
	let mut max_red = 0;
	let mut max_green = 0;
	let mut max_blue = 0;

	// Get a substring from "Game " to ":" & then parse that as an i32
	let game_id: i32 = (&line[5..line.find(":").unwrap()]).parse::<i32>().unwrap();

	// just the rounds of the game
	let line_without_game: String = (&line[ (line.find(":").unwrap()+1).. ]).into();

	let rounds = line_without_game.split(";");

	for round in rounds
	{
		let cubes = round.split(",");

		for cube_colour in cubes
		{
			// skip the first " " then find the right most " " and convert what's inbetween into a number
			let num: i32 = (&cube_colour[1..cube_colour.rfind(" ").unwrap()]).parse::<i32>().unwrap();

			// get the colour
			let colour = &cube_colour[(cube_colour.rfind(" ").unwrap() + 1)..];
           
			// if the number of cubes is larger than the max allowable number of cubes of that colour, return 0
            match colour
            {
                "red" => 
				{
					if num > max_red	{max_red = num};
				},
                "green" => 
				{
					if num > max_green	{max_green = num};
				},
                "blue" => 
				{
					if num > max_blue	{max_blue = num};
				},
                _ => 
				{
					println!("uh oh")
				},
            }
		}
	}
	
	return max_red * max_green * max_blue; // assuming we got to this point, 
}
