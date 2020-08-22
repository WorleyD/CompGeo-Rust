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
	fn line_pt_dist_test() {
		//Test a line segment
		let p1 = Point::new(1.0, 9.0);
		let p4 = Point::new(12.0, 2.0);
		let p5 = Point::new(6.0, 4.0);

		let p2 = Point::new(3.0, 5.0);
		let p3 = Point::new(9.0, 0.0);
		let p0 = Point::new(3.0,5.0);

		let l1 = Line::new(p2, p3, false);
		assert_eq!(l1.distance_to_point(&p1), p2.distance(&p1));
		assert_eq!(l1.distance_to_point(&p4), p3.distance(&p4));
		assert!(l1.distance_to_point(&p5) - 1.1523319192613102 < EPSILON);
		assert_eq!(l1.distance_to_point(&p0) < EPSILON, true);


		// Now test an infinite line
		let p6 = Point::new(2.0, 5.0);
		let p7 = Point::new(10.0, 6.0);
		let l2 = Line::new(p6, p7, true);

		let p8 = Point::new(1.0, 5.0);
		let p9 = Point::new(2.0, 6.0);
		let p10 = Point::new(13.0, 7.0);
		let p11 = Point::new(6.0, 4.0);

		assert!(l2.distance_to_point(&p8) - 0.124034734 < EPSILON);
		assert!(l2.distance_to_point(&p9)-  0.99227787 < EPSILON);
		assert!(l2.distance_to_point(&p10)- 0.6201736729< EPSILON);
		assert!(l2.distance_to_point(&p11)- 1.48841681507 < EPSILON);

	}	

	#[test] 
	fn line_line_dist_test() {

		let p1 = Point::new(1.0, 1.0);
		let p2 = Point::new(5.0, 5.0);
		let l1 = Line::new(p1,p2,false);

		let p3 = Point::new(2.0, 1.0);
		let p4 = Point::new(2.0, -5.0);	//vertical line segment
		let l2 = Line::new(p3,p4,false);

		let p5 = Point::new(4.0, 12.0);
		let p6 = Point::new(8.0, 5.0);
		let l3 = Line::new(p5,p6,true);

		let p7 = Point::new(5.0, 0.0);
		let p8 = Point::new(6.0, 0.0);	//infinite horizontal line
		let l4 = Line::new(p7,p8,true);

		assert_eq!(l1.distance_to_line(&l2), l1.distance_to_point(&p3));
		assert_eq!(l1.distance_to_line(&l3), l3.distance_to_point(&p2));
		assert_eq!(l1.distance_to_line(&l4), 1.0);

		assert_eq!(l2.distance_to_line(&l3), l3.distance_to_point(&p3));
		assert_eq!(l2.distance_to_line(&l4), 0.0);

		assert_eq!(l3.distance_to_line(&l4), 0.0);
	}

	#[test]
	fn parallel_test() {
		let p1 = Point::new(1.0,1.0);
		let p2 = Point::new(5.0,5.0);
		let l1 = Line::new(p1,p2, true);

		let p3 = Point::new(2.0,2.0);
		let p4 = Point::new(6.0,6.0);
		let l2 = Line::new(p3,p4, true);

		let p5 = Point::new(0.0, 5.0);
		let p6 = Point::new(0.0,10.0);
		let l3 = Line::new(p5,p6, false);

		let p7 = Point::new(1.0, 5.0);
		let p8 = Point::new(1.0, 10.0);
		let l4 = Line::new(p7,p8, false);

		let p9 = Point::new(1.0, 0.0);
		let p10 = Point::new(5.0, 0.0);
		let l5 = Line::new(p9,p10, true);

		let p11 = Point::new(1.0,1.0);
		let p12 = Point::new(5.0,1.0);
		let l6 = Line::new(p11,p12, false);

		assert!(l1.is_parallel(&l2));
		assert!(!l1.is_parallel(&l3));
		assert!(!l1.is_parallel(&l4));
		assert!(!l1.is_parallel(&l5));
		assert!(!l1.is_parallel(&l6));

		assert!(l2.is_parallel(&l1));
		assert!(!l2.is_parallel(&l3));
		assert!(!l2.is_parallel(&l4));
		assert!(!l2.is_parallel(&l5));
		assert!(!l2.is_parallel(&l6));

		assert!(!l3.is_parallel(&l1));
		assert!(!l3.is_parallel(&l2));
		assert!(l3.is_parallel(&l4));
		assert!(!l3.is_parallel(&l5));
		assert!(!l3.is_parallel(&l5));

		assert!(!l4.is_parallel(&l1));
		assert!(!l4.is_parallel(&l2));
		assert!(l4.is_parallel(&l3));
		assert!(!l4.is_parallel(&l5));
		assert!(!l4.is_parallel(&l6));

		assert!(!l5.is_parallel(&l1));
		assert!(!l5.is_parallel(&l2));
		assert!(!l5.is_parallel(&l3));
		assert!(!l5.is_parallel(&l4));
		assert!(l5.is_parallel(&l6));

		assert!(!l6.is_parallel(&l1));
		assert!(!l6.is_parallel(&l2));
		assert!(!l6.is_parallel(&l3));
		assert!(!l6.is_parallel(&l4));
		assert!(l6.is_parallel(&l5));

		assert!(l1.is_parallel(&l1));
		assert!(l2.is_parallel(&l2));
		assert!(l3.is_parallel(&l3));
		assert!(l4.is_parallel(&l4));
		assert!(l5.is_parallel(&l5));
		assert!(l6.is_parallel(&l6));

	}
	//LINE TEST END


	//CIRCLE TEST START

	//CIRCLE TEST END


	//POLYGON TEST START

	//POLYGON TEST END
}