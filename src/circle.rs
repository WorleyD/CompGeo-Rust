use crate::point::Point;

const EPSILON: f64 = 0.00001;
const PI: f64 = 3.141592653589793238462;
//Struct
pub struct Circle {
	center: Point,
	radius: f64,
}

//Constructor
impl Circle {
	pub fn new(center: Point, radius: f64) -> Self {
		if radius > 0.0 { 
			return Circle {
				center,
				radius,
			}
		}
		Circle {
			center, 
			radius: 0.0,
		}
	}
}

//Methods
impl Circle {
	pub(crate) fn circumference(&self) -> f64 {
		2.0*PI*self.radius
	}

	pub(crate) fn area(&self) -> f64 {
		2.0*PI*self.radius*self.radius
	}

	pub(crate) fn intersects_circle(&self, other: &Circle) -> bool {
		self.center.distance(&other.center) < self.radius + other.radius 
	}

	pub(crate) fn distance_from_circle(&self, other: &Circle) -> f64 {
		if self.intersects_circle(other) {
			return 0.0;
		}
		self.center.distance(&other.center) - self.radius + other.radius 
	}

	pub(crate) fn distance_from_point(&self, p:&Point) -> f64 {
		let d = self.center.distance(&p) - self.radius;
		if d < 0.0 {
			return 0.0;
		}
		d
	}

	//p1 and p2 are points that lie on the circle
	pub(crate) fn arc_length(&self, p1: &Point, p2: &Point) ->f64 {
		let c = self.circumference();
		let d1 = self.distance_from_point(&p1);
		let d2 = self.distance_from_point(&p2);
		let d3 = p1.distance(&p2);

		let n = (d1*d1) + (d2*d2) - (d3*d3);
		let d = 2.0*d1*d2;

		let angle = (n/d).acos();
		let length = angle/(2.0*PI);
		c*length
	}

}

