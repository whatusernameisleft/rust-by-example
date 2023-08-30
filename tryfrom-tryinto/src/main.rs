use std::convert::TryFrom;
use std::convert::TryInto;

#[derive(Debug, PartialEq)]
struct EvenNumber(i32);

impl TryFrom<i32> for EvenNumber {
    type Error = String;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value % 2 == 0 {
            Ok(EvenNumber(value))
        } else {
            Err(format!("amogus, {} is not even", value))
        }
    }
}

fn main() {
    let eight = EvenNumber::try_from(8);
    let five = EvenNumber::try_from(5);

    match eight {
        Ok(even_number) => println!("{} is even.", even_number.0),
        Err(e) => println!("error: {}", e),
    }

    match five {
        Ok(even_number) => println!("{:?} is even.", even_number),
        Err(e) => println!("error: {}", e),
    }

    let eight_res: Result<EvenNumber, String> = (8i32).try_into();
    let five_res: Result<EvenNumber, String> = (5i32).try_into();

    match eight_res {
        Ok(even_number) => println!("{:?} is even.", even_number),
        Err(e) => println!("error: {}", e),
    }

    match five_res {
        Ok(even_number) => println!("{:?} is even.", even_number),
        Err(e) => println!("error: {}", e),
    }

    // TryFrom

    // assert_eq!(EvenNumber::try_from(8), Ok(EvenNumber(8)));
    // assert_eq!(EvenNumber::try_from(5), Err("amogus"));

    // TryInto

    // let result: Result<EvenNumber, &str> = (8i32).try_into();
    // assert_eq!(result, Ok(EvenNumber(8)));
    // let result: Result<EvenNumber, &str> = (5i32).try_into();
    // assert_eq!(result, Err("amogus"));
}
