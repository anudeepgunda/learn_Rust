//primitive data types
// int, float, bool, char
//
fn main() {
    let x: i32 = 42;
    let y: u64 = 100;
    let z: i64 = x + y;
    println!("signed Integer:{}", x);
    println!("unsigned Integer:{}", y);

    //float
    //f32 and f64
    let pi: f64 = 3.14;
    println!("pi value in float:{}", pi);
    // bool true, false
    let is_snowing: bool = true;
    println!("is snowing:{}", is_snowing);

    //char type
    let letter: char = 'a';
    println!("letter of alphabet:{}", letter);
}
