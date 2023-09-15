// extern crate bar; // May be required for Rust 2015 edition or earlier

fn main() {
    bar::public_function();

    // Error! `private_function` is private
    // bar::private_function();

    bar::indirect_access();
}
