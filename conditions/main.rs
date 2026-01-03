fn main() {
    let age: i16 = 18;
    if age >= 18 {
        println!("You can drive a Car");
    } else {
        println!("You cannot drive a car");
    }

    // regular way o fhandling multiple conditions
    let number: u16 = 6;
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 3");
    } else {
        println!("number is divisible by 4,3 and 2");
    }
    //this can be handled in match based:
    match (number % 2, number % 3, number % 4) {
        (0, _, _) => {
            println!("divisible by 2")
        }
        (_, 0, _) => {
            println!("divisible by 3")
        }
        (_, _, 0) => {
            println!("divisible by 4")
        }
    }

    let condition = true;
    let conditionnumber = if condition { 5 } else { 6 };
    println!("number is: {conditionnumber}");
}
