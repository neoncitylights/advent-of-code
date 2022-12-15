use std::str::FromStr;
use std::fs;
use std::cmp::Ordering;

#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, PartialOrd)]
pub enum Shape {
	Rock = 1u8,
	Paper = 2u8,
	Scissors = 3u8,
}

impl Shape {
	// The score for a single round is the score for the shape you selected
	// - (1 for Rock, 2 for Paper, and 3 for Scissors) 
	// plus the score for the outcome of the round
	// - (0 if you lost, 3 if the round was a draw, and 6 if you won).
	fn get_outcome_score(&self, outcome: Ordering) -> u8 {
		let shape_score = *self as u8;
		match outcome {
			Ordering::Less => shape_score, // lost
			Ordering::Equal => shape_score + 3, // tie
			Ordering::Greater => shape_score + 6, // won
		}
	}

	fn get_player2_by_outcome(&self, outcome: Ordering) -> Self {
		match (&self, outcome) {
			(Shape::Rock, Ordering::Less) => Self::Scissors,
			(Shape::Rock, Ordering::Equal) => Self::Rock,
			(Shape::Rock, Ordering::Greater) => Self::Paper,
			(Shape::Paper, Ordering::Less) => Self::Rock,
			(Shape::Paper, Ordering::Equal) => Self::Paper,
			(Shape::Paper, Ordering::Greater) => Self::Scissors,
			(Shape::Scissors, Ordering::Less) => Self::Paper,
			(Shape::Scissors, Ordering::Equal) => Self::Scissors,
			(Shape::Scissors, Ordering::Greater) => Self::Rock,
		}
	}
}

impl TryFrom<char> for Shape {
	type Error = &'static str;

	fn try_from(value: char) -> Result<Self, Self::Error> {
			match value {
				'A' | 'X' => Ok(Self::Rock), 
				'B' | 'Y' => Ok(Self::Paper),
				'C' | 'Z' => Ok(Self::Scissors),
				_ => Err("Not a valid character to represent a shape"),
		}
	}
}

impl Ord for Shape {
	fn cmp(&self, other: &Self) -> Ordering {
		match (&self, other) {
			(Self::Paper, Self::Paper)
			| (Self::Scissors, Self::Scissors)
			| (Self::Rock, Self::Rock) => Ordering::Equal,

			(Self::Paper, Self::Rock)
			| (Self::Scissors, Self::Paper)
			| (Self::Rock, Self::Scissors) => Ordering::Greater,

			(Self::Rock, Self::Paper)
			| (Self::Paper, Self::Scissors)
			| (Self::Scissors, Self::Rock) => Ordering::Less,
		}
	}
}

pub fn day_of_code2_pt1(s: &str) -> u32 {
	let mut score: u32 = u32::MIN;
	for line in s.lines() {
		let split: Vec<&str> = line.split(' ').collect();
		
		let player1_char = char::from_str(split[0]).unwrap();
		let player2_char = char::from_str(split[1]).unwrap();

		let player1_shape = Shape::try_from(player1_char).unwrap();
		let player2_shape = Shape::try_from(player2_char).unwrap();
		
		let ord = player2_shape.cmp(&player1_shape);
		score += player2_shape.get_outcome_score(ord) as u32;
	}

	score
}

pub fn get_intended_outcome(c: char) -> Ordering {
	match c {
		'Z' => Ordering::Greater,
		'Y' => Ordering::Equal,
		'X' => Ordering::Less,
		_ => unreachable!()
	}
}

pub fn day_of_code2_pt2(s: &str) -> u32 {
	let mut score: u32 = u32::MIN;
	for line in s.lines() {
		let split: Vec<&str> = line.split(' ').collect();

		let first_char = char::from_str(split[0]).unwrap();
		let second_char = char::from_str(split[1]).unwrap();

		let player1_shape = Shape::try_from(first_char).unwrap();
		let intended_outcome = get_intended_outcome(second_char);
		let player_2_shape = player1_shape.get_player2_by_outcome(intended_outcome);

		score += player_2_shape.get_outcome_score(intended_outcome) as u32;
	}

	score
}

fn main() {
	let dataset = fs::read_to_string("src/day2.txt")
		.expect("day1.txt file does not exist/cannot be found.");

	println!("Day 2, part 1: {}", day_of_code2_pt1(&dataset));
	println!("Day 2, pat 2: {}", day_of_code2_pt2(&dataset));
}
