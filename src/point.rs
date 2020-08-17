use crate::line::Line;

const EPSILON: f64 = 0.00001;
//POINT
pub struct Point {
	pub x: f64,
	pub y: f64,
}

//Constructor
impl Point {
	pub fn new(x: f64, y:f64) -> Self {
		Self {
			x,
			y, 
		}
	}
}

//Methods
impl Point {
	pub(crate) fn distance(&self, other: &Point) -> f64{
		((self.x - other.x)*(self.x-other.x) + (self.y - other.y)*(self.y - other.y)).sqrt()
	}

	pub(crate) fn distance_to_line(&self, other: &Line) -> f64 {
		other.distance_to_point(&self)
	}
}
