pub mod line;
pub mod point;
pub mod circle;
pub mod polygon;


#[cfg(test)]
mod tests {
	use super::point::*;
	use super::line::*;
	use super::circle::*;
	use super::polygon::*;
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
	fn point_collinear_test() {
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
	fn point_orientation_test() {
		let p1 = Point::new(1.0, 2.0);
		let p2 = Point::new(3.0, 5.0);
		let p3 = Point::new(4.0, 6.0);

		assert_eq!(p1.orientation(&p2, &p3), 1);
		assert_eq!(p3.orientation(&p2, &p1), 2);
	}

	#[test]
	//not tested yet
	fn point_convex_hull_test() {
		let xs: [f64; 7] = [0.0, 0.0, 1.0, 2.0, 1.0, 1.0, 2.0];
		let ys: [f64; 7] = [0.0, 6.0, 7.0, 5.0, 2.0, 1.0, 0.0];

		let mut points: Vec<Point> = Vec::new();

		for i in 0..6 {
			let p = Point::new(xs[i], ys[i]);
			points.push(p);
		}

		let res = convex_hull(&points);
		for p in &res {
			println!("{}, {}", p.x, p.y);
		}
		//run cargo test -- --nocapture
		//to see stdout output during testing
		assert!(res.len() == 5);
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
	fn line_parallel_test() {
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

	#[test]
	fn line_intersects_test() {
		let p1 = Point::new(1.0, 1.0);
		let p2 = Point::new(5.0, 5.0);
		let l1 = Line::new(p1,p2,true);

		let p3 = Point::new(2.0, 1.0);
		let p4 = Point::new(2.0, -5.0);	//vertical line segment
		let l2 = Line::new(p3,p4,false);

		let p5 = Point::new(4.0, 12.0);
		let p6 = Point::new(8.0, 5.0);
		let l3 = Line::new(p5,p6,true);

		let p7 = Point::new(5.0, 0.0);
		let p8 = Point::new(6.0, 0.0);	//infinite horizontal line
		let l4 = Line::new(p7,p8,true);

		assert!(!l1.intersects(&l2));
		assert!(l1.intersects(&l3));
		assert!(l1.intersects(&l4));

		assert!(!l2.intersects(&l1));
		assert!(!l2.intersects(&l3));
		assert!(l2.intersects(&l4));

		assert!(l3.intersects(&l1));
		assert!(!l3.intersects(&l2));
		assert!(l3.intersects(&l4));

		assert!(l4.intersects(&l1));
		assert!(l4.intersects(&l2));
		assert!(l4.intersects(&l3));
		
	}

	#[test]
	//Not tested yet
	fn line_intersection_test() {
		assert!(false);
	}
	//LINE TEST END


	//CIRCLE TEST START
	#[test]
	fn circle_circumference_test() {
		let p1 = Point::new(0.0,0.0);
		let r:f64 = 6.0;
		let c = Circle::new(p1, r);
		assert!(c.circumference() - 37.69911 < EPSILON);
	}

	#[test]
	fn circle_area_test() {
		let p1 = Point::new(0.0,0.0);
		let r:f64 = 6.0;
		let c = Circle::new(p1, r);
		assert!(c.area() - 226.19467105 < EPSILON);
	}

	#[test]
	fn circle_intersects_circle() {
		let p1 = Point::new(0.0,0.0);
		let r1:f64 = 6.0;
		let c1 = Circle::new(p1, r1);

		let p2 = Point::new(0.0,0.0);
		let r2:f64 = 6.0;
		let c2 = Circle::new(p2, r2);

		let p3 = Point::new(10.0,10.0);
		let r3:f64 = 3.0;
		let c3 = Circle::new(p3, r3);

		let p4 = Point::new(4.0,4.0);
		let r4:f64 = 7.0;
		let c4 = Circle::new(p4, r4);

		assert!(c1.intersects_circle(&c2));
		assert!(!c1.intersects_circle(&c3));
		assert!(c1.intersects_circle(&c4));
	}

	#[test]
	fn circle_distance_from_circle_test() {
		let p1 = Point::new(0.0,0.0);
		let r1:f64 = 6.0;
		let c1 = Circle::new(p1, r1);

		let p2 = Point::new(10.0,10.0);
		let r2:f64 = 9.0;
		let c2 = Circle::new(p2, r2);

		let p3 = Point::new(-10.0,-10.0);
		let r3:f64 = 3.0;
		let c3 = Circle::new(p3, r3);

		assert_eq!(c1.distance_from_circle(&c2), 0.0);
		assert_eq!(c1.distance_from_circle(&c3), p1.distance(&p3) - r1 - r3);

	}

	#[test] 
	fn circle_distance_from_point() {
		let p1 = Point::new(0.0,0.0);
		let r1:f64 = 6.0;
		let c1 = Circle::new(p1, r1);

		let p2 = Point::new(2.0,1.0);

		let p3 = Point::new(-10.0,-10.0);


		assert_eq!(c1.distance_from_point(&p2), 0.0);
		assert_eq!(c1.distance_from_point(&p3), p1.distance(&p3) - r1);
	}

	#[test]
	fn circle_contains_circle_test() {
		let p1 = Point::new(0.0,0.0);
		let r1:f64 = 6.0;
		let c1 = Circle::new(p1, r1);

		let p2 = Point::new(1.0,1.0);
		let r2:f64 = 2.0;
		let c2 = Circle::new(p2, r2);

		let p3 = Point::new(6.0,0.0);
		let r3:f64 = 3.0;
		let c3 = Circle::new(p3, r3);

		let p4= Point::new(10.0,10.0);
		let r4:f64 = 2.0;
		let c4 = Circle::new(p4, r4);

		assert!(c1.contains_circle(&c2));
		assert!(!c1.contains_circle(&c3));
		assert!(!c1.contains_circle(&c4));
	}

	#[test]
	fn circle_arc_length_test() {
		let p1 = Point::new(0.0,0.0);
		let r1:f64 = 6.0;
		let c1 = Circle::new(p1, r1);

		let p2 = Point::new(-6.0, 0.0);
		let p3 = Point::new(6.0, 0.0);

		let p4 = Point::new(0.0, 6.0);
		let p5 = Point::new(0.0, -6.0);

		let c2 = c1.circumference()/2.0;
		let c4 = c2/2.0;

		assert!(c1.arc_length(&p3, &p2) - c2 < EPSILON);
		assert!(c1.arc_length(&p5, &p4) - c2 < EPSILON);

		assert!(c1.arc_length(&p5, &p3) - c4 < EPSILON);
		assert!(c1.arc_length(&p3, &p4) - c4 < EPSILON);
	}
	//CIRCLE TEST END


	//POLYGON TEST START
	#[test]
	fn polygon_area_test() {
		// This is a concave, regular polygon
		let xs: [f64; 6] = [0.0, 0.0, 1.0, 2.0, 1.0, 2.0];
		let ys: [f64; 6] = [0.0, 6.0, 7.0, 5.0, 2.0, 0.0];

		let mut points: Vec<Point> = Vec::new();

		for i in 0..6 {
			let p = Point::new(xs[i], ys[i]);
			points.push(p);
		}
		let poly = Polygon::from_vec(points);
		assert_eq!(poly.area(), 10.0);

	}
	//POLYGON TEST END
}