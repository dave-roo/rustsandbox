use std::{i8, i16, i32, i64, u8, u16, u32, u64, isize, usize, f32, f64};

use std::io::stdin;

fn main() {

	let mut vect1 = vec![1,2,3,4,5];
	let  rand_array = [1,2,3];
	println!("{}", rand_array[0]);
	println!("{}", rand_array.len());
	println!("Second 2 : {:?}", &rand_array[1..3]);

	for i in &vect1 {
		println!("Vect: {}", i);
	}

	vect1.push(6);
	vect1.pop();

    struct Circle {
        x: f64,
        y: f64,
        radius: f64, 
    }

        fn get_radius(circle: &Circle) -> f64 {
	        circle.radius
        }

        impl Circle {
	        pub fn get_x(&self) -> f64 {
	        self.x
        }
    }

}	