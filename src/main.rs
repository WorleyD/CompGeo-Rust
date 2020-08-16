use std::num;

const EPSILON: f64 = 0.00001;
//POINT
struct Point {
	x: f64,
	y: f64,
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
	fn distance(&self, other: &Point) -> f64{
		((self.x - other.x)*(self.x-other.x) + (self.y - other.y)*(self.y - other.y)).sqrt()
	}

	fn distance_to_line(&self, other: &Line) -> f64 {
		other.distance_to_point(&self)
	}
}


//LINE
struct Line {
	p1: Point,
	p2: Point,
	infinite: bool,
}

//Constructors
impl Line {
	//TODO Ensure any two points used to create a line ARE NOT the same
	//from 2 point objects
	pub fn new(p1: Point, p2: Point, infinite:bool) -> Self {
		Self {
			p1,
			p2,
			infinite,
		}
	}

	//from y = mx + b equation (TODO how to handle vertical lines?)
	pub fn from_equation(m:f64, b:f64, p1:Point) -> Self {
		let p2x = p1.x + 1.0;
		let p2y = m*p2x + b;
		let p = Point::new(p2x, p2y);
		Self {
			p1,
			p2: p,
			infinite: true,
		}
	}

	//from 4 floats 
	pub fn from_coordinates(x1: f64, y1: f64, x2: f64, y2:f64, infinite:bool) -> Self {
		let p1 = Point::new(x1,y1);
		let p2 = Point::new(x2,y2);
		Line::new(p1, p2, infinite)
	}
}

// Methods
impl Line {
	fn length(&self) -> f64 {
		self.p1.distance(&self.p2)
	}

	fn distance_to_point(&self, p: &Point) -> f64 {
		let num = (self.p2.y - self.p1.y)*p.x - (self.p2.x - self.p1.x)*p.y + self.p2.x*self.p1.y - self.p2.y*self.p1.x;
		let denom = self.length();
		if denom < EPSILON {
			return num/denom;
		}
		//basic error information
		-1.0
	}

	fn distance_to_line(&self, other: &Line) -> f64 {
		if self.infinite || other.infinite {
			if self.is_parallel(other) {
				//all distances are the same so return this one
				return self.p1.distance(&other.p1);
			}
			//if theyre infinite and not parallel they intersect, so dist is 0
			return 0.0;
		}
		let d1 = self.distance_to_point(&other.p1);
		let d2 = self.distance_to_point(&other.p2);
		let d3 = other.distance_to_point(&self.p1);
		let d4 = other.distance_to_point(&self.p2);

		f64::min(d1,f64::min(d2,f64::min(d3,d4)))
	}

	//check if slopes are equal
	fn is_parallel(&self, other: &Line) -> bool {
		let m1 = (self.p2.x - self.p1.x) / (self.p2.y - self.p1.y);
		let m2 = (other.p2.x - other.p1.x) / (other.p2.y - other.p1.y);
		return abs(m2 - m2) < EPSILON;
	}
}

fn main() {
    println!("Hello, world!");
}
