fn divison(x: f64, y: f64) -> Option<f64> {
	if y = 0.0 {
		None
	} else {
		Some(x/y)
	}
}

fn main() {
	let res = divison(5.0, 7.0);
	match res {
		Some(x) -> println!("{}", x),
		None -> println!("cannot divide by 0"),
	}
}