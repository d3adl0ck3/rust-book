pub fn all() {
    raw_pointers::dereferenceing_raw_pointers();
    raw_pointers::call_dangerous();
    raw_pointers::using_the_safe_split_at_mut_function();
    extern_functions_to_call_external_code::try_it();
    // Ok we're not really calling it from C but it is close :-)
    extern_functions_to_call_external_code::call_from_c();
    accessing_or_modifying_a_mutable_static_variable::defining_and_using_an_immutable_static_variable();
    accessing_or_modifying_a_mutable_static_variable::reading_from_or_writing_to_a_mutable_static_variable_is_unsafe();
    advanced_traits::try_it_out();
    default_generic_type_parameters_and_operator_overloading::implementing_the_add_trait_to_overload_the_plus_operator();
    fully_qualified_syntax_for_disambiguation::calling_fly_on_an_instance_of_human();
    fully_qualified_syntax_for_disambiguation::specifying_which_traits_fly_method_we_want_to_call();
    fully_qualified_syntax_for_disambiguation::a_trait_with_an_assoiated_function_blah();
    using_supertraits_to_require_one_traits_functionality_within_another_trait::try_it_out();
    using_the_newtype_pattern_to_implement_external_traits_on_external_types::creating_a_wrapper_around_vec_string();
    creating_type_synonyms_with_type_aliases::try_it_out();
    function_pointers::try_it_out();
}

mod raw_pointers {
    fn never_gonna_do_this() {
        // WOOF!
        let address = 0x012345usize;
        let r = address as *const i32;
    }
    pub fn dereferenceing_raw_pointers() {
        let mut num = 5;

        let r1 = &num as *const i32;
        let r2 = &mut num as *mut i32;

        unsafe {
            println!("r1 is: {}", *r1);
            println!("r2 is: {}", *r2);
        }
    }
    unsafe fn dangerous() {
        println!("dangerous!");
    }
    pub fn call_dangerous() {
        unsafe{
            dangerous();
        }
    }
    pub fn using_the_safe_split_at_mut_function() {
        let mut v = vec![1, 2, 3, 4, 5, 6];
        let r = &mut v[..];
        let (a, b) = r.split_at_mut(3);
        assert_eq!(a, &mut [1, 2, 3]);
        assert_eq!(b, &mut [4, 5, 6]);
        println!("Safe abstraction over unsafe!");
    }
    use std::slice;

    fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
        let len = values.len();
        let ptr = values.as_mut_ptr();

        assert!(mid <= len);

        unsafe {
            (
                slice::from_raw_parts_mut(ptr, mid),
                slice::from_raw_parts_mut(ptr.add(mid), len - mid),
                )
        }
    }
}

mod extern_functions_to_call_external_code {
    extern "C" {
        fn abs(input: i32) -> i32;
    }
    pub fn try_it() {
        unsafe {
            println!("Absolute value of -3 according to c: {}", abs(-3));
        }
    }
    #[no_mangle]
    pub extern "C" fn call_from_c() {
        println!("Just called a Rust function from C!");
    }
}

mod accessing_or_modifying_a_mutable_static_variable {
    static HELLO_WORLD: &str = "Hello, world!";

    pub fn defining_and_using_an_immutable_static_variable() {
        println!("name is: {}", HELLO_WORLD);
    }
    static mut COUNTER: u32 = 0;

    fn add_to_count(inc: u32) {
        unsafe {
            COUNTER += inc;
        }
    }

    pub fn reading_from_or_writing_to_a_mutable_static_variable_is_unsafe() {
        add_to_count(3);

        unsafe {
            println!("COUNTER: {}", COUNTER);
        }
    }
}

mod implementing_an_unsafe_trait {
    unsafe trait Foo {
        // methods go here
    }

    unsafe impl Foo for i32 {
        // method implementations go here
    }

    fn main() {}
}

mod advanced_traits {
    pub trait MyIterator {
        type Item;
        fn next(&mut self) -> Option<Self::Item>;
    }
    struct Counter {
        value: usize,
        max: usize,
    }
    impl Counter {
        fn new(max: usize) -> Counter {
            Counter{ value: 0, max}
        }
    }
    impl MyIterator for Counter {
       type Item = usize;
       fn next(&mut self) -> Option<Self::Item> {
        if self.value>=self.max {
            None
        } else {
            let current = self.value;
            self.value += 1;
            Some(current)
        }
       }
    }
    pub fn try_it_out() {
        let mut counter = Counter::new(10);
        while let Some(i) = counter.next() {
            println!("Value was {i}");
        }
    }
}
mod default_generic_type_parameters_and_operator_overloading {
    use std::ops::Add;

    #[derive(Debug, Copy, Clone, PartialEq)]
    struct Point {
        x: i32,
        y: i32,
    }

    impl Add for Point {
        type Output = Point;

        fn add(self, other: Point) -> Point {
            Point {
                x: self.x + other.x,
                y: self.y + other.y,
            }
        }
    }

    pub fn implementing_the_add_trait_to_overload_the_plus_operator() {

            let result = Point { x: 1, y: 0 } + Point { x: 2, y: 3 };
            let expected = Point { x: 3, y: 3 };

            println!("expected {:?} result {:?}",expected,result);
    }   

    trait MyAdd<Rhs=Self> {
        type Output;
        fn add(self, rhs: Rhs) -> Self::Output;
    }

    struct Millimeters(u32);
    struct Meters(u32);

    impl Add<Meters> for Millimeters {
        type Output = Millimeters;

        fn add(self, other: Meters) -> Millimeters {
            Millimeters(self.0 + (other.0 * 1000))
        }
    }
    impl Add for Millimeters {
        type Output = Millimeters;

        fn add(self, other: Millimeters) -> Millimeters {
            Millimeters(self.0 + other.0)
        }
    }
}
mod fully_qualified_syntax_for_disambiguation {
    trait Pilot {
        fn fly(&self);
    }

    trait Wizard {
        fn fly(&self);
    }

    struct Human;

    impl Pilot for Human {
        fn fly(&self) {
            println!("This is your captain speaking.");
        }
    }

    impl Wizard for Human {
        fn fly(&self) {
            println!("Up!");
        }
    }

    impl Human {
        fn fly(&self) {
            println!("*waving arms furiously*");
        }
    }
    pub fn calling_fly_on_an_instance_of_human() {
        let person = Human;
        person.fly();
    }
    pub fn specifying_which_traits_fly_method_we_want_to_call() {
        let person = Human;
        Pilot::fly(&person);
        Wizard::fly(&person);
        Human::fly(&person);// This is the same as the next line
        person.fly();
    }
    trait Animal {
        fn baby_name() -> String;
    }

    struct Dog;

    impl Dog {
        fn baby_name() -> String {
            String::from("Spot")
        }
    }

    impl Animal for Dog {
        fn baby_name() -> String {
            String::from("puppy")
        }
    }

    pub fn a_trait_with_an_assoiated_function_blah() {
        println!("A baby dog is called a {}", Dog::baby_name());
        // This doesn't work since there could be many implementations of Animal
        // println!("A baby dog is called a {}", Animal::baby_name());
        // Using this syntax is cool where in this case there is no receiver
        // <Type as Trait>::function(receiver_if_method, next_arg, ...);
        println!("A baby dog is called a {}", <Dog as Animal>::baby_name())
    }
}
mod using_supertraits_to_require_one_traits_functionality_within_another_trait {
   use std::fmt; 
   trait OutlinePrint : fmt::Display {
       fn outline_print(&self) {
           let output = self.to_string();
           let len = output.len();
           println!("{}", "*".repeat(len + 4));
           println!("*{}*", " ".repeat(len + 2));
           println!("* {} *", output);
           println!("*{}*", " ".repeat(len + 2));
           println!("{}", "*".repeat(len + 4));
       }
   }
   struct Point {
        x: i32,
        y: i32,
   }
   impl fmt::Display for Point {
       fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f,"({}, {})", self.x, self.y)
       }
   }
   impl OutlinePrint for Point {}
   pub fn try_it_out() {
    let p = Point{x:3,y:7};
    p.outline_print();
   }
}
mod using_the_newtype_pattern_to_implement_external_traits_on_external_types {
    use std::fmt;

    struct Wrapper(Vec<String>);

    impl fmt::Display for Wrapper {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "[{}]", self.0.join(", "))
        }
    }

    pub fn creating_a_wrapper_around_vec_string() {
        let w = Wrapper(vec![String::from("hello"), String::from("world")]);
        println!("w = {}", w);
    }
}
mod creating_type_synonyms_with_type_aliases {
    type Kilometers = i32;
    pub fn try_it_out() {
        let x: i32 = 5;
        let y: Kilometers = 5;

        println!("x + y = {}", x + y);
    }
}
mod introducing_a_type_alias_thunk_to_reduce_repetition {
    type Thunk = Box<dyn Fn() + Send + 'static>;
    fn try_it_out()->Thunk {
        Box::new(|| println!("hi"))
    }
    fn takes_long_type(f: Thunk) {
        // --snip--
    }

    fn returns_long_type() -> Thunk {
        try_it_out()
    }   
}
mod the_never_type_that_never_returns {
    fn bar() -> ! {
        panic!("just panic");
    }
}
mod dynamically_sized_types {
    // I am not sure what to put in here!
}
mod function_pointers {
    fn add_one(x: i32) -> i32 {
        x + 1
    }

    fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
        f(arg) + f(arg)
    }

    pub fn try_it_out() {
        let answer = do_twice(add_one, 5);
        let list_of_numbers = vec![1, 2, 3];
        let list_of_strings: Vec<String> =
            list_of_numbers.iter().map(|i| i.to_string()).collect();
        let list_of_numbers = vec![1, 2, 3];
        let list_of_strings: Vec<String> =
            list_of_numbers.iter().map(ToString::to_string).collect();
        println!("The answer is: {}", answer);

        enum Status {
            Value(u32),
            Stop,
        }

        let list_of_statuses: Vec<Status> = (0u32..20).map(Status::Value).collect();
    }
}
