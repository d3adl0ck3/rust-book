pub fn all() {
    println!("This is chapter 3!");
    item_1::mismatched_types();
    item_1::tuple_struct();
    item_1::basic_enum();
    item_1::print_page_blah(true,false);
    item_1::print_page_now();
    item_1::enum_match();
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

}
