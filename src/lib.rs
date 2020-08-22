pub mod line;
pub mod point;
pub mod circle;
pub mod polygon;


#[cfg(test)]
mod tests {
	use super::point::Point;
	use super::line::Line;
	use super::circle::Circle;
	use super::polygon::Polygon;

	const EPSILON: f64 = 0.00001;

	//POINT TEST START
	#[test]
	fn point_dist_test() {
		let p1 = Point::new(1.0, 4.0);
		let p2 = Point::new(2.0, -5.0);
		let p3 = Point::new(1.0, 4.0);
		let d = p1.distance(&p2);
		let d2 = p1.distance(&p3);

		assert!(d - 9.055385 < EPSILON);
		assert_eq!(d2, 0.0);
	}

	#[test]
	fn collinear_test() {
		//this also tests triangleArea 
		let p1 = Point::new(1.0, 2.0);
		let p2 = Point::new(1.0, 3.0);
		let p3 = Point::new(1.0, 4.0);
		let p4 = Point::new(2.0, 2.0);

		assert!(p1.collinear(&p2, & p3));
		assert!(p1.collinear(&p3, & p2));
		assert_eq!(p1.collinear(&p2, &p4), false);
	}

	#[test]
	fn orientation_test() {
		let p1 = Point::new(1.0, 2.0);
		let p2 = Point::new(3.0, 5.0);
		let p3 = Point::new(4.0, 6.0);

		assert_eq!(p1.orientation(&p2, &p3), 1);
		assert_eq!(p3.orientation(&p2, &p1), 2);
	}
	//POINT TEST END


	//LINE TEST START
	#[test]
	fn line_pt_dist() {
		let p1 = Point::new(1.0, 9.0);
		let p4 = Point::new(12.0, 2.0);
		let p5 = Point::new(6.0, 4.0);

		let p2 = Point::new(3.0, 5.0);
		let p3 = Point::new(9.0, 0.0);

		let l1 = Line::new(p2, p3, false);
		assert_eq!(l1.distance_to_point(&p1), p2.distance(&p1));
		assert_eq!(l1.distance_to_point(&p4), p3.distance(&p4));
		assert!(l1.distance_to_point(&p5) - 1.1523319192613102 < EPSILON);
	}	
	//LINE TEST END


	//CIRCLE TEST START

	//CIRCLE TEST END


	//POLYGON TEST START

	//POLYGON TEST END
}