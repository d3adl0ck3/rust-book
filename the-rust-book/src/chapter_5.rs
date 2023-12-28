pub fn all() {
   defining_and_instantiating_structs(); 
   creating_instances_from_other_instances_with_struct_update_syntax();
   using_tuple_structs_without_named_fields_to_create_different_types();
   ownership_of_struct_data();
   an_example_program_using_structs_v1();
   refactoring_with_tuples();
   refactoring_with_structs();
   using_dbg();
   method_syntax();
   methods_with_more_parameters();
   associated_functions();
   multipl_impl_blocks();
}
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
fn defining_and_instantiating_structs() {
    let _user1 = User {
    active: true,
    username: String::from("deadlocke"),
    email: String::from("deadlock0813@gmail.com"),
    sign_in_count: 1,
    };

    // Shadow a mut version
    let mut user1 = User {
    active: true,
    username: String::from("deadlocke"),
    email: String::from("deadlock0813@gmail.com"),
    sign_in_count: 1,
    };
    user1.email = String::from("anotheremail@example.com");

    let _user1 = build_user(String::from("deadlock0813@gmail.com"),String::from("deadlocke"));
    let _user1 = using_the_field_init_shorthand(String::from("deadlock0813@gmail.com"),String::from("deadlocke"));

}
fn build_user(email: String, username: String) -> User {
    User{
        active: true,
        username: username,
        email: email,
        sign_in_count: 1,
    }
}
fn using_the_field_init_shorthand(email: String, username: String) -> User {
    User{
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}
fn creating_instances_from_other_instances_with_struct_update_syntax() {
    let user1 = using_the_field_init_shorthand(String::from("stpuid@email"),String::from("deadlocke"));
    let _user2 = User{
        email: String::from("override@email.com"),
        ..user1};
    // Because we stole the username move only field from user1 we cannot use user1 any longer
}
fn using_tuple_structs_without_named_fields_to_create_different_types() {
   let _black = Color(0,0,0);
   let _origin = Point(0,0,0);
}
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn unit_like_structs_without_any_fields() {
    let _subject = AlwaysEqual;
}
struct AlwaysEqual;

fn ownership_of_struct_data() {
    // Doesn't work without lifetime specifiers
    /*
     * struct User {
     *  active: bool,
     *  username: &str,
     *  email: &str,
     *  sign_in_count: u64,
     * }
     * fn main() {
     *  let user1 = User {
     *      active: true,
     *      username: "someusername123",
     *      email: "someone@example.com",
     *      sign_in_count: 1,
     *  }
     *
     */
}

fn an_example_program_using_structs_v1() {
    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels",
        area_v1(width1,height1)
        );
}
fn area_v1(width: u32, height: u32) -> u32 {
    width * height
}

fn refactoring_with_tuples() {
    let rect1 = (30, 50);
    println!(
        "The area of the rectangle is {} square pixels",
        area_tuple(rect1)
    );
}
fn area_tuple(dimensions: (u32, u32)) -> u32 {
    return dimensions.0 * dimensions.1;
}

fn refactoring_with_structs() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    // Possible because we have derive(Debug)
    println!("debug rect1 is {:?}",rect1);
    println!("pretty rect1 is {:#?}",rect1);

    println!(
            "The area of the rectangle is {} square pixels.",
            area_struct(&rect1)
        );
}
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}
fn area_struct(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
fn using_dbg() {
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    dbg!(&rect1);
}

fn method_syntax() {
    let rect1 = Rectangle{
        width: 30,
        height: 50,
    };
    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
    if rect1.width() {
        println!("The rectange gas a nonzero width; it is {}",rect1.width);
    }
}
impl Rectangle {
    // Functions here defined on Rectangle are Associated Functions
    // When some form of self is the first argument they are methods
    // when the first argument is not self they are just associated functions often used for
    // factories
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn width(&self) -> bool {
       self.width > 0 
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    // This is a factory that just takes a size and creates a square
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}
fn methods_with_more_parameters() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}
fn associated_functions() {
    let sq = Rectangle::square(3);
    println!("Created square {:?}",sq);
}
// Multiple impl blocks

impl Rectangle {
    fn area_too(&self)->u32 {
        self.width * self.height
    }
}
fn multipl_impl_blocks() {
    let sq = Rectangle::square(4);
    println!("sq {:?} area_too {}",sq,sq.area_too());
}
