use std::fs;

/// Solution for day 1, part 1
pub fn compute_day_1(s: &str) -> u32 {
	let mut calories: Vec<u32> = vec![];
	let mut max: u32 = u32::MIN;

	for line in s.lines() {
		if line.is_empty() {
			let total = calories.iter().sum();
			if total > max {
				max = total;
			}
			calories.clear();
		} else {
			calories.push(line.parse::<u32>().unwrap());
		}
	}
	
	max
}

/// Solution for day 1, part 2
pub fn compute_day_1_pt2(s: &str) -> u32 {
	let mut current_elf_calories: Vec<u32> = vec![];
	let mut calories: Vec<u32> = vec![];
	let mut maxes: [u32; 3] = [u32::MIN; 3];

	for line in s.lines() {
		if line.is_empty() {
			let total = current_elf_calories.iter().sum();
			current_elf_calories.clear();
			calories.push(total);
		} else {
			current_elf_calories.push(line.parse::<u32>().unwrap());
		}
	}

	let calories_len: usize = calories.len();
	calories.sort();

	maxes[0] = calories[calories_len - 1];
	maxes[1] = calories[calories_len - 2];
	maxes[2] = calories[calories_len - 3];
	dbg!(maxes);

	maxes.iter().sum()
}

fn main() {
	let day1data = fs::read_to_string("src/day1.txt")
		.expect("day1.txt file does not exist/cannot be found.");
	
	let str_ref = &day1data;
	println!("{}", compute_day_1(str_ref));
	println!("{}", compute_day_1_pt2(str_ref));
}

