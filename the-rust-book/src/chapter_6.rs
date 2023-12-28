// Enums and Pattern Matching
pub fn all() {
    defining_an_enum();
    call_enum_method();
    option_stuff();
    the_match_control_flow_construct();
    matching_with_option_t();
    catch_all_patterns_and_the_underscore_placeholder();
    concise_control_flow_with_if_let();
}
enum IpAddrKind {
    V4(u8,u8,u8,u8),
    V6(String),
}
fn defining_an_enum() {
    let home = IpAddrKind::V4(127,0,0,1);
    let loopback = IpAddrKind::V6(String::from("::1"));
    route(home);
    route(loopback);
}

fn route(_ip_kind: IpAddrKind) {

}
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32,i32,i32),
}
impl Message {
    fn call(&self) {
        println!("Called on enum varient!");
    }
}
fn call_enum_method() {
    let var = Message::Move{x:1,y:-1};
    var.call();
}

// The Option Enum and Its Advantages Over Null Values
pub fn option_stuff() {
    let some_number = Some(5);
    let some_char = Some('e');
    let absent_number: Option<i32> = None;

    println!("{:?} {:?} {:?}",some_number,some_char,absent_number);
}
#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // -- snip--
    Utah,
    // -- snip--
}
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Pennylicious!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!",state);
            25
        },
    }
}
fn the_match_control_flow_construct() {
   let cents_in_quarter = value_in_cents(Coin::Quarter(UsState::Utah)); 
   println!("cents in quarter {cents_in_quarter}");
}
fn matching_with_option_t() {
    // Note it is valid to define a function internal to another function...
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }
    let five = Some(5);
    println!("Some(5) {:?}",five);
    let six = plus_one(five);
    println!("Some(6) {:?}",six);
    let none = plus_one(None);
    println!("Nun {:?}",none);
}
/*
 * Won't compile!  Must handle all varients
fn matches_are_exhaustive(x: Option<i32>) -> Option<i32> {
    match x {
        Some(i) => Some(i+1),
    }
    
}
*/

fn catch_all_patterns_and_the_underscore_placeholder() {
    // Do something specfic with the number when not 3/7 
    fn inner_1() {
        let dice_roll = 9;
        match dice_roll {
            3 => add_fancy_hat(),
            7 => remove_fancy_hat(),
            other => move_player(other),
        }
        fn add_fancy_hat() {}
        fn remove_fancy_hat() {}
        fn move_player(_num_spaces: u8) {}
    }
    // Do something non-specific when the number is not 3/7
    fn inner_2() {
        let dice_roll = 9;
        match dice_roll {
            3 => add_fancy_hat(),
            7 => remove_fancy_hat(),
            _ => reroll(),
        }

        fn add_fancy_hat() {}
        fn remove_fancy_hat() {}
        fn reroll() {}
    }
    // Don't do anything with number if not 3/7
    fn inner_3() {
        let dice_roll = 9;
        match dice_roll {
            3 => add_fancy_hat(),
            7 => remove_fancy_hat(),
            _ => (),
        }

        fn add_fancy_hat() {}
        fn remove_fancy_hat() {}
    }
    inner_1();
    inner_2();
    inner_3();
}
fn concise_control_flow_with_if_let() {
    fn using_match() {
        let config_max = Some(3u8);
        match config_max {
            Some(max) => println!("The maximum is configured to be {}",max),
            _ => (),
        }
    }
    fn using_if_let() {
        let config_max = Some(3u8);
        if let Some(max) = config_max {
          println!("The maximum is configured to be {}", max);  
        }
    }
    fn count_non_quarters_match(coin: &Coin) {
        let mut count = 0;
        match coin {
            Coin::Quarter(state) => println!("State quarter from {:?}!",state),
            _ => count += 1,
        }
        println!("Count is {count}");

    }
    fn count_non_quarters_if_let(coin: &Coin) {
        let mut count = 0;
        if let Coin::Quarter(state) = coin {
            println!("State quarter from {:?}!",state);
        } else {
            count += 1;
        }
        println!("Count is {count}");
    }
    using_match();
    using_if_let();
    let quarter = Coin::Quarter(UsState::Utah);
    count_non_quarters_match(&quarter);
    count_non_quarters_if_let(&quarter);
    let penny = Coin::Penny;
    count_non_quarters_match(&penny);
    count_non_quarters_if_let(&penny);
}
