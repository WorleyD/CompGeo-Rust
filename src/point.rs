use crate::line::Line;
use std::vec::Vec;

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

	pub(crate) fn collinear(&self, p1: &Point, p2: &Point) -> f64 {
		0.5*(self.x - p1.x)*(p1.y - p2.y) - (p1.x-p2.x)*(self.y-p1.y)
	}

	//helper function to find orientation of 3 points
	pub(crate) fn orientation(&self, p1: &Point, p2: &Point) -> i32 {
		let o = self.collinear(p1, p2);
		if f64::abs(o) < EPSILON {
			return 0;
		}

		return if o > 0.0 {1} else {2}
	}
}

// Returns a vector of indexes for points in convex hull
// TODO implement Copy for Points so a vec of points can be returned
pub fn convex_hull(points: &Vec<Point>) -> Vec<usize>{
	let n = points.len();

	if n < 3 {
		return vec![0,1,2]
	}

	let mut hull = Vec::new();

	let l = leftmost_index(points);

	let mut p = l;
	let mut q = 0;

	loop {
		hull.push(p);
		q = (p+1)%n;

		for i in 0..n {
			if points[p].orientation(&points[i], &points[q]) == 2{
				q = i;
			}
		}

		p = q;

		if p == l {
			break;
		}
	}

	return hull
}

//Helper function for convex hull
pub fn leftmost_index(points: &Vec<Point>) -> usize {
	let mut m = 0;
	for i in 1..points.len() {
		if points[i].x < points[m].x {
			m = i
		}
		else if points[i].x == points[m].x {
			if points[i].y > points[m].y {
				m = i
			}
		}
	}
	m
}