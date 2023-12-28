pub fn all() {
    println!("Chapter 10!");
    removing_duplication_by_extracting_a_function();
    generic_data_types();
    in_struct_definitions();
    traits_defining_shared_behavior();
    preventing_dangling_references_with_lifetimes();
    lifetime_annotations_in_struct_definitions();
    the_kit_and_caboodle();
}
fn removing_duplication_by_extracting_a_function() {
    fn initial() {
        let number_list = vec![24,50,25,100,65];
        let mut largest = &number_list[0];
        for number in &number_list {
            if number > largest {
                largest = number;
            }
        }
        println!("The largest number is {}",largest);
    }
    fn duplicated_like_crazy() {
        let number_list = vec![34, 50, 25, 100, 65];

        let mut largest = &number_list[0];

        for number in &number_list {
            if number > largest {
                largest = number;
            }
        }

        println!("The largest number is {}", largest);

        let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

        let mut largest = &number_list[0];

        for number in &number_list {
            if number > largest {
                largest = number;
            }
        }

        println!("The largest number is {}", largest);
    }
    fn largest(list: &[i32]) -> &i32 {
        let mut largest = &list[0];
        for item in list {
            if item > largest {
                largest = item;
            }
        }
        largest
    }
    fn using_largest() {
        let number_list = vec![34, 50, 25, 100, 65];
        let result = largest(&number_list);
        println!("The largest number is {}",result);

        let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];
        let result = largest(&number_list);
        println!("The largest number is {}",result);
    }
    initial();
    duplicated_like_crazy();
    using_largest();
}
fn generic_data_types() {
    fn largest_i32(list: &[i32]) -> &i32 {
        let mut largest = &list[0];

        for item in list {
            if item > largest {
                largest = item;
            }
        }

        largest
    }

    fn largest_char(list: &[char]) -> &char {
        let mut largest = &list[0];

        for item in list {
            if item > largest {
                largest = item;
            }
        }

        largest
    }

    fn non_generic() {
        let number_list = vec![34, 50, 25, 100, 65];

        let result = largest_i32(&number_list);
        println!("The largest number is {}", result);

        let char_list = vec!['y', 'm', 'a', 'q'];

        let result = largest_char(&char_list);
        println!("The largest char is {}", result);
    }
    fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
        let mut largest = &list[0];

        for item in list {
            if item > largest {
                largest = item;
            }
        }

        largest
    }
    fn generic() {
        let number_list = vec![34, 50, 25, 100, 65];

        let result = largest(&number_list);
        println!("The largest number is {}", result);

        let char_list = vec!['y', 'm', 'a', 'q'];

        let result = largest(&char_list);
        println!("The largest char is {}", result);
    }
    generic();
}
fn in_struct_definitions(){
    struct Point<T> {
        x: T,
        y: T,
    }
    impl<T> Point<T> {
        fn x(&self) -> &T {
            &self.x
        }
    }
    impl Point<f32> {
        fn distance_from_origin(&self) -> f32 {
            (self.x.powi(2) + self.y.powi(2)).sqrt()
        }
    }
    fn try_it() {
        let integer = Point { x: 5, y: 10 };
        println!("integer.x = {}",integer.x());
        let float = Point { x: 1.0, y: 4.0 };
        // This won't work because x and y must be the same type
        // let wont_work = Point { x: 1, y: 4.0 }
        println!("distance from origin {}",float.distance_from_origin());
    }
    fn mixup_types (){
        struct Point<X1,Y1> {
            x: X1,
            y: Y1,
        }
        impl<X1, Y1> Point<X1, Y1> {
            fn mixup<X2,Y2>(self, other: Point<X2,Y2>)->Point<X1,Y2> {
                Point {
                    x: self.x,
                    y: other.y,
                }
            }
        }
        let p1 = Point {x: 5, y: 10.4};
        let p2 = Point { x: "Hello", y: 'c' };
    
        let p3 = p1.mixup(p2);
        println!("p3.x = {}, p3.y = {}",p3.x,p3.y);
    }
    try_it();
    mixup_types();
}
fn traits_defining_shared_behavior() {
    pub trait Summary {
        fn summarize(&self) -> String;
        fn summarize_default(&self) -> String {
            String::from("(Read More...)")
        }
    }
    pub struct NewsArticle{
        pub headline: String,
        pub location: String,
        pub author: String,
        pub content: String,
    }
    impl Summary for NewsArticle {
        fn summarize(&self) -> String {
            format!("{}, by {} ({})",self.headline, self.author, self.location)
        }
    }
    pub struct Tweet {
        pub username: String,
        pub content: String,
        pub reply: bool,
        pub retweet: bool,
    }
    impl Summary for Tweet {
        fn summarize(&self) -> String {
            format!("{}: {}",self.username, self.content)
        }
    }
    fn doit() {
        let tweet = Tweet{
            username: String::from("horse_ebooks"),
            content: String::from(
                "of course, as you probably already know, people",
                ),
                reply: false,
                retweet: false,
        };
        println!(" 1 new tweet: {}", tweet.summarize());
        println!(" 1 new tweet: default summarize {}",tweet.summarize_default());
        notify(&tweet);
        notify_long(&tweet);
    }
    // these are equivalent
    fn notify(item: &impl Summary) {
        println!("Breaking new! {}",item.summarize());
    }
    fn notify_long<T: Summary>(item: &T) {
        println!("Breaking new! {}",item.summarize());
    }
    fn notify_any_types(item1: &impl Summary, item2: &impl Summary) {
    }
    fn notify_same_types<T: Summary>(item1: &T, item2: &T) {
    }
    use std::fmt::Display;
    use std::fmt::Debug;
    fn notify_multiple_bounds(item: &(impl Summary + Display)) {
    }
    fn notify_multiple_bounds_too<T: Summary + Display>(item: &T) {
    }
    fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
        7
    }
    fn returns_summarizable() -> impl Summary {
        Tweet {
            username: String::from("horse_ebooks"),
            content: String::from(
                "of course, as you probably already know, people",
                ),
                reply: false,
                retweet: false,
        }
    }
    doit();

}
fn using_trait_bounds_to_condistionally_implement_methods() {
    use std::fmt::Display;

    struct Pair<T> {
        x: T,
        y: T,
    }
    impl<T> Pair<T> {
        fn new (x: T, y: T) -> Self {
            Self{ x, y }
        }
    }
    impl<T: Display + PartialOrd> Pair<T> {
        fn cmp_display(&self) {
            if self.x >= self.y {
                println!("The largest member is x = {}", self.x);
            } else {
                println!("The largest member is y = {}", self.y);
            }
        }
    }
}
fn preventing_dangling_references_with_lifetimes() {
    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }
    fn longest_first_param_always_returned<'a>(x: &'a str, y: &str) -> &'a str {
        x
    }
    /*
    fn longest_lifetime_not_related_to_param<'a>(x: &str, y: &str) -> &'a str {
        let result = String::from("really long string");
        result.as_str()
    }
    */
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);
    println!("longest {}",longest("Hi there","1.21 Gigawatts"));
}
fn lifetime_annotations_in_struct_definitions() {
    #[derive(Debug)]
    struct ImportantExcerpt<'a> {
        part: &'a str,
    }
    impl <'a> ImportantExcerpt<'a> {
        fn announce_and_return_part(&self, announcement: &str) -> &str {
            println!("Attention please: {}", announcement);
                self.part
        }
    }
    fn main() {
        let novel = String::from("Call me Ishmael. Some years ago...");
        let first_sentence = novel.split('.').next().expect("Could not find a '.'");
        let i = ImportantExcerpt {
            part: first_sentence,
        };
        println!("important {:?}",i);
    }
    fn lifetime_elision() {
        // One reference in and one out so can deduce they have the same lifetime
        fn first_word(s: &str) -> &str {
            let bytes = s.as_bytes();
            for (i, &item) in bytes.iter().enumerate() {
                if item == b' ' {
                    return &s[0..i];
                }
            }

            &s[..]
        }
    }
    fn lifetime_annotations_in_method_definitions() {

    }
    // This &str lives for the life of the program
    let s: &'static str = "I have a static lifetime.";
    main();
}
fn the_kit_and_caboodle() {
    use std::fmt::{self,Display};
    fn longest_with_an_announcement<'a, T>(
        x: &'a str,
        y: &'a str,
        ann: T,
    ) -> &'a str
    where
        T: Display,
    {
        println!("Announcement! {}", ann);
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }
    #[derive(Clone,Copy)]
    struct MyDisplayable<'a> {
        value: &'a str,
    }
    impl<'a> Display for MyDisplayable<'a> {
        fn fmt(&self, f: &mut fmt::Formatter) -> std::fmt::Result {
            write!(f,"value is {}",self.value)
        }
    }
    longest_with_an_announcement("blah","double blah",MyDisplayable{value:"display me fooo"});
}
