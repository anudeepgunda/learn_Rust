fn main() {
    let _x: i32 = 5;
    let _x = _x + 1;
    {
        let _x = "special";
        println!("inner scope x value is : {}", _x);
    }

    println!("the value of  x in outter scope is : {}", _x);
}
