use std::collections::HashMap;
use rand::Rng;

fn main() {
    
    println!("Please enter how many numbers do you want to generate: ");
		let numbers = generate_numbers(input_u32());

	println!("The sum is: {:?}", sum(&numbers));

 	println!("The mean is: {:?}", mean(&numbers));
 	
	println!("The Median is {:?}", median(&numbers));
 	
 	let (mode_val, mode_count) = mode(&numbers);
 	println!("The mode is {:?} ({:?} times)", mode_val, mode_count);

}

fn generate_numbers(count : u32 ) -> Vec<i32>{
	let mut numbers : Vec<i32> = Vec::new();
	println!("Generating {} numbers", count);
	for _ in 0..count {
		numbers.push(rand::thread_rng().gen_range(1,101));	
	}
    numbers
}

fn sum(numbers : &Vec<i32> ) -> i32 {
	let mut sum = 0i32;
    for i in numbers.iter(){
    	sum += *i;
    } 	
    sum
}

fn mean(numbers: &Vec<i32>) -> f32 {
	let average : f32;
 	let sum = sum(&numbers) as f32;
 	average = sum/( numbers.len() as f32 );
 	average
}

fn median(numbers: &Vec<i32>) -> f32 {
	let mut sorted : Vec<i32> = numbers.clone();
	sorted.sort();
	print_numbers(&sorted);
	match sorted.len() % 2 {
		0 => {
			println!("Len/2-1 {:?}, len/2 {:?}", sorted[sorted.len()/2 -1], sorted[sorted.len()/2] );
			(sorted[sorted.len()/2 - 1] as f32 + sorted[sorted.len()/2] as f32) / 2f32
		},
		_ => {
			sorted[sorted.len()/2] as f32
		}
	}
}

fn print_numbers(numbers : &Vec<i32>) {
	print!("[");
	for number in numbers.iter(){
		print!(" {}", number)
	}	
	println!("]");
}

fn mode(numbers: &Vec<i32>) -> (i32, i32) {
	let mut mode_map = HashMap::new();
 	for i in numbers.iter(){
 		let occurences = mode_map.entry(i).or_insert(0);
 		*occurences += 1;
 	}

	
 	let mut mode = 0;
 	let mut max_count = 0;
 	for (key, val) in mode_map.iter(){
 		if *val > max_count {
 			max_count = *val;
 			mode = **key;
 		}
 	}
 	(mode, max_count)
}

fn input_u32() -> u32{
	let mut input = String::new();
	loop {
		match std::io::stdin().read_line(&mut input) {
			Err(_) => {
				println!("Error reading line");

				continue;
			},
			Ok(_) => {
				match input.trim().parse() {
					Err(_) => {
						println!("Enter a valid number");
						input.clear();
						continue;
					},
					Ok(num) => break num,
				}
			}
		}
	}
}