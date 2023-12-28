use std::ops::Deref;
pub fn all() {
    using_box_to_store_data_on_the_heap();
    using_box_to_get_a_recursive_type_with_known_size();
    running_code_cleanup_with_the_drop_trait();
    running_code_cleanup_with_the_drop_trait_early();
    rc_based_list();
    rc_refcell_based_list();
    creating_a_reference_cycle::create_cycle();
    creating_a_tree_data_structure_a_node_with_child_nodes::try_it();
    creating_a_tree_data_structure_a_node_with_child_nodes::visualizing_changes_to_strong_count_and_weak_count();
}

fn using_box_to_store_data_on_the_heap() {
    let b = Box::new(5);
    println!("b = {}", b);
}
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use self::List::{Cons, Nil};

fn using_box_to_get_a_recursive_type_with_known_size() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
}

struct MyBox<T>(T);
impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T>{
    MyBox(x)
    }
}
impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
fn hello(name: &str) {
    println!("Hello, {name}!");
}

struct CustomSmartPointer {
    data: String,
}
impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}
fn running_code_cleanup_with_the_drop_trait() {
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointers created."); 
}

fn running_code_cleanup_with_the_drop_trait_early() {
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    println!("CustomSmartPointer created."); 
    drop(c);
    println!("CustomSmartPointer dropped before the end of main."); 
}
fn rc_based_list() {
    enum List {
        Cons(i32, Rc<List>),
        Nil,
    }

    use std::rc::Rc;

    fn try_it() {
        let a = Rc::new(List::Cons(5, Rc::new(List::Cons(10, Rc::new(List::Nil)))));
        println!("count after creating a = {}", Rc::strong_count(&a));
        let b = List::Cons(3, Rc::clone(&a));
        println!("count after creating b = {}", Rc::strong_count(&a));
        {
            let c = List::Cons(4, Rc::clone(&a));
            println!("count after creating c = {}", Rc::strong_count(&a));
        }
        println!("count after c goes out of scope = {}", Rc::strong_count(&a));
    }
    try_it();
}
fn rc_refcell_based_list() {
    #[derive(Debug)]
    enum List {
        Cons(Rc<RefCell<i32>>, Rc<List>),
        Nil,
    }

    use std::cell::RefCell;
    use std::rc::Rc;

    fn try_it() {
        let value = Rc::new(RefCell::new(5));

        let a = Rc::new(List::Cons(Rc::clone(&value), Rc::new(List::Nil)));

        let b = List::Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
        let c = List::Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

        *value.borrow_mut() += 10;

        println!("a after = {:?}", a);
        println!("b after = {:?}", b);
        println!("c after = {:?}", c);
    }
    try_it();
}
pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    pub fn new(messenger: &'a T, max: usize) -> LimitTracker<'a, T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quota!");
        } else if percentage_of_max >= 0.9 {
            self.messenger
                .send("Urgent warning: You've used up over 90% of your quota!");
        } else if percentage_of_max >= 0.75 {
            self.messenger
                .send("Warning: You've used up over 75% of your quota!");
        }
    }
}
mod creating_a_reference_cycle {
    use std::cell::RefCell;
    use std::rc::Rc;
    use self::List::{Cons, Nil};
    #[derive(Debug)]
    enum List {
        Cons(i32, RefCell<Rc<List>>),
        Nil,
    }

    impl List {
        fn tail(&self) -> Option<&RefCell<Rc<List>>> {
            match self {
                Cons(_, item) => Some(item),
                Nil => None,
            }
        }
    }
    pub fn create_cycle() {

        let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

        println!("a initial rc count = {}", Rc::strong_count(&a));
        println!("a next item = {:?}", a.tail());

        let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

        println!("a rc count after b creation = {}", Rc::strong_count(&a));
        println!("b initial rc count = {}", Rc::strong_count(&b));
        println!("b next item = {:?}", b.tail());

        if let Some(link) = a.tail() {
            *link.borrow_mut() = Rc::clone(&b);
        }

        println!("b rc count after changing a = {}", Rc::strong_count(&b));
        println!("a rc count after changing a = {}", Rc::strong_count(&a));

        // Uncomment the next line to see that we have a cycle;
        // it will overflow the stack
        // println!("a next item = {:?}", a.tail());
    }

}
mod creating_a_tree_data_structure_a_node_with_child_nodes {
    use std::cell::RefCell;
    use std::rc::{Rc, Weak};
    #[derive(Debug)]
    struct Node {
        value: i32,
        parent: RefCell<Weak<Node>>,
        children: RefCell<Vec<Rc<Node>>>,
    }
    pub fn try_it() {
        let leaf = Rc::new(Node{
            value: 3,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![]),
        });
        println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
        let branch = Rc::new(Node{
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });
        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);
        println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    }
    pub fn visualizing_changes_to_strong_count_and_weak_count() {
        let leaf = Rc::new(Node {
            value: 3,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![]),
        });

        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf),
            );

        {
            let branch = Rc::new(Node {
                value: 5,
                parent: RefCell::new(Weak::new()),
                children: RefCell::new(vec![Rc::clone(&leaf)]),
            });

            *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

            println!(
                "branch strong = {}, weak = {}",
                Rc::strong_count(&branch),
                Rc::weak_count(&branch),
                );

            println!(
                "leaf strong = {}, weak = {}",
                Rc::strong_count(&leaf),
                Rc::weak_count(&leaf),
                );
        }

        println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf),
            );
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn following_the_pointer_to_the_value() {
        let x = 5;
        let y = &x;

        assert_eq!(5, x);
        assert_eq!(5, *y);
    }
    #[test]
    fn using_box_t_like_a_reference() {
        let x = 5;
        let y = Box::new(x);

        assert_eq!(5, x);
        assert_eq!(5, *y);
    }
    #[test]
    fn deref_mybox() {

        let x = 5;
        let y = MyBox::new(x);

        assert_eq!(5, x);
        assert_eq!(5, *y);
    }
    #[test]
    fn deref_coercion() {
        let m = MyBox::new(String::from("Rust"));
        hello(&m);
    }
    use std::cell::RefCell;

    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>,
    }
    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: RefCell::new(vec![]),
            }
        }
    }
    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            self.sent_messages.borrow_mut().push(String::from(message));
        }
    }
    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);
        limit_tracker.set_value(80);
        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
    }
}
