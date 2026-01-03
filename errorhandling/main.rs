fn main() {
    match divide_result(10.0, 0.0) {
        DivideResult::Ok(result) => println!("Result: {}", result),
        DivideResult::Err(err) => println!("Error :{}", err),
    }
}

//Approach-2
enum DivideResult<T, E> {
    // Defines the generic Result Type
    Ok(T),  // Represents a value
    Err(E), // Represents an Error
}

fn divide_result(numerator: f64, denominator: f64) -> DivideResult<f64, String> {
    if denominator == 0.0 {
        DivideResult::Err("Cannot Divide by 0".to_string())
    } else {
        DivideResult::Ok(numerator / denominator)
    }
}

//Approach-1
enum Options<T> {
    // Defines the generic Option type
    Some(T), // Represents the value
    None,    // Represents no value
}
fn divide_option(numerator: f64, denominator: f64) -> Option<f64> {
    if denominator == 0.0 {
        None
    } else {
        Some(numerator / denominator)
    }
}
