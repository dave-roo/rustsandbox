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

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index = index + 1;
    }

    //or

        for element in a.iter() {
        println!("the value is: {}", element);
    }

}
