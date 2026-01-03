fn main() {
    /*
    let mut counter = 0;
    loop {
        if counter == 60210 {
            break;
        }
        println!("value in the loop increment: {counter}");
        counter = counter + 1;
    }
    */
    //While
    let mut number = 3;
    while number != 0 {
        println!("{number}");
        number -= 1;
        if number == 2 {
            break;
        }
    }

    //for loop
    let a = [1, 2, 3, 4, 5, 6];
    let b = ["a", "b", "c", "d", "e", "f"];
    for element in a {
        println!("{element}");
        for alpha in b {
            println!("{alpha}{element}");
        }
    }
}
