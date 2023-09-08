fn main() {
    fizzbuzz_match(110);
}

fn fizzbuzz_if(n: i32) {
    (1..=n)
        .map(|i| -> String {
            let mut s = "".to_string();
            if i % 3 == 0 {
                s.push_str("fizz")
            }
            if i % 5 == 0 {
                s.push_str("buzz");
            }
            if s.eq("") {
                s.push_str(&i.to_string());
            }
            s
        })
        .for_each(|s| println!("{}", s));
}

fn fizzbuzz_match(n: i32) {
    (1..=n).for_each(|i| match (i % 3, i % 5, i % 7) {
        (0, 0, 0) => println!("fizzbuzzbar"),
        (0, 0, _) => println!("fizzbuzz"),
        (0, _, 0) => println!("fizzbar"),
        (_, 0, 0) => println!("buzzbar"),
        (0, _, _) => println!("fizz"),
        (_, 0, _) => println!("buzz"),
        (_, _, 0) => println!("bar"),
        (_, _, _) => println!("{}", i),
    })
}
