use std::{num::ParseIntError, str::FromStr};

// #[derive(Debug, Eq, PartialEq)]
// struct PenKind;

#[derive(Debug, Eq, PartialEq)]
enum Direction {
	North,
	South,
	East,
	West,
}

#[derive(Debug, Eq, PartialEq)]
enum Command {
	PenDown,
	PenUp,
	PenSelect(usize),
	Move(Direction, isize),
}

#[derive(Debug, Eq, PartialEq)]
struct CommandParseError;

impl From<DirectionParseError> for CommandParseError {
	fn from(_: DirectionParseError) -> Self {
		CommandParseError
	}
}

impl From<ParseIntError> for CommandParseError {
	fn from(_: ParseIntError) -> Self {
		CommandParseError
	}
}

struct DirectionParseError;

impl FromStr for Direction {
	type Err = DirectionParseError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s {
			"N" => Ok(Self::North),
			"S" => Ok(Self::South),
			"E" => Ok(Self::East),
			"W" => Ok(Self::West),
			_ => Err(DirectionParseError),
		}
	}
}

impl FromStr for Command {
	type Err = CommandParseError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let command_str: Vec<&str> = s.split_ascii_whitespace().collect();

		match command_str.as_slice() {
			["U"] => Ok(Self::PenUp),
			["D"] => Ok(Self::PenDown),
			["P", n] => Ok(Self::PenSelect(n.parse()?)),
			[d @ ("N" | "S" | "W" | "E"), v] => Ok(Self::Move(d.parse()?, v.parse()?)),
			_ => Err(CommandParseError),
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use Direction::*;

	#[test]
	fn parsing_pen_select_works() {
		assert_eq!("P 4".parse::<Command>(), Ok(Command::PenSelect(4)));
	}

	#[test]
	fn parsing_direction_works() {
		assert_eq!("N 7".parse::<Command>(), Ok(Command::Move(North, 7)));
		assert_eq!("W -7".parse::<Command>(), Ok(Command::Move(West, -7)));
		assert_eq!("S 1".parse::<Command>(), Ok(Command::Move(South, 1)));
		assert_eq!("E 2".parse::<Command>(), Ok(Command::Move(East, 2)));
	}

	#[test]
	fn parsing_pen_down_works() {
		assert_eq!("D".parse::<Command>(), Ok(Command::PenDown));
	}

	fn into_commands(command_strs: &[&str]) -> Result<Vec<Command>, CommandParseError> {
		command_strs.into_iter().map(|line| line.parse()).collect()
	}

	#[test]
	fn parsing_works() {
		let commands = ["P 3", "D", "S 7", "W -9", "U", "N 4", "E 5"];

		let expected = vec![
			Command::PenSelect(3),
			Command::PenDown,
			Command::Move(South, 7),
			Command::Move(West, -9),
			Command::PenUp,
			Command::Move(North, 4),
			Command::Move(East, 5),
		];

		assert_eq!(into_commands(&commands), Ok(expected));
	}

	#[test]
	fn parsing_invalid_command_seq_works() {
		let commands = ["D", "E u"];
		assert_eq!(into_commands(&commands), Err(CommandParseError));
	}

	#[test]
	fn parsing_invalid_command_works() {
		assert_eq!("E".parse::<Command>(), Err(CommandParseError));
	}
}

// Example Domain Language Script
// D
// U
// P 2
// N 8
// S 1
// E 5
// W 6
