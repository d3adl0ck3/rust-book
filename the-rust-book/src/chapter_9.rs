// Is you delete the hello.txt file or run the command somewhere it is not present
// you will get a panic on the first method that emits one.  To run through other methods
// you will need to comment out the prior ones that panic to get to the later ones
use std::{error::Error,fs::{self,File}};
use std::io::{self,ErrorKind,Read};
pub fn all() {
    // unrecoverable errors with panic
    // panic sucks don't use it
    recoverable_errors_with_result();
    matching_on_different_errors();
    alternatives_to_using_match_with_result();
    shortcuts_for_panic_on_error_unwap_and_expect();
    propagating_errors();
    creating_custom_types_for_validation();
}
fn recoverable_errors_with_result() {

   let greeting_file_result = File::open("hello.txt");
   // if we try to use this it could panic depending how we do it
   println!("{:?}",greeting_file_result);
}
fn matching_on_different_errors() {
    let greeting_file_result = File::open("hello.txt");

    let _greeting_file = match greeting_file_result {
        Ok(file) => {
            println!("Ok got file!");
            file
        },
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            }
        },
    };
}
fn alternatives_to_using_match_with_result() {
    let _greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
}

fn shortcuts_for_panic_on_error_unwap_and_expect() {
    let _greeting_file = File::open("hello.txt").unwrap();
    let _greeting_file = File::open("hello.txt")
        .expect("hello.txt should be included in this project");
}
fn propagating_errors() {
    fn read_username_from_file() -> Result<String, io::Error> {
        let username_file_result = File::open("hello.txt");
        let mut username_file = match username_file_result {
            Ok(file)=>file,
            Err(e) => return Err(e),
        };
        let mut username = String::new();
        match username_file.read_to_string(&mut username) {
            Ok(_) => Ok(username),
            Err(e) => Err(e),
        }
    }
    fn read_username_from_file_short() -> Result<String, io::Error>{
        let mut username_file = File::open("hello.txt")?;
        let mut username = String::new();
        username_file.read_to_string(&mut username)?;
        Ok(username)
    }
    fn read_username_from_file_shorter() -> Result<String, io::Error>{
        let mut username = String::new();
        File::open("hello.txt")?.read_to_string(&mut username)?;
        Ok(username)
    }
    fn read_username_from_file_shortest() -> Result<String, io::Error>{
        fs::read_to_string("hello.txt")
    }
    fn where_the_qm_operator_can_be_used() {
        // Not allowed since we return () not Result<File,std::io::Error>
        // let greeting_file = File::open("hello.txt")?;
    }
    fn last_char_of_first_line(text: &str) -> Option<char> {
     text.lines().next()?.chars().last()   
    }
    fn what_main_can_return() -> Result<(), Box<dyn Error>> {
        let greeting_file = File::open("helloz.txt")?;
        Ok(())
    }
    fn to_panic_or_not() {
        use std::net::IpAddr;

        let home: IpAddr = "127.0.0.1"
            .parse()
            .expect("Hardcoded IP address should be valid");   
        println!("{:?}",home);
    }

    let result = read_username_from_file();
    println!("{:?}",result);
    let result = read_username_from_file_short();
    println!("{:?}",result);
    let result = read_username_from_file_shorter();
    println!("{:?}",result);
    let result = read_username_from_file_shortest();
    println!("{:?}",result);
    let result = what_main_can_return();
    println!("{:?}",result);
    to_panic_or_not();
}
fn creating_custom_types_for_validation() {
    #[derive(Debug)]
    pub struct Guess {
        value: i32,
    }
    impl Guess {
        pub fn new(value: i32) -> Guess {
            if value < 1 || value > 100 {
                panic!("Guess value must be between 1 and 100, got {}.", value);
            }
            Guess {value }
        }
        pub fn value(&self) -> i32 {
            self.value
        }
    }

    let guess = Guess::new(50);
    println!("guess {:?}",guess);
}
