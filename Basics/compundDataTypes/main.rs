//Compound data types
// //arrays, tuples, slices and strings

fn main() {
    //arrays

    let letters: [char; 3] = ['a', 'b', 'c'];
    println!("Letter array:{:?}", letters);

    let fruits: [&str; 3] = ["apple", "Banana", "Grapes"];
    println!("Fruits Array 1st element:{:?}", fruits[0]);
    println!("Fruits Array 2nd element:{:?}", fruits[1]);
    println!("Fruits Array 3rd element:{:?}", fruits[2]);

    //Tuples
    let human: (String, i32, bool) = ("Alice".to_string(), 30, false);
    println!("Human Tuple:{:?}", human);

    let my_mix_tuple = ("Kratos", 23, true, [1, 2, 4, 3, 5]);
    println!("My Mix Tuple:{:?}", my_mix_tuple);

    //slices: [1,2,3,4,5]
    let number_slices: &[i32] = &[1, 2, 3, 4, 5];
    println!("Number Slices:{:?}", number_slices);

    let animal_slices: &[&str] = &["Lion", "ELephant", "Crocodile"];
    println!("Animal Slices:{:?}", animal_slices);

    let book_slices: &[&str] = &["Harry Potter", "IT", "ZEN"];
    println!("Book Slices:{:?}", book_slices);

    //string vs string slices(&str)
    //String [growable, mutable, owned string type]
    let mut stone_cold: String = "Hell, ".to_string();
    println!("Stone Cold Says :{}", stone_cold);
    stone_cold.push_str("Yeah!");
    println!("Stone Cold append :{}", stone_cold);

    //B-string Slice
    let string: String = String::from("Hello World!");
    let slice: &str = &string;
    println!("Slice Value :{}", slice);
}
