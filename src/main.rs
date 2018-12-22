#![feature(step_trait)]

use std::iter::Step;
use std::fmt::Display;
use std::convert::From;

// As not all the possible type conversions from integers to floats are implemented by the primitive types
// we would need to implement the conversions here using traits - but it falls foul of Orphan rules for Trait implementations
// https://doc.rust-lang.org/error-index.html#E0117
// So this needs fixing
impl From<f32> for u16 {
    fn from(float: f32) -> Self {
        float as u16
    }
}

fn check_conversion<Original: Step + Display + From<To>, To: Display + From<Original>>(min: Original, max: Original, _fake: To, from_type: &str, to_type: &str) {
    let mut converted: To;
    let mut converted_back: Original;
    let mut count: usize = 0;

    println!("Checking from {} to {}", from_type, to_type);
    for original in min..max {
        converted = To::from(original);
        converted_back = Original::from(converted);
        if original != converted_back {
            println!("{} in {} is {} in {}", original, from_type, converted, to_type);
            count += 1;
        }
    }
    println!("total count that do not match between u32 and f32 is {}", count);
}

fn main() {
    // Do for u16 to f32 first
    let min:u16 = std::u16::MIN;
    let max: u16 = std::u16::MAX;
    let fake: f32 = 0.0;
    check_conversion(min, max, fake, "u32", "f32" )

}
