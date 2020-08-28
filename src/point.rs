use crate::line::Line;
use std::vec::Vec;

const EPSILON: f64 = 0.00001;

//POINT
#[derive(Copy, Clone)]
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
	pub fn equals(&self, other: &Point) -> bool {
		self.x == other.x && self.y == other.y
	}

	pub fn distance(&self, other: &Point) -> f64{
		((self.x - other.x)*(self.x-other.x) + (self.y - other.y)*(self.y - other.y)).sqrt()
	}

	pub fn distance_squared(&self, other: &Point) -> f64{
		(self.x - other.x)*(self.x-other.x) + (self.y - other.y)*(self.y - other.y)
	}

	pub fn distance_to_line(&self, other: &Line) -> f64 {
		other.distance_to_point(&self)
	}

	pub fn collinear(&self, p1: &Point, p2: &Point) -> bool {
		self.triangle_area(p1, p2) < EPSILON
	}

	pub(crate) fn triangle_area(&self, p1: &Point, p2: &Point) -> f64 {
		f64::abs(0.5*(self.x - p1.x)*(p1.y - p2.y) - (p1.x-p2.x)*(self.y-p1.y))
	}

	//helper function to find orientation of 3 points (collinear, clockwise, or counterclockwise)
	pub fn orientation(&self, p1: &Point, p2: &Point) -> i32 {
		let o = (p1.y - self.y)*(p2.x - p1.x) - (p1.x - self.x)*(p2.y - p1.y);
		
		if f64::abs(o) < EPSILON {
			return 0;
		}

		return if o > 0.0 {1} else {2}
	}

	pub fn on_line(&self, line:&Line) -> bool {
		let l1 = Line::new(*self, line.p1, false);
		let l2 = Line::new(*self, line.p2, false);
		let p1 = Point::new(&self.x - line.p1.x, &self.y - line.p1.y);
		let p2 = Point::new(&self.x - line.p2.x, &self.y - line.p2.y);
		l1.is_parallel(&l2) && p1.x*p2.y + p2.x*p1.y < 0.0   
	} 
}

// Returns a vector of points in convex hull
// This is an implementation of Jarvis's Algorithm, 
// Will be replaced with something quicker soon 
// Currently this is O(m*n) where n is # of points and m is # of points in the hull
pub fn convex_hull(points: &Vec<Point>) -> Vec<Point>{
	let n = points.len();

	if n <= 3 {
		return points.to_vec();
	}

	let mut hull = Vec::new();

	let l = leftmost_index(points);

	let mut p = l;
	let mut q = 0;

	loop {
		hull.push(points[p]);
		q = (p+1)%n;

		for i in 0..n {
			if points[p].orientation(&points[i], &points[q]) == 2 {
				q = i;
			}
		}

		p = q;

		if p == l {
			break;
		}
	}
	hull
}


//Helper function for convex hull
pub(crate) fn leftmost_index(points: &Vec<Point>) -> usize {
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



pub(crate) fn closest_brute_force(points: Vec<Point>, n:usize) -> (Point, Point, f64){
	let mut min = f64::MAX;
	let mut p1 = points[0];
	let mut p2 = points[1];

	//check all points against all other points and return minimum
	for i in 0..n {
		for j in i+1..n {
			let mut d = points[i].distance_squared(&points[j]); 
			if d < min {
				min = d;
				p1 = points[i];
				p2 = points[j];
			}
		}
	}
	(p1, p2, min)
}

pub(crate) fn closest_strip(points: Vec<Point>, p1: &Point, p2:&Point, n:usize, d:f64) -> (Point, Point, f64) {
	let mut min = d;
	let mut p1r = *p1;
	let mut p2r = *p2;
	let mut strip = points.clone();
	//sort by y value
	strip.sort_unstable_by(|a,b| a.y.partial_cmp(&b.y).unwrap());

	//due to sorting and the passed in d acting as an upper bound
	//this loop loks to be O(n^2) but is actually O(n) 
	//as it only runs up to 6n times. 
	for i in 0..n {
		let mut j = i+1;
		while j < n && strip[j].y - strip[i].y < min {
			min = strip[i].distance_squared(&strip[j]);
			p1r = strip[i];
			p2r = strip[j];
			j+=1;
		}

	}
	(p1r, p2r, min)
}

pub(crate) fn closest_util(points: Vec<Point>, n:usize) -> (Point, Point, f64) { 
	//base case
	if n <= 3 {
		return closest_brute_force(points, n);
	}

	//find mid point
	let mid:usize = n/2;
	let p = points[mid];

	//split the left and right side recursively, taking min for each side
	let (pl1,pl2,dl) = closest_util(points[0..mid].to_vec(), mid);
	let (pr1, pr2, dr) = closest_util(points[mid..n].to_vec(), n-mid);

	//take lower half
	let (p1,p2,d) = if dl < dr {(pl1,pl2,dl)} else {(pr1, pr2, dr)};

	//collect eligible points close to the middle line
	let mut strip = Vec::new();
	for i in 0..n {
		if f64::abs(points[i].x - p.x) < d {
			strip.push(points[i]);
		}
	}

	let l = strip.len();
	//get minimum around this middle strip
	let (ps1,ps2, d3) = closest_strip(strip, &p1, &p2, l, d);

	//return minimum of lower side and middle strip 
	if d <= d3 {
		return (p1,p2,d);
	}
	(ps1,ps2,d3)
}

pub fn closest_pair(points: &Vec<Point>) -> (Point, Point, f64) {
	//sort vector by x value
	let mut sortable = points.clone();
	sortable.sort_unstable_by(|a,b| a.x.partial_cmp(&b.x).unwrap());

	//get points and distance
	let (p1,p2,d) = closest_util(sortable, points.len());

	//all distance calculations within the algorithm use distance_squared for more speed/precision
	//so take sqrt here on the returned minimum
	(p1,p2,f64::sqrt(d))
}
