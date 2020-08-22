use crate::point::Point;

const EPSILON: f64 = 0.00001;

//LINE
pub struct Line {
	p1: Point,
	p2: Point,
	infinite: bool,
}

//Constructors
impl Line {
	//TODO Ensure any two points used to create a line ARE NOT the same
	//from 2 point objects
	pub fn new(p1: Point, p2: Point, infinite: bool) -> Self {
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
	pub fn from_coordinates(x1: f64, y1: f64, x2: f64, y2:f64, infinite: bool) -> Self {
		let p1 = Point::new(x1,y1);
		let p2 = Point::new(x2,y2);
		Line::new(p1, p2, infinite)
	}
}

// Methods
impl Line {
	pub fn length(&self) -> f64 {
		if self.infinite {
			return -1.0;
		}
		self.p1.distance(&self.p2)
	}

	//works for infinte lines and segments
	pub fn distance_to_point(&self, p: &Point) -> f64 {
		//pretty easy if infinite
		if !self.infinite {
			//check if its closer to an endpoint of the segment
			let dx = self.p2.x - self.p1.x;
			let dy = self.p2.y - self.p1.y;

			let d2x = self.p1.x - p.x;
			let d2y = self.p1.y - p.y;

			let dot = (dx*d2x + dy*d2y);
			if dot > EPSILON {
				return (d2x*d2x + d2y*d2y).sqrt();
			}

			let d3x = p.x - self.p2.x;
			let d3y = p.y - self.p2.y;

			let dot2 = (dx*d3x + dy*d3y);
			if dot2 > EPSILON {
				return (d3x*d3x + d3y*d3y).sqrt();
			}

		}
		let num = f64::abs((self.p2.y - self.p1.y)*p.x - (self.p2.x - self.p1.x)*p.y + self.p2.x*self.p1.y - self.p2.y*self.p1.x);
		let denom = self.p1.distance(&self.p2);	
		if f64::abs(denom) > EPSILON {
			return num/denom;
		}
		//basic error information
		return -1.0;

	}

	pub fn distance_to_line(&self, other: &Line) -> f64 {

		if self.is_parallel(other) {
			//all distances are the same so return this one arbitrarily
			return self.p1.distance(&other.p1);
		}

		if self.intersects(&other) {
			return 0.0;
		}

		//distance of 2 line segments is always min of the distance
		//from one endpoint to the other line 
		let d1 = self.distance_to_point(&other.p1);
		let d2 = self.distance_to_point(&other.p2);
		let d3 = other.distance_to_point(&self.p1);
		let d4 = other.distance_to_point(&self.p2);

		f64::min(d1,f64::min(d2,f64::min(d3,d4)))
	}

	//check if slopes are equal
	pub fn is_parallel(&self, other: &Line) -> bool {
		let a1 = self.p2.y - self.p1.y;
		let b1 = self.p1.x - self.p2.x;

		let a2 = other.p2.y - other.p1.y;
		let b2 = other.p1.x - other.p2.x;


		f64::abs(a1*b2 - a2*b1) < EPSILON
	}


	pub fn intersects(&self, other: &Line) -> bool {
		if self.infinite && other.infinite && !self.is_parallel(&other) {
			return true;
		}
		if self.infinite {
			let m = (self.p2.y - self.p1.y) / (self.p2.x - self.p1.x);
			let b = self.p2.y - m*self.p2.x;

			let d1 = other.p1.y - m*other.p1.x - b;
			let d2 = other.p2.y - m*other.p2.x - b;

			return (d1 < 0.0 && d2 > 0.0) || (d1 > 0.0 && d2 < 0.0); 

		}
		else if other.infinite {
			let m = (other.p2.y - other.p1.y) / (other.p2.x - other.p1.x);
			let b = other.p2.y - m*other.p2.x;

			let d1 = self.p1.y - m*self.p1.x - b;
			let d2 = self.p2.y - m*self.p2.x - b; 

			return (d1 < 0.0 && d2 > 0.0) || (d1 > 0.0 && d2 < 0.0); 
		}
		let o1 = self.p1.orientation(&self.p2, &other.p1);
		let o2 = self.p1.orientation(&self.p2, &other.p2);
		let o3 = other.p1.orientation(&other.p2, &self.p1);
		let o4 = other.p1.orientation(&other.p2, &self.p2);

		o1 != o2 && o3 != o4

	}

	//Intersection of line SEGMENTS 
	pub fn intersection(&self, other: &Line) -> Point {

		//do a parallel check. Would just call the function but these values are needed later
		let a1 = self.p2.y - self.p1.y;
		let b1 = self.p1.x - self.p2.x;
		let c1 = a1*self.p1.x + b1*self.p1.y;

		let a2 = other.p2.y - other.p1.y;
		let b2 = other.p1.x - other.p2.x;
		let c2 = a2*other.p1.x + b2*other.p1.y;

		let d = a1*b2 - a2*b1;
		if f64::abs(d) < EPSILON {
			//Throw an error here, lines were parallel
			return Point {
				x: f64::MAX,
				y: f64::MAX,
			}
		}
		Point {
			x: (b2*c1 - b1*c2)/d,
			y: (a1*c2 - a2*c1)/d,
		}
	}


}