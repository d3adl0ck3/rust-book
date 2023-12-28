#[derive(Debug)]
struct MyDroppable {
    name: &'static str,
}
impl MyDroppable {
    fn new(name: &'static str) -> Self {
        println!("Allocating {name}");
        MyDroppable{name}
    }
}
impl Drop for MyDroppable {
    fn drop(&mut self) {
        println!("Getting the drop on me {}",self.name);
    }
}

pub fn all() {
    custom();
    variable_scope();
    the_string_type();
    variables_and_data_interacting_with_move();
    variables_and_data_interacting_with_clone();
    stack_only_data_copy();
    ownership_and_functions();
    return_values_and_scope();
    tuple_time();
    references_and_borrowing();
    mutable_references();
    the_slice_type();
    string_slices();
    string_slices_as_parameters();
    other_slices();
}
fn custom() {
    let drop_outer = MyDroppable::new("outer name");    
    {
        let drop_inner = MyDroppable::new("inner name");
        println!("drop_inner {:?} scope ends",drop_inner);
    }
    println!("drop_outer {:?} scope ends",drop_outer);
}
fn variable_scope() {
    let s = "hello";
    {
        let s = "hello";
        println!("s '{s}' is good till the end of this block");
        // s (just a reference to static data) dropped here
    }
    println!("s '{s}' is good till the end of this method");
}
fn the_string_type() {
    let mut s = String::from("hello");
    s.push_str(", world!");
    println!("{}",s);
}
/*
fn memory_and_allocation() {
  // I guess just see 'the_string_type'  
}
*/

fn variables_and_data_interacting_with_move() {
    // this just copies the value 5 to y as well, leaving x intact
    let x = 5;
    let y = x;
    println!("x:{x} y:{y}");

    // This reassignes ownership of the string from s1 to s2 afterwhich s1 is no longer valid
    let s1 = String::from("hello");
    let s2 = s1;
    //doesn't work!
    //println!("s1 {s1} s2 {s2}");
    println!("s1 NA s2 {s2}");

}
fn variables_and_data_interacting_with_clone() {
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 {s1} s2 {s2}");
}
fn stack_only_data_copy() {
    // scalar data types (ints/doubles/bools/chars) implement Copy so no need to clone
    let x = 5;
    let y = x;
    println!("x:{x} y:{y}");
}
fn ownership_and_functions() {
    let s = String::from("hello");  // s comes into scope

    takes_ownership(s);             // s's value moves into the function...
    // ... and so is no longer valid here

    let x = 5;                      // x comes into scope

    makes_copy(x);                  // x would move into the function,
    // but i32 is Copy, so it's okay to still
    // use x afterward
}
fn takes_ownership(some_string: String) {
    println!("{some_string}");
}// Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.
fn makes_copy(some_integer: i32) {
    println!("{some_integer}");
}
fn return_values_and_scope() {
    let _s1 = gives_ownership(); // moves ownership into return
    let s2 = String::from("hello"); // s2 comes into scope
    let _s3 = takes_and_gives_back(s2); // s2 moved into function then back out to s3
}
fn gives_ownership() -> String {             // gives_ownership will move its
                                             // return value into the function
                                             // that calls it

    let some_string = String::from("yours"); // some_string comes into scope

    some_string                              // some_string is returned and
                                             // moves out to the calling
                                             // function
}

// This function takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String { // a_string comes into
                                                      // scope
    a_string  // a_string is returned and moves out to the calling function
}

fn tuple_time() {
    // This is a hacky way to pass ownership and get it back when we should be passing in a
    // reference
    let s1 = String::from("hello");
    let (s2, len) = calculate_length(s1);
    println!("The length of '{}' is {}.", s2, len);
}
fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}
fn references_and_borrowing() {
    let s1 = String::from("hello");   
    let len = calculate_length_ref(&s1);
    println!("The length of '{}' is {}.", s1, len);
}
fn calculate_length_ref(s: &String) -> usize {
    s.len()
}
fn mutable_references() {
    let mut s = String::from("hello");
    change(&mut s);
    println!("Changed {s}");
    let _r1 = &mut s;
    // this won't work because we can only have one mutable reference and no
    // other references at all
    // let r2 = &mut s;  
    
    // Shadow with a new one
    let mut s = String::from("hello2");
    // We can take a mutable reference with a short lifetime and then take a new reference after
    {
        let _r1 = &mut s;
    }
    let _r2 = &mut s;

    // Can't take a mutable reference if any immutable ones exist
    let /*mut*/ s = String::from("hello");

    let _r1 = &s; // no problem
    let _r2 = &s; // no problem
    // Can't declare r3 as mutable reference
    //let r3 = &mut s; // BIG PROBLEM
    //println!("{}, {}, and {}", r1, r2, r3);
    
    // Ok to take a mutable reference after any usages of immutable ones
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);
    // variables r1 and r2 will not be used after this point

    let r3 = &mut s; // no problem
    println!("{}", r3);

}
fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn dangling_references() {
    // Eh, don't return a reference to owned data
}

fn the_slice_type() {
    
    let s = String::from("This is the end");
    // Should return 4 which is the length of the first word
    let _offset = first_word_string_ref(&s);

    // Create a mutable string
    let mut s = String::from("hello world");

    let _word = first_word_string_ref(&s); // word will get the value 5

    s.clear(); // this empties the String, making it equal to ""

    // word still has the value 5 here, but there's no more string that
    // we could meaningfully use the value 5 with. word is now totally invalid!
    
}

fn string_slices() {

    let s = String::from("hello world");
    let hello = &s[0..5];
    let world = &s[6..11];

    println!("hello '{hello}' world '{world}'");

    let s = String::from("hello");
    let slice = &s[0..2];
    println!("slice s[0..2] {slice}");
    let slice = &s[..2];
    println!("slice s[..2] {slice}");

    let len = s.len();

    let slice = &s[3..len];
    println!("slice s[3..len] {slice}");
    let slice = &s[3..];
    println!("slice s[3..] {slice}");

    let /*mut*/ s = String::from("hello world");
    let word = first_word_string_ref_return_slice(&s);
    // Can't do this since we have a reference into the word
    // s.clear() 
    println!("the first word is: {}",word);
}

fn first_word_string_ref(s: &String) -> usize {
   let bytes = s.as_bytes(); 
   for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
   }
   s.len()
}
 
fn first_word_string_ref_return_slice(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..] 
}

fn string_slices_as_parameters() {
    let s = "Hello, world!";
    let s = first_word(s);
    println!("s {s}");
    let my_string = String::from("hello world");

    // `first_word` works on slices of `String`s, whether partial or whole
    let _word = first_word(&my_string[0..6]);
    let _word = first_word(&my_string[..]);
    // `first_word` also works on references to `String`s, which are equivalent
    // to whole slices of `String`s
    let _word = first_word(&my_string);

    let my_string_literal = "hello world";

    // `first_word` works on slices of string literals, whether partial or whole
    let _word = first_word(&my_string_literal[0..6]);
    let _word = first_word(&my_string_literal[..]);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let _word = first_word(my_string_literal);
}
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..] 
}
fn other_slices() {
    let a = [1,2,3,4,5];
    let slice = &a[1..3];
    println!("Comparing slice");
    assert_eq!(slice,&[2,3]);
}
