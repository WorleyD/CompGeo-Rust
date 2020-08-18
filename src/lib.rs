pub mod line;
pub mod point;
pub mod circle;
pub mod polygon;


#[cfg(test)]
mod tests {
	use super::point;
	use super::line;
	use super::circle;
	use super::polygon;

	const EPSILON: f64 = 0.00001;

	#[test]
	fn point_dist_test() {
		let p1 = point::Point::new(1.0, 4.0);
		let p2 = point::Point::new(2.0, -5.0);
		let p3 = point::Point::new(1.0, 4.0);
		let d = p1.distance(&p2);
		let d2 = p1.distance(&p3);

		assert!(d - 9.055385 < EPSILON);
		assert_eq!(d2, 0.0);

	}
}