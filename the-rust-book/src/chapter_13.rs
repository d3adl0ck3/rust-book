
pub fn all() {
    shirt_company_giveaway();
    try_expensive_closure();
    fn_versus_closure_syntax();
    capturing_references_or_moving_ownership();
    moving_captured_values_out_of_functions_and_the_fn_traits();
    processing_a_series_of_items_with_iterators();
}


#[derive(Debug,PartialEq, Copy, Clone)]
enum ShirtColor {
    Red, 
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>)->ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {

            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }
        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}
fn shirt_company_giveaway() {
   let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref1, giveaway1
    );

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref2, giveaway2
    ); 
}

fn try_expensive_closure() {
    use std::time::Duration;
    let expensive_closure = |num: u32| -> u32 {
        println!("calculating slowly...");
        std::thread::sleep(Duration::from_millis(1));
        num
    }; 
    expensive_closure(23);
}
fn fn_versus_closure_syntax() {
    fn  add_one_v1   (x: u32) -> u32 { x + 1 }
    let add_one_v2 = |x: u32| -> u32 { x + 1 };
    let add_one_v3 = |x|             { x + 1 };
    let add_one_v4 = |x|               x + 1  ;

    // These are required so the types can be inferred from the usage
    let result = add_one_v3(2);
    let result = add_one_v4(2);

    let example_closure = |x| x;

    let s = example_closure(String::from("hello"));
    // Cannot now try using the same closure on a different type
    //let n = example_closure(5);
}
fn capturing_references_or_moving_ownership() {
    fn capturing_reference() {
        let list = vec![1, 2, 3];
        println!("Before defining closure: {:?}", list);

        let only_borrows = || println!("From closure: {:?}", list);

        println!("Before calling closure: {:?}", list);
        only_borrows();
        println!("After calling closure: {:?}", list);
    }
    fn capturing_mutably() {
        let mut list = vec![1, 2, 3];
        println!("Before defining closure: {:?}", list);

        let mut borrows_mutably = || list.push(7);
        //println!("This won't work because list is mutably borrowed by the closure {:?}",list);
        borrows_mutably();
        println!("After calling closure: {:?}", list);
    }

    fn send_to_other_thread() {
        let list = vec![1, 2, 3];
        println!("Before defining closure: {:?}", list);

        std::thread::spawn(move || println!("From thread: {:?}", list))
            .join()
            .unwrap();
    }

    capturing_reference();
    capturing_mutably();
    send_to_other_thread();
}

fn moving_captured_values_out_of_functions_and_the_fn_traits() {
    /*
     *
    impl<T> Option<T> {
        pub fn unwrap_or_else<F>(self, f: F) -> T
        where
            F: FnOnce() -> T
        {
            match self {
                Some(x) => x,
                None => f(),
            }
        }
    }
     *
     */
    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }
    fn using_sort_by_key() {
        let mut list = [
            Rectangle { width: 10, height: 1 },
            Rectangle { width: 3, height: 5 },
            Rectangle { width: 7, height: 12 },
        ];

        list.sort_by_key(|r| r.width);
        println!("{:#?}", list);
    }
    using_sort_by_key();
}
fn processing_a_series_of_items_with_iterators() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();
    for val in v1_iter {
        println!("Got: {}",val);
    }
    /*
       pub trait Iterator {
        type Item;

        fn next(&mut self) -> Option<Self::Item>;

        // methods with default implementations elided
        }
    */
}
fn methods_that_produce_other_iterators() {


}
#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}
fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn iterator_demostration() {
        let v1 = vec![1, 2, 3];
        let mut v1_iter = v1.iter();
        assert_eq!(v1_iter.next(), Some(&1));
        assert_eq!(v1_iter.next(), Some(&2));
        assert_eq!(v1_iter.next(), Some(&3));
        assert_eq!(v1_iter.next(), None);
    }
    #[test]
    fn iterator_sum() {
        let v1 = vec![1, 2, 3];

        let v1_iter = v1.iter();

        let total: i32 = v1_iter.sum();

        assert_eq!(total, 6);
    }
    #[test]
    fn warn_does_nothing() {
        let v1: Vec<i32> = vec![1, 2, 3];

        v1.iter().map(|x| x + 1);
    }
    #[test]
    fn map_example() {
        let v1: Vec<i32> = vec![1, 2, 3];

        let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

        assert_eq!(v2, vec![2, 3, 4]);
    }
    #[test]
    fn filters_by_size() {
        let shoes = vec![
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe{
                size: 13,
                style: String::from("sandal"),
            },
            Shoe{
                size: 10,
                style: String::from("boot"),
            },
        ];
        let in_my_size = shoes_in_size(shoes, 10);

        assert_eq!(
            in_my_size,
            vec![
                Shoe {
                    size: 10,
                    style: String::from("sneaker")
                },
                Shoe {
                    size: 10,
                    style: String::from("boot")
                },
            ]
        );
    }
}

