pub fn public_function() {
    println!("called bar's `public_function()`");
}

fn private_function() {
    println!("called bar's `private_function()`");
}

pub fn indirect_access() {
    print!("called bar's `indirect_access()`, that\n> ");

    private_function();
}
