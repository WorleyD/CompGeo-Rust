use crate::point::Point;
use std::vec::Vec;

const EPSILON: f64 = 0.00001;

//Struct
pub struct Polygon {
	points: Vec<Point>,
	//true: clockwise - false: counterclockwise
	orientation: bool,
}

//Constructors
impl Polygon {
	// This assumes: 
	//	1. There are 3+ points in the vector
	//	2. The first 3 points are not collinear
	// Should handle cases where that is not guaranteed
	pub fn from_vec(points: Vec<Point>) -> Self {
		let o = points[0].orientation(&points[1], &points[2]);

		Polygon {
			points,
			orientation: o==1,
		}
	}

}

//Methods
impl Polygon {
	pub fn area(&self) -> f64{
		let n = self.points.len();
		let mut xs = Vec::new();
		let mut ys = Vec::new();
		
		for p in &self.points {
			xs.push(p.x);
			ys.push(p.y);
		}

		let mut area: f64 = 0.0;

		for i in 0..n-1 {
			area += xs[i]*ys[i+1] - xs[i+1]*ys[i]
		}
		area += xs[n-1]*ys[0] - xs[0]*ys[n-1];

		//if points were counterclockwise area will be negative
		return f64::abs(area)/2.0
	}
}