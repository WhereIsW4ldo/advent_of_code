use std::fs;

// This is the main function
pub fn day6() {
	// read file from input
    let inp = fs::read_to_string("day6.txt").expect("could not read file");
	
	const LEN: i32 = 14;
	let mut buffer: Vec<char> = Vec::new();
	
	for (amount, ch) in inp.chars().enumerate()
	{
		//println!("{}", &char); 
		// doel: tel hoeveel unieke characters na elkaar voorkomen
		if buffer.len() == LEN as usize
		{
			println!("stopped at: {amount}");
			break;
		}
		match buffer.iter().position(|&x| x == ch)
		{
			Some(index) => {
				//println!("index: {index}");
				for _ in 0..index+1
				{
					buffer.remove(0);
				}
				buffer.push(ch); 
			},
			None => {
				buffer.push(ch);

			},
		}
		//println!("{:?}", buffer);
	}
	//println!("{:?}", buffer);
}