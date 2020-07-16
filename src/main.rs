use std::collections::HashMap;
use rand::Rng;

fn main() {
    
    println!("Generating 100 numbers");
    let mut numbers : [i32; 100] = [0; 100];
    
    print!("[ ");
    for i in numbers.iter_mut(){
    	*i = rand::thread_rng().gen_range(1,101);
    	print!("{:?}, ", i);
    }
    println!("]");

    let mut sum = 0i32;
    for i in numbers.iter(){
    	sum += *i;
    }
 	println!("The sum is: {:?}", sum);
 	let average : f32;
 	let sum = sum as f32;
 	average = sum/100f32;
 	println!("The average is: {:?}", average);
 	

 	
 	numbers.sort();
 	let mut mode_map = HashMap::new();
 	for i in numbers.iter(){
 		let count = mode_map.entry(i).or_insert(0);
 		*count += 1;
 	}

 	let mean : f32 = (numbers[49] as f32 + numbers[50] as f32)/2f32;

 	println!("The Mean is {:?}", mean );
 	
 	let mut mode = 0;
 	let mut max_count = 0;
 	for (key, val) in mode_map.iter(){
 		if *val > max_count {
 			max_count = *val;
 			mode = **key
 		}
 	}
 	println!("The mode is {:?} ({:?} times)", mode, max_count);


}
