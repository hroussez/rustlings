// option2.rs
// Make me compile! Execute `rustlings hint option2` for hints

fn main() {
    let optional_word = Some(String::from("rustlings"));
    let word = if optional_word.is_some() {
        Some(println!("The word is: {}", optional_word.unwrap()));
    } else {
        Some(println!("The optional word doesn't contain anything"));
    };

    let mut optional_integers_vec: Vec<Option<i8>> = Vec::new();
    for x in 1..10 {
        optional_integers_vec.push(Some(x));
    }

    while let Some(integer) = optional_integers_vec.pop() {
        println!("current value: {}", integer.unwrap());
    }
}
