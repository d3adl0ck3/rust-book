pub fn all() {
    all_the_places_patterns_can_be_used::conditional_if_let_expressions();
    all_the_places_patterns_can_be_used::while_let_conditional_loops();
    all_the_places_patterns_can_be_used::for_loops();
    all_the_places_patterns_can_be_used::let_statements();
    all_the_places_patterns_can_be_used::function_parameters();
    refutability_when_a_pattern_might_fail_to_match::does_not_work();
    matching_literals::matching_named_variables();
    matching_literals::multiple_patterns();
    matching_literals::matching_ranges_of_values_with_dot_dot_equal();
    matching_literals::destructuring_structs();
    matching_literals::destructuring_and_matching_literal_values_in_one_pattern();
    matching_literals::destructuring_enums();
    destructuring_nested_structs_and_enums::matching_on_nested_enums();
    destructuring_nested_structs_and_enums::destructuring_structs_and_tuples();
    destructuring_nested_structs_and_enums::ignoring_parts_of_a_value_with_a_nested_underscore();
    destructuring_nested_structs_and_enums::ignoring_parts_of_a_value_with_dot_dot();
    destructuring_nested_structs_and_enums::matching_only_the_first_and_last_values_in_a_tuple();
    extra_conditionals_with_match_guards::adding_a_match_guard_to_a_pattern();
    extra_conditionals_with_match_guards::using_a_match_guard_to_test_for_equality_with_an_outer_variable();
    extra_conditionals_with_match_guards::at_bindings();
}

mod all_the_places_patterns_can_be_used {
    pub fn conditional_if_let_expressions() {
        let favorite_color: Option<&str> = None;
        let is_tuesday = false;
        let age: Result<u8, _> = "34".parse();

        if let Some(color) = favorite_color {
            println!("Using your favorite color, {color}, as the background");
        } else if is_tuesday {
            println!("Tuesday is green day!");
        } else if let Ok(age) = age {
            if age > 30 {
                println!("Using purple as the background color");
            } else {
                println!("Using orange as the background color");
            }
        } else {
            println!("Using blue as the background color");
        }
    }
    pub fn while_let_conditional_loops() {
        let mut stack = Vec::new();
        stack.push(1);
        stack.push(2);
        stack.push(3);
        while let Some(top) = stack.pop() {
            println!("{}", top);
        }
    }
    pub fn for_loops() {
        let v = vec!['a', 'b', 'c'];

        for (index, value) in v.iter().enumerate() {
            println!("{} is at index {}", value, index);
        }
    }
    pub fn let_statements() {
        let x = 5;
        let (x, y, z) = (1, 2, 3);
        // This won't work since the number of elements in the tuple don't match
        // let (x, y) = (1, 2, 3);
        let (x, y, _) = (1, 2, 3);
        let (x, y, .. )= (1, 2, 3, 4);
    }
    fn print_coordinates(&(x, y): &(i32, i32)) {
        println!("Current location: ({}, {})", x, y);
    }

    pub fn function_parameters() {
        let point = (3, 5);
        print_coordinates(&point);
    }
}
mod refutability_when_a_pattern_might_fail_to_match {
    pub fn does_not_work() {
        // the option might be Some or None so this could fail so not allowed
        // error[E0005]: refutable pattern in local binding: `None` not covered
        // let Some(x) = some_option_value;
        // this works
        let some_option_value = Some(4);
        if let Some(x) = some_option_value {
            println!("{}", x);
        }
        // Similarly this does not work
        // if let x = 5 {
        //   println!("{}", x);
        //};
        // warning: irrefutable `if let` pattern
    }
}

mod matching_literals {
    pub fn matching_named_variables() {
        let x = Some(5);
        let y = 10;

        match x {
            Some(50) => println!("Got 50"),
            Some(y) => println!("Matched, y = {y}"),
            _ => println!("Default case, x = {:?}", x),
        }

        println!("at the end: x = {:?}, y = {y}", x);
    }
    pub fn multiple_patterns() {
        let x = 1;

        match x {
            1 | 2 => println!("one or two"),
            3 => println!("three"),
            _ => println!("anything"),
        }
    }
    pub fn matching_ranges_of_values_with_dot_dot_equal() {
        let x = 5;

        match x {
            1..=5 => println!("one through five"),
            _ => println!("something else"),
        }
        let x = 'c';

        match x {
            'a'..='j' => println!("early ASCII letter"),
            'k'..='z' => println!("late ASCII letter"),
            _ => println!("something else"),
        }
    }
    struct Point {
        x: i32,
        y: i32,
    }

    pub fn destructuring_structs() {
        let p = Point { x: 0, y: 7 };

        let Point { x: a, y: b } = p;
        assert_eq!(0, a);
        assert_eq!(7, b);
        let Point { x, y } = p;
        assert_eq!(0, x);
        assert_eq!(7, y);
    }

    pub fn destructuring_and_matching_literal_values_in_one_pattern() {
        let p = Point { x: 0, y: 7 };

        match p {
            Point { x, y: 0 } => println!("On the x axis at {x}"),
            Point { x: 0, y } => println!("On the y axis at {y}"),
            Point { x, y } => {
                println!("On neither axis: ({x}, {y})");
            }
        }
    }
    enum Message{
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }
    pub fn destructuring_enums() {
        let msg = Message::ChangeColor(0, 160, 255);

        match msg {
            Message::Quit => {
                println!("The Quit variant has no data to destructure.");
            }
            Message::Move { x, y } => {
                println!("Move in the x direction {x} and in the y direction {y}");
            }
            Message::Write(text) => {
                println!("Text message: {text}");
            }
            Message::ChangeColor(r, g, b) => {
                println!("Change the color to red {r}, green {g}, and blue {b}",)
            }
        }
    }
}
mod destructuring_nested_structs_and_enums {
    enum Color {
        Rgb(i32, i32, i32),
        Hsv(i32, i32, i32),
    }

    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(Color),
    }
    pub fn matching_on_nested_enums() {
        let msg = Message::ChangeColor(Color::Hsv(0, 160, 255));

        match msg {
            Message::ChangeColor(Color::Rgb(r, g, b)) => {
                println!("Change color to red {r}, green {g}, and blue {b}");
            }
            Message::ChangeColor(Color::Hsv(h, s, v)) => {
                println!("Change color to hue {h}, saturation {s}, value {v}")
            }
            _ => (),
        }
    }
    struct Point {
        x: i32,
        y: i32,
    }
    pub fn destructuring_structs_and_tuples() {
        let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });
        println!("{feet} {inches} {x} {y}"); 
    }
    pub fn ignoring_parts_of_a_value_with_a_nested_underscore() {
        let mut setting_value = Some(5);
        let new_setting_value = Some(10);

        match (setting_value, new_setting_value) {
            (Some(_), Some(_)) => {
                println!("Can't overwrite an existing customized value");
            }
            _ => {
                setting_value = new_setting_value;
            }
        }

        println!("setting is {:?}", setting_value);

        let numbers = (2, 4, 8, 16, 32);

        match numbers {
            (first, _, third, _, fifth) => {
                println!("Some numbers: {first}, {third}, {fifth}")
            }
        }
        let s = Some(String::from("Hello!"));

        if let Some(_s) = s {
            println!("found a string");
        }

        // We can't print this since it was consumed in the match expression
        //println!("{:?}", s);
        let s = Some(String::from("Hello!"));

        if let Some(_) = s {
            println!("found a string");
        }
        // This is ok since the Some(_) does not take ownership 
        println!("{:?}", s);
    }
    pub fn ignoring_parts_of_a_value_with_dot_dot() {
        struct Point {
            x: i32,
            y: i32,
            z: i32,
        }

        let origin = Point { x: 0, y: 0, z: 0 };

        match origin {
            Point { x, .. } => println!("x is {}", x),
        }
    }   
    pub fn matching_only_the_first_and_last_values_in_a_tuple() {
        let numbers = (2, 4, 8, 16, 32);

        match numbers {
            (first, .., last) => {
                println!("Some numbers: {first}, {last}");
            }
        }
        /*
         * this is not ok - ambiguous
         let numbers = (2, 4, 8, 16, 32);

         match numbers {
            (.., second, ..) => {
                println!("Some numbers: {}", second)
            },
         }
         */
    }
}
mod extra_conditionals_with_match_guards {
    pub fn adding_a_match_guard_to_a_pattern() {
        let num = Some(4);

        match num {
            Some(x) if x % 2 == 0 => println!("The number {} is even", x),
            Some(x) => println!("The number {} is odd", x),
            None => (),
        }
    }
    pub fn using_a_match_guard_to_test_for_equality_with_an_outer_variable() {
        let x = Some(5);
        let y = 10;

        match x {
            Some(50) => println!("Got 50"),
            Some(n) if n == y => println!("Matched, n = {n}"),
            _ => println!("Default case, x = {:?}", x),
        }

        println!("at the end: x = {:?}, y = {y}", x);
    }
    pub fn at_bindings() {
        enum Message {
            Hello { id: i32 },
        }

        let msg = Message::Hello { id: 5 };

        match msg {
            Message::Hello {
                id: id_variable @ 3..=7,
            } => println!("Found an id in range: {}", id_variable),
            Message::Hello { id: 10..=12 } => {
                println!("Found an id in another range")
            }
            Message::Hello { id } => println!("Found some other id: {}", id),
        }
    }
}
