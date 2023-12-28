//

pub fn all(interactive: bool) {
    variables_and_mutability();
    shadowing();
    data_types();
    numeric_operations();
    boolean_type();
    the_tuple_type();
    the_array_type();
    if interactive {
        invalid_array_element_access();
    }
    functions();
    statements_and_expressions();
    functions_with_return_values();
    comments();
    control_flow();
    using_if_in_a_let_statement();
    repetition_with_loops();
    loop_labels_to_disambiguate_between_multiple_loops();
    conditional_loops_with_while();
    looping_through_a_collection_with_for();
}
fn variables_and_mutability() {
    let x = 5;
    println!("The value of x is {x}");
    // x = 6; Cannot work since x is not mut
    // Redeclare x as mut - this is a different x!
    let mut x = 5;
    println!("Mut x is {x}");
    x = 6;
    println!("mut x is now {x}");
    println!("THREE_HOURS_IN_SECONDS {THREE_HOURS_IN_SECONDS}");
}

const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn shadowing() {
    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }
    println!("The value of x is: {x}");

    let spaces = "    ";
    println!("Spaces string is '{spaces}'");
    let spaces = spaces.len();
    println!("Spaces len is {spaces}");
}

fn data_types() {
    // let guess = "42".parse().expect("Not a number??"); - Requires type information to know which parse to call
    let guess: u32 = "42".parse().expect("Not a number??");
    println!("{guess}");
    let value: i8 = -8;
    println!("i8 value {value}");
    let value: u8 = 8;
    println!("u8 value {value}");
    let value: i16 = -257;
    println!("i16 value {value}");
    let value: u16 = 65530;
    println!("u16 value {value}");
    let value: i32 = -70000;
    println!("i32 value {value}");
    let value: u32 = 2_147_483_649;
    println!("u32 value {value}");
    let value: i64 = -70000;
    println!("i64 value {value}");
    let value: u64 = 2_147_483_649;
    println!("u64 value {value}");
    let value: i128 = -70000;
    println!("i128 value {value}");
    let value: u128 = 2_147_483_1289;
    println!("u128 value {value}");
    let value: isize = 0b1111_0000;
    println!("isize binary value {value}");
    let value: usize = 0b1111_0000;
    println!("usize binary value {value}");
    let value: f32 = 2.0;
    println!("f32 value {value}");
    let value: f64 = 2.0;
    println!("f64 value {value}");
}
fn numeric_operations() {
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1

    // remainder
    let remainder = 43 % 5;
    println!("{sum} {difference} {product} {quotient} {truncated} {remainder}");
}
fn boolean_type() {
    let t = true;
    let f: bool = false;
    println!("{t} {f}");
}

fn the_tuple_type() {
    let tup = (500, 6.4, 1);

    let (_, y, _) = tup;

    println!("The value of y is: {y}");

    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;

    println!("{five_hundred} {six_point_four} {one}");
}

fn the_array_type() {
    let a = [1,2,3,4,5];
    println!("{:?}",a);
    let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];
    println!("{:?}",months);

    let a: [i32;5] = [1,2,3,4,5];
    println!("{:?}",a);
    let a  = [3;5];
    println!("{:?}",a);
    let a = [1,2,3,4,5];
    let first = a[0];
    let second = a[1];
    println!("first {first} second {second}");
}
fn invalid_array_element_access() {
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    std::io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}
 
fn functions() {
    another_function(12);
    print_labeled_measurement(5,'h');
}
fn another_function(x: i32) {
    println!("Another Function {x}!");
}
fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}
fn statements_and_expressions() {
    let y = {
        let x = 3;
        x + 1 // No semi-colon! returns value of expression
    }; // Semi-colon required here
    println!("The value of y is: {y}");
}
fn functions_with_return_values() {
    let x = five();
    println!("The value of x is: {x}");
    let x = plus_one(5);
    println!("The value of x is: {x}");
}
fn five() -> i32 {
    5
}
fn plus_one(x: i32) -> i32 {
    x + 1
}
fn comments() {
    println!("Check out my amaze-balls comments!");
    // Till end of line comment
    /* Wrapping comment
     This is a wrapping comment
     ** Some people put more stars
     */
}
fn control_flow() {
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
    let number = 7;
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    // This doesn't work since the if can only check a boolean
    /*
     * let number = 3;
     * if number {
     *  println!("number was three");
     *  }
     */
    let number = 3;
    if number != 0 {
        println!("number was something other than zero");
    }
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}
fn using_if_in_a_let_statement() {
    let condition = true;
    let number = if condition {5} else {6};
    println!("The value of the number is: {number}");
    /*
     * This doesn't work since return types must be the same
     *  let number = if condition { 5 } else { "six" };
     */
}
fn repetition_with_loops() {

    // In the text this is an infinite loop requiring a ctrl-C
    // here we add the counter and break
    let mut x = 0;
    loop {
        println!("again! {x}");
        x += 1;
        if x >= 5 { break };
    }

    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");
}
fn loop_labels_to_disambiguate_between_multiple_loops() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}
fn conditional_loops_with_while() {
    let mut number = 3;
    while number != 0 {
        println!("{number}!");
        number -= 1;
    }
    println!("LIFTOFF!!!");
}
fn looping_through_a_collection_with_for() {
    let a = [10,20,30,40,50];
    let mut index = 0;
    while index < 5 {
        println!("the value is: {}", a[index]);
        index += 1;
    }

    for element in a {
        println!("the value is: {element}");
    }

    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LEFTOFF!!!");

}
