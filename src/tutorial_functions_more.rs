fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn main() {
    let x1 = five();
    let x2 = plus_one(x1);

    println!("The value of x is: {}", x2);
}
