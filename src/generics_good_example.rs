struct Val {
    val: f64
}

struct GenVal<T>{
    gen_val: T
}

// impl of Val
impl Val {
    fn value(&self) -> &f64 { &self.val }
}

// impl of GenVal for a generic type `T`
impl <T> GenVal<T> {
    fn value(&self) -> &T { &self.gen_val }
}



fn largest<T>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

struct Point<T> {
    x: T,
    y: T,
}

fn main() {

    let x = Val { val: 3.0 };
    let y = GenVal { gen_val: 3i32 };
    let number_list = vec![34, 50, 25, 100, 65];

    //let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    //let result = largest(&char_list);
    println!("The largest char is {}", result);
	
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
//  let wontwork = Point { x: 1, y: 4.0 };

    println!("{}, {}", x.value(), y.value());
}