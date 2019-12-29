#[macro_use] extern crate text_io;

fn main() {
    let x: i64 = read!();
    if(x % 4) == 0 && (x % 100) != 0{
        println!("The year, {}, is a leap year", x);
    }
    else if (x % 400) == 0 {
        println!("The year, {}, is a leap year", x);
    }
    else {
        println!("The year, {}, is not a leap year", x);
    }
}
