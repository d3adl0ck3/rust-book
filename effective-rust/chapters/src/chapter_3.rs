pub fn all() {
    println!("This is chapter 3!");
    item_1::mismatched_types();
    item_1::tuple_struct();
    item_1::basic_enum();
    item_1::print_page_blah(true,false);
    item_1::print_page_now();
    item_1::enum_match();
    item_2::function_pointers();
    item_2::closures::try_it_out_without_closures();
    item_2::closures::try_it_out_with_closures();
}
mod item_1 {
    pub fn mismatched_types() {
     let x: i32 = 42;
     // Nope!  i32 can't fit into an i16
     //let y: i16 = x;
     //This works but would panic if the value doesn't fit
     let y: i16 = x.try_into().unwrap();
     println!("x:{x} y:{y}");
     let x = 42i32; // Integer literal with type suffix
     let y: i64 = x.into(); // into is required even though this is a 'safe' conversion
     println!("x:{x} y:{y}");
    }
    pub fn tuple_struct() {
        struct TextMatch(usize, String);
        let m = TextMatch(12, "needle".to_owned());
        assert_eq!(m.0, 12);
    }
    enum HttpResultCode {
        Ok = 200,
        NotFound = 404,
        Teapot = 418,
    }
    pub fn basic_enum() {
        let code = HttpResultCode::NotFound;
        assert_eq!(code as i32, 404);
    }
    pub fn print_page_blah(both_sides: bool, colour: bool) {
        println!("both_sides: {both_sides} colour: {colour}");
    }
    #[derive(Debug)]
    pub enum Sides {
        Both,
        Single,
    }
    #[derive(Debug)]
    pub enum Output {
        BlackAndWhite,
        Colour,
    }

    pub fn print_page(sides: Sides, colour: Output) {
        println!("both_sides: {:?} colour: {:?}",sides,colour);
    }
    pub fn print_page_now() {
        print_page(Sides::Both,Output::BlackAndWhite);
        print_page(Sides::Single,Output::Colour);
        // Won't even compile now :-)
        //print_page(Output::Colour,Sides::Both);
    }
    pub fn enum_match() {
        let code = HttpResultCode::Teapot;
        let msg = match code {
            HttpResultCode::Ok => "Ok",
            HttpResultCode::NotFound => "Not found",
            // forgot to deal with the all-important "I'm a teapot" code
            // Must handle all variants or provide catch all
            _ => "Without this catch all it won't compile!",
        };
    }
    // enums with fields stuff
    struct Job;
    struct CpuId;
    use  std::collections::{HashSet,HashMap};
    pub enum SchedulerState {
        Inert,
        Pending(HashSet<Job>),
        Running(HashMap<CpuId, Vec<Job>>),
    }
    #[derive(Debug)]
    struct RgbColour(u32,u32,u32);
    // Dead giveaway that there should be some enum variants to enforce
    // that a fg_colour isn't applicable when monochrome
    pub struct DisplayProps {
        x: u32,
        y: u32,
        monochrome: bool,
        // `fg_colour` must be (0, 0, 0) if `monochrome` is true.
        fg_colour: RgbColour,
    }
    #[derive(Debug)]
    enum Colour {
        Monochrome,
        Foreground(RgbColour),
    }

    struct DisplayProperties {
        x: u32,
        y: u32,
        colour: Colour,
    }
    // options and errors stuff
    // No examples but generally speaking return a Result<T,ErrorType>
    // whenever something can fail!
    // Always use Option<Something> when it may or may not be present
    // Jusdgement required for collection types where empty may or may
    // not mean the same thing as absent
}
mod item_2 {
    enum Shape {
        Rectangle { width: f64, height: f64 },
        Circle { radius: f64 },
    }

    impl Shape {
        pub fn area(&self) -> f64 {
            match self {
                Shape::Rectangle { width, height } => width * height,
                Shape::Circle { radius } => std::f64::consts::PI * radius * radius,
            }
        }
    }
    
    fn sum(x: i32, y: i32) -> i32 {
        x + y
    }
    pub fn function_pointers() {
        let op: fn(i32, i32) -> i32 = sum;
        let result = op(2,3);
        println!("result is {result}");
    }
    pub mod closures {
        // In real code, an `Iterator` method would be more appropriate.
        // Should use the FnMut trait instead so it can accept closures
        //fn modify_all(data: &mut [u32], mutator: fn(u32) -> u32) {
        //    for value in data {
        //        *value = mutator(*value);
        //    }
       // }
        fn add2(v: u32) -> u32 {
            v + 2
        }
        pub fn try_it_out_without_closures() {
            let mut data = vec![1,2,3];
            modify_all(&mut data, add2);
            assert_eq!(data, vec![3, 4, 5,]);
        }
        pub fn try_it_out_with_closures() {
            let amount_to_add = 3;
            let add_n = |y| {
                y + amount_to_add
            };
            let z = add_n(5);
            println!("add_n of 5 = {z}");
            assert_eq!(z, 8);
        }
        pub fn modify_all<F>(data: &mut [u32], mut mutator: F)
            where
            F: FnMut(u32) -> u32,
            {
                for value in data {
                    *value = mutator(*value);
                }
            }
    }
    pub mod traits {
        pub trait Sort {
            fn sort(&mut self);
        }
        // Use marker traits to distinguish behaviors that cannot be expressed in the trait
        // method siganture
        pub trait StableSort: Sort {}
    }
    use std::fmt::Debug;
    use crate::chapter_3::item_2::traits::Sort;
    // Prefer accepting trait types to concrete types
    // Use trait bounds to express requirements on the types used in generics
    pub fn dump_sorted<T>(mut collection: T)
        where
        T: Sort + IntoIterator,
        T::Item: Debug,
        {
            // Next line requires `T: Sort` trait bound.
            collection.sort();
            // Next line requires `T: IntoIterator` trait bound.
            for item in collection {
                // Next line requires `T::Item : Debug` trait bound
                println!("{:?}", item);
            }
        }

}
mod item_3 {
    // Avoid matching Option and Result
    pub fn avoid_match() {
        struct S {
            field: Option<i32>,
        }

        let s = S { field: Some(42) };
        match &s.field {
            Some(i) => println!("field is {}", i),
            None => {}
        }
        if let Some(i) = &s.field {
            println!("field is {}", i);
        }
        let result = std::fs::File::open("/etc/passwd");
        let f = match result {
            Ok(f) => f,
            Err(_e) => panic!("Failed to open /etc/passwd!"),
        };
        let f = std::fs::File::open("/etc/passwd").unwrap();
    }

    // prefer Result to Option
    pub struct UserId;
    pub fn find_user(username: &str) -> Result<UserId, std::io::Error> {
        let f = match std::fs::File::open("/etc/passwd") {
            Ok(f) => f,
            Err(e) => return Err(e),
        };
        println!("Got me an f-ing f {:?}",f);
        Ok(UserId{})
    }
    pub fn find_user_short(username: &str) -> Result<UserId, std::io::Error> {
        let f = std::fs::File::open("/etc/passwd")?; 
        println!("Got me an f-ing f {:?}",f);
        Ok(UserId{})
    }
    pub fn find_user_remap(username: &str) -> Result<UserId, String> {
        let f = std::fs::File::open("/etc/passwd")
            .map_err(|e| format!("Failed to open password file: {:?}", e))?;
        println!("Got me an f-ing f {:?}",f);
        Ok(UserId{})
    }
    struct InputData {
        payload: Option<Vec<u8>>,
    }
    impl InputData {
    pub fn encrypted(&self) -> Vec<u8> {
            self.payload.as_ref().unwrap_or(&vec![]).clone()
        }
    }
}
// Prefer idiomatic arror variants
mod item_4 {
    // Minimal Errors
    pub struct UserId;
    pub fn find_user(username: &str) -> Result<UserId, String> {
        let f = std::fs::File::open("/etc/passwd")
            .map_err(|e| format!("Failed to open password file: {:?}", e))?;
        Ok(UserId{})
    }
    // We might like to impl Error for String{} but we can't because of Orphan ruloe
    // Type alias doesn't help eaither since it is only an alias not a new type
    // pub type MyError = String;
    //impl std::error::Error for MyError {}
    

    #[derive(Debug)]
    pub struct MyError(String);

    impl std::fmt::Display for MyError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.0)
        }
    }

    impl std::error::Error for MyError {}

    pub fn find_useri_too(username: &str) -> Result<UserId, MyError> {
        let f = std::fs::File::open("/etc/passwd").map_err(|e| {
            MyError(format!("Failed to open password file: {:?}", e))
        })?;
        Ok(UserId{})
    }
    // From<String> lets the compiler automatically apply any relevant From tain implementations
    // needed to get to the desired type
    impl std::convert::From<String> for MyError {
        fn from(msg: String) -> Self {
            Self(msg)
        }
    }
    pub fn find_useri_trie(username: &str) -> Result<UserId, MyError> {
        let f = std::fs::File::open("/etc/passwd")
            .map_err(|e| format!("Failed to open password file: {:?}", e))?;
        Ok(UserId{})
    }
    // Nested Errors
    pub mod nested_errors { 
        #[derive(Debug)]
        pub enum MyError {
            Io(std::io::Error),
            Utf8(std::string::FromUtf8Error),
            General(String),
        }
        impl std::fmt::Display for MyError {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    MyError::Io(e) => write!(f, "IO error: {}", e),
                    MyError::Utf8(e) => write!(f, "UTF-8 error: {}", e),
                    MyError::General(s) => write!(f, "General error: {}", s),
                }
            }
        }
        use std::error::Error;

        impl Error for MyError {
            fn source(&self) -> Option<&(dyn Error + 'static)> {
                match self {
                    MyError::Io(e) => Some(e),
                    MyError::Utf8(e) => Some(e),
                    MyError::General(_) => None,
                }
            }
        }
        impl From<std::io::Error> for MyError {
            fn from(e: std::io::Error) -> Self {
                Self::Io(e)
            }
        }
        impl From<std::string::FromUtf8Error> for MyError {
            fn from(e: std::string::FromUtf8Error) -> Self {
                Self::Utf8(e)
            }
        }
        const MAX_LEN: usize = 32;
        use std::io::BufRead;
        pub fn first_line(filename: &str) -> Result<String, MyError> {
            let file = std::fs::File::open(filename)?; // via `From<std::io::Error>`
            let mut reader = std::io::BufReader::new(file);
            let mut buf = vec![];
            let len = reader.read_until(b'\n', &mut buf)?; // via `From<std::io::Error>`
            let result = String::from_utf8(buf)?; // via `From<std::string::FromUtf8Error>`
            if result.len() > MAX_LEN {
                return Err(MyError::General(format!("Line too long: {}", len)));
            }
            Ok(result)
        }
    }
}
pub mod item_5 {
    pub mod copy {
        pub fn copy_semantics(){
            #[derive(Debug, Clone, Copy)]
            struct KeyId(u32);
            let k = KeyId(42);
            let k2 = k; // value bitwise copied from k to k2
            println!("k={:?}", k);
        }
    }
    pub mod default {
        #[derive(Default)]
        struct Color {
            red: u8,
            green: u8,
            blue: u8,
            alpha: u8,
        }
        pub fn default_semantics() {
            let c = Color {
                red: 128,
                ..Default::default()
            };
        }
    }
    // No example code, just see item I guess
}
// Understand type conversions
pub mod item_6 {
    pub fn no_automagic_conversions() {
        let x: u32 = 2;
        // Won't work even though u64 is guaranteed to hold any u32
        //let y: u64 = x;
        let y: u64 = x.into();
    }
    pub mod user_defined_type_conversions {
        // Implement only the Try[From|Into] trait if a conversion can fail
        // Implement the From trait for conversions since Into is automatically implemented in
        // terms of From
        // Use Into as a trait bound rather than from since Into is available if From is
    }
}
