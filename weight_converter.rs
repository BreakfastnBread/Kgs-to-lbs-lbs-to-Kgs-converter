use std::io;

fn main() {
    let mut weight_input = String::new();
    let mut unit_input = String::new();

    println!("how much do you weight [ number ]?");

    io::stdin()
        .read_line(&mut weight_input)
        .expect("Failed to read line");

    let weight: f32 = weight_input
        .trim()
        .parse()
        .expect("Failed to parse");

    println!("In what unit? [ Lb, Kg ]?");

    io::stdin()
        .read_line(&mut unit_input)
        .expect("Failed to read line");

    let weight_in_pounds = &weight/0.45;
    let weight_in_kilos = &weight * 0.45;

    unit_input.pop();

    match unit_input.as_str() {
        "Kg" => println!("You weigh {} pounds!", weight_in_pounds),
        "Lb" => println!("You weigh {} kilos!", weight_in_kilos),
        &_ => println!("Unrecognised input: {}", unit_input),
    }
}
