// Generate prime numbers fast.

use std::time::SystemTime;

const REQUESTED_NUMBER_CEIL : usize = 1000000;

fn main()
{
	let now = SystemTime::now();

	// Index is number, value is primeness.
	let is_prime = |i:usize| -> bool { 
		if i < 2 { return false; }
		for j in 2..i { if i % j == 0 { return false; } };
		return true;
	};

	let mut output : [bool; REQUESTED_NUMBER_CEIL] = [true; REQUESTED_NUMBER_CEIL];
	output[0] = false; 
	output[1] = false;

	// eratosthenes sieve:
	let mut index: usize = 2;
	let mut quad: usize = index*index;
	while quad < REQUESTED_NUMBER_CEIL
	{
		if is_prime(index) {
			let mut also_quad = index*index;
			while also_quad < REQUESTED_NUMBER_CEIL
			{
				output[also_quad] = false;
				also_quad += index;
			}
		}
		quad = index * index;
		index += 1;
	}

	println!("{}", now.elapsed().unwrap().as_micros());
	
	//for i in output.iter().enumerate().filter(|i| *i.1) { print!("{} {}, ", i.0, i.1); }; 
	return;
}