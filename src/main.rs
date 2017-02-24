extern crate rand;
use std::io::{self, Read};
use rand::Rng;
use rand::distributions::{IndependentSample, Range};

fn factory_1994(rand: u32) -> String{
	let s : String = format!("{}", rand);
	s
	
	
}

fn main() {
	let mut rng = rand::thread_rng();
	let mut range = Range::new(1, 101);
	let read = io::stdin();
	let mut input = String::new();
	println!("Enter number of itterations: ");
	read.read_line(&mut input);
	let mut random;
	let mut test;
	let mut itt = input.trim().parse::<i32>().expect("invalid input");
	while(itt != 0){
		random = range.ind_sample(&mut rng);
		test = factory_1994(random);
		println!("{}", test);
		itt -= 1;
	}






}