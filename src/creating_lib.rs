pub fn public_function() {
    println!("called `public_function()`");
}

fn private_function() {
    println!("called private function `private_function()`");
}

pub fn indirect_access() {
    print!("called `indirect_access()`, that\n> ");

    private_function();
}

//$ rustc --crate-type=lib rary.rs
//$ ls lib*

//output-> library.rlib