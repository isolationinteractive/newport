use crate::Vector2;

use std::{
	convert::From,
	ops::RangeInclusive,
};

use serde::{
	Deserialize,
	Serialize,
};

#[derive(Copy, Clone, Default, Debug, Serialize, Deserialize, PartialEq)]
pub struct Rect {
	pub min: Vector2,
	pub max: Vector2,
}

impl Rect {
	pub const INFINITY: Rect = Rect {
		min: Vector2 {
			x: -f32::INFINITY,
			y: -f32::INFINITY,
		},
		max: Vector2 {
			x: f32::INFINITY,
			y: f32::INFINITY,
		},
	};

	pub const ZERO: Rect = Rect {
		min: Vector2::ZERO,
		max: Vector2::ZERO,
	};

	pub fn from_min_max(min: impl Into<Vector2>, max: impl Into<Vector2>) -> Self {
		Self {
			min: min.into(),
			max: max.into(),
		}
	}

	pub fn from_center(pos: Vector2, size: Vector2) -> Self {
		let min = pos - size / 2.0;
		let max = pos + size / 2.0;
		Self { min, max }
	}

	pub fn from_ranges(x: RangeInclusive<f32>, y: RangeInclusive<f32>) -> Rect {
		(
			Vector2::new(*x.start(), *y.start()),
			Vector2::new(*x.end(), *y.end()),
		)
			.into()
	}

	pub fn width(self) -> f32 {
		self.max.x - self.min.x
	}

	pub fn height(self) -> f32 {
		self.max.y - self.min.y
	}

	pub fn x_range(self) -> RangeInclusive<f32> {
		self.min.x..=self.max.x
	}

	pub fn y_range(self) -> RangeInclusive<f32> {
		self.min.y..=self.max.y
	}

	pub fn size(self) -> Vector2 {
		Vector2::new(self.width(), self.height())
	}

	pub fn center(self) -> Vector2 {
		let x = self.min.x + self.width() / 2.0;
		let y = self.min.y + self.height() / 2.0;
		Vector2::new(x, y)
	}

	pub fn bottom_left(self) -> Vector2 {
		self.min
	}

	pub fn top_right(self) -> Vector2 {
		self.max
	}

	pub fn bottom_right(self) -> Vector2 {
		(self.max.x, self.min.y).into()
	}

	pub fn top_left(self) -> Vector2 {
		(self.min.x, self.max.y).into()
	}

	pub fn point_overlap(self, point: Vector2) -> bool {
		self.min.x <= point.x
			&& self.max.x >= point.x
			&& self.min.y <= point.y
			&& self.max.y >= point.y
	}

	pub fn split_top(&mut self, size: f32) -> Rect {
		let max = self.max;

		self.max.y -= size;

		let min = Vector2::new(self.min.x, self.max.y);

		(min, max).into()
	}

	pub fn split_bottom(&mut self, size: f32) -> Rect {
		let min = self.min;

		self.min.y += size;

		let max = Vector2::new(self.max.x, self.min.y);

		(min, max).into()
	}

	pub fn split_left(&mut self, size: f32) -> Rect {
		let min = self.min;

		self.min.x += size;

		let max = Vector2::new(self.min.x, self.max.y);

		(min, max).into()
	}

	pub fn split_right(&mut self, size: f32) -> Rect {
		let max = self.max;

		self.max.x -= size;

		let min = Vector2::new(self.max.x, self.min.y);

		(min, max).into()
	}

	pub fn intersect(self, other: Self) -> Self {
		let min = (self.min.x.max(other.min.x), self.min.y.max(other.min.y)).into();
		let max = (self.max.x.min(other.max.x), self.max.y.min(other.max.y)).into();
		(min, max).into()
	}

	pub fn overlaps(self, other: Self) -> bool {
		self.min.x <= other.max.x
			&& other.min.x <= self.max.x
			&& self.min.y <= other.max.y
			&& other.min.y <= self.max.y
	}

	pub fn translate(self, amount: Vector2) -> Self {
		Self {
			min: self.min + amount,
			max: self.max + amount,
		}
	}

	pub fn expand_to_include_x(&mut self, x: f32) {
		self.min.x = self.min.x.min(x);
		self.max.x = self.max.x.max(x);
	}

	pub fn expand_to_include_y(&mut self, y: f32) {
		self.min.y = self.min.y.min(y);
		self.max.y = self.max.y.max(y);
	}

	pub fn expand(self, other: Self) -> Self {
		let min = (self.min.x.min(other.min.x), self.min.y.min(other.min.y)).into();
		let max = (self.max.x.max(other.max.x), self.max.y.max(other.max.y)).into();
		(min, max).into()
	}

	pub fn shrink(self, amount: f32) -> Self {
		(self.min + amount, self.max - amount).into()
	}

	pub fn left(self) -> f32 {
		self.min.x
	}

	pub fn right(self) -> f32 {
		self.max.x
	}

	pub fn top(self) -> f32 {
		self.max.y
	}

	pub fn bottom(self) -> f32 {
		self.min.y
	}
}

impl From<(Vector2, Vector2)> for Rect {
	fn from(min_max: (Vector2, Vector2)) -> Self {
		let (min, max) = min_max;
		Self { min, max }
	}
}

impl From<(f32, f32, f32, f32)> for Rect {
	fn from(rect: (f32, f32, f32, f32)) -> Self {
		let (x0, y0, x1, y1) = rect;
		Self {
			min: Vector2::new(x0, y0),
			max: Vector2::new(x1, y1),
		}
	}
}