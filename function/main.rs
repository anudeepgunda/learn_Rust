fn main() {
    println!("Hello, world!");
    tell_height(143);
    human_id("Alice", 22, 182.5);
    let _x = {
        let price: i32 = 5;
        let qty: i32 = 10;
        price * qty
    };
    println!("Result:{}", _x);
    let add = add_numbers(12, 10);
    println!("Sumof 2 Numbers {}", add);

    let bmi = calcuate_bmi(74.2, 1.4);
    println!("BMI is: {:.2}", bmi);
}

fn tell_height(height: i32) {
    println!("Height:{}", height);
}

fn human_id(name: &str, age: u32, height: f32) {
    println!(
        "My Nameis: {}, I am {} years old, and My height is {} cm",
        name, age, height
    );
}

fn add_numbers(number1: i32, number2: i32) -> i32 {
    number1 + number2
}

//BMI= weight(Kg)/height(m)^2
fn calcuate_bmi(weight: f32, height: f32) -> f32 {
    weight / (height * height)
}
