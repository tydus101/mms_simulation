extern crate rand;
use std::io::{self, Read};
use rand::Rng;
use rand::distributions::{IndependentSample, Range};

fn factory_1994(rand: u32) -> String{
	let mut s = String::new();
	match rand {
		1...31 => s = "Brown".to_string(),
		31...51 => s = "Yellow".to_string(),
		51...71 => s = "Red".to_string(),
		71...81 => s = "Orange".to_string(),
		81...91 => s = "Tan".to_string(),
		91...101 => s = "Green".to_string(),
		_ => s = "Invalid".to_string(),
	}
	s	
}

fn factory_1996(rand: u32) -> String{
        let mut s = String::new();
        match rand {
                1...25 => s = "Blue".to_string(),
                25...45 => s = "Green".to_string(),
                45...61 => s = "Orange".to_string(),
                61...75 => s = "Yellow".to_string(),
                75...88 => s = "Red".to_string(),
                88...101 => s = "Brown".to_string(),
                _ => s = "Invalid".to_string(),
        }
        s
}

fn main() {
	let mut rng = rand::thread_rng(); //New Random number gen
	let mut range = Range::new(1, 101); //Define range for random number
	let read = io::stdin(); 
	let mut input = String::new(); //line input string

	println!("Enter number of itterations: ");
	read.read_line(&mut input);

	let mut random; //random number
	let mut old; //old M and M
	let mut new; //new M and m
	//parse number of itterations from stdin
	let mut itt = input.trim().parse::<i32>().expect("invalid input");
	let mut old_num = 0;
	let mut new_num = 0;
	while(itt != 0){
		random = range.ind_sample(&mut rng);
		old = factory_1994(random);
		random = range.ind_sample(&mut rng);
		new = factory_1996(random);
		if(old == "Yellow"){old_num += 1;}
		if(new == "Yellow"){new_num += 1;}
		itt -= 1;
	}
	println!("Old Yellows: {}", old_num);
        println!("New Yellows: {}", new_num);


}
