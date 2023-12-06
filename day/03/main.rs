pub use advent_of_code_2023::shared;

//https://adventofcode.com/2023/day/3

static DEBUG: bool = true;

fn main()
{
	let mut lines: Vec<String> = shared::read_lines("day/03/input.txt");
	
	let mut part_1_sum = 0;
	let mut part_2_sum = 0;

	/* 
	 * Add an 'empty' pre & post line to avoid some hassle with the first and last lines later
	 */

	let empty_line: String = ".".repeat(lines.first().unwrap().len());
	
	lines.insert(0, empty_line.clone());
	lines.push(empty_line.clone());

	/* 
	 *		Add an 'empty' char to the beginning and end of every line to make life easier later
	 */

	let mut new_lines: Vec<String> = Vec::new();
	
	for line in lines
	{
	    let mut temp = line;
	    
	    temp.push('.');
	    temp.insert(0,'.');
	    
	    new_lines.push(temp);
	}	
	let lines = new_lines;

	/*
	 *		Do the thing!
	 */

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
			if DEBUG
			{
				print!("{} : ", line_num);
			}

			part_1_sum += part_1(
				lines.get(line_num-1).unwrap().clone(), 
				line.clone(), 
				lines.get(line_num+1).unwrap().clone()
			);

			if DEBUG
			{
				println!("");
			}
		}

		line_num += 1;
	});
	
	println!("Part 1: {}!", part_1_sum);
	println!("Part 2: {}!", part_2_sum);
}

fn part_1(prev_line: String, line: String, next_line: String) -> i32
{
	let mut sum: i32 = 0;
	let mut num = 0;
    
    let mut is_part = false; // because we only add numbers to the sum if they're a part number

    // get the strings' chars as a vector
    let prev_chars: Vec<char> = prev_line.chars().collect();
    let chars: Vec<char> = line.chars().collect();
    let next_chars: Vec<char> = next_line.chars().collect();

    for (i, c) in line.chars().enumerate()
    {
        // ignore the first and last characters (we added them earler)
        if 0 < i && i < line.len() - 1
        {
            // is the current character a number?
            if let Ok(digit) = c.to_string().parse::<i32>()
            {
                num = (num*10) + digit; // add the current character to the stored number
                
                // check if the number is ajacent to a symbol
                // this is about to be very ugly and double check a bunch of chars
                
                let mut num_symbols = 0;
                
                num_symbols += is_symbol(*prev_chars.get(i-1).unwrap());
                num_symbols += is_symbol(*prev_chars.get(i).unwrap());
                num_symbols += is_symbol(*prev_chars.get(i+1).unwrap());
                
                num_symbols += is_symbol(*chars.get(i-1).unwrap());
                num_symbols += is_symbol(*chars.get(i+1).unwrap());
                
                num_symbols += is_symbol(*next_chars.get(i-1).unwrap());
                num_symbols += is_symbol(*next_chars.get(i).unwrap());
                num_symbols += is_symbol(*next_chars.get(i+1).unwrap());
                
                if num_symbols > 0
                {
                    is_part = true;
                } 
            }
            else if num != 0
            {
                // if it IS a part, then add the current number to the sum
                if is_part
                {
					if DEBUG
					{
						print!("{} ",num);
					}

                    sum += num;
                    is_part = false; // reset is_part
                }

				num = 0;
            }
        }
    }

	// catch any final numbers that don't have a symbol after them
	if is_part
	{
		if DEBUG
		{
			print!("{} ",num);
		}

		sum += num;
	}

	return sum;
}


fn is_symbol(c: char) -> i32
{
    // is it a number?
    if let Ok(_digit) = c.to_string().parse::<i32>()
    {
        return 0; // a number is not a symbol
    }
    
    // is it 'empty'
    if c == '.'
    {
        return 0;  // 'empty' is not a symbol
    }
    
    return 1; // if we've gotten here then it's probably a symbol
}
