/* 
Day 3: https://adventofcode.com/2022/day/3

Q1: Find the item type that appears in both compartments of each rucksack. What is the sum of the priorities of those item types?
Q2: Find the item type that corresponds to the badges of each three-Elf group. What is the sum of the priorities of those item types?
*/

use std::fs;

// 65-90 - range from A to Z (-64)
// 97-122 - range from a to z (-96)

fn decode_ascii(c: char) -> i32 {
	let res: i32;
	let num: u32 = c.into();

	if c.is_lowercase() {
		res = (num as i32) - 96;
	} else {
		res = (num as i32) - 38;
	}

	res
}

pub struct Rucksacks {
	c: Vec<String>,
}

impl Rucksacks {
	pub fn part1(&self) -> i32 {
		let mut sum: i32 = 0;
		
		for i in &self.c {
			let split = i.split_at(i.len()/2);
			let f: Vec<String> = vec![split.0.to_string(), split.1.to_string()];

			let a: Vec<char> = f[0].chars().collect();

			for y in a {
				let item: i32;
				if f[1].to_string().contains(y) {
					item = decode_ascii(y);
					sum+=item;
					break;
				}
			}
		}


		sum
	}

	pub fn part2(&self) -> i32 {
		let mut f: Vec<String> = Vec::new();

		let mut sum: i32 = 0;

		let mut i = 0;
		while i < self.c.len() {
			f.push(self.c[i].to_string());
			if (i+1) % 3 == 0 {
				let a: Vec<char> = f[0].chars().collect();
				
				for y in a {
					let item: i32;
					if f[1].to_string().contains(y) && f[2].to_string().contains(y) {
						item = decode_ascii(y);
						sum+=item;
						break;
					}
				}
				
				f = Vec::new();
			}
			i += 1;
		}
		sum
	}
}


fn main() {
	let file: Vec<String> = fs::read_to_string("./input.txt")
		.expect("error")
		.split("\n")
		.filter(|x| !x.is_empty())
		.map(|x| x.to_string())
		.collect();
	
	let r = Rucksacks {c: file};
	
	println!("A1: {}", r.part1());
	println!("A2: {}", r.part2());
}

#[cfg(test)]
mod tests {
	#[test]
	fn ascii() {
		use crate::decode_ascii;

		let chars: Vec<char> = vec!['p', 'L', 'P', 'v', 't', 's'];
		let results: Vec<i32> = vec![16, 38, 42, 22, 20, 19];

		let mut i = 0;
		while i < chars.len() {
			assert_eq!(decode_ascii(chars[i]), results[i]);
			i += 1;
		}
	}
}	