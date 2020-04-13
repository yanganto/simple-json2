extern crate alloc;
use alloc::{ vec::Vec };
use crate::parser::{Error, Position};
use core::fmt::Debug;

#[derive(Debug, Copy, Clone, Default, PartialEq, Eq)]
pub struct SimplePosition {
	pub index: u32,
	pub line: u32,
	pub column: u32,
}

impl SimplePosition {
	pub fn next(&self, c: char) -> Self {
		let new_line = c == '\n';
		Self {
			index: self.index + 1,
			line: if new_line { self.line + 1 } else { self.line },
			column: if new_line { 0 } else { self.column + 1 },
		}
	}
}

impl Position for SimplePosition {
	fn index(&self) -> u32 {
		self.index
	}

	fn line(&self) -> u32 {
		self.line
	}

	fn column(&self) -> u32 {
		self.column
	}
}

impl core::ops::Sub<Self> for SimplePosition {
	type Output = i32;

	fn sub(self, rhs: SimplePosition) -> Self::Output {
		if self.index > rhs.index {
			(self.index - rhs.index) as i32
		} else {
			-((rhs.index - self.index) as i32)
		}
	}
}

#[derive(Debug, PartialEq, Eq)]
pub struct SimpleError {
	pub reasons: Vec<(Option<SimplePosition>, &'static str)>,
}

impl Error for SimpleError {
	type Position = SimplePosition;

	fn reasons(&self) -> &[(Option<Self::Position>, &'static str)] {
		&self.reasons[..]
	}

	fn add_reason(self, position: Option<Self::Position>, reason: &'static str) -> Self {
		let mut reasons = self.reasons;
		reasons.push((position, reason));
		Self { reasons }
	}

	fn plain_str(reason: &'static str) -> Self {
		Self { reasons: vec![(None, reason)] }
	}
}
