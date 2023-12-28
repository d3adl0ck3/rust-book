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
}
