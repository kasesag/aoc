/* 
Day 1: https://adventofcode.com/2022/day/1

Q1: Find the Elf carrying the most Calories. How many total Calories is that Elf carrying?
Q2: Find the top three Elves carrying the most Calories. How many Calories are those Elves carrying in total?
*/

use std::fs::File;
use std::io::{BufReader, BufRead};

fn main() {
	let file = File::open("./input.txt").expect("Cannot find input file!");
	let reader = BufReader::new(&file);
	
	let mut elves: Vec<i32> = Vec::new();
	let mut calc: i32 = 0;

	for l in reader.lines() {
		if let Ok(line) = l {
			if line.len() != 0 {
				let n: i32 = line.parse().unwrap();
				calc+=n;
			} else {
				elves.push(calc);
				calc = 0;
				continue;
			}
		}
	}
	elves.push(calc);	
	elves.sort_by(|a, b| b.cmp(a));

	println!("A1: {}", elves[0]);
	println!("A2: {}", elves[0] + elves[1] + elves[2]);
}