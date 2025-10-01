fn another_function() {
    println!("Another function.");
}

// This is a fuction with parameters, expecting for an argument, parameters should include the type
fn another_function_0(x: i32) {
    println!("The value of x is: {x}");
}

// When a fuction has multiple parameters, separate them with a coma ','
fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn main() {
    println!("Hello, world!");

    another_function(); // This is a fuction

    another_function_0(5); // This is a fuction with arguments

    print_labeled_measurement(5, 'h');

    let y = 6; // This is a statement
    println!("{y}");

    // In rust there are either statements or expressions
    // for instance, the inner code in the curly braces is an expresion
    let y = {
        let x = 3;
        x + 1
    };
    println!("The value of y is: {y}"); 

    let x = five();
    println!("The value of x is: {x}");

    let x = plus_one(5);
    println!("The value of x is: {x}");

}

