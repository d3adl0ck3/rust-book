pub fn all() {
    raw_pointers::dereferenceing_raw_pointers();
    raw_pointers::call_dangerous();
    raw_pointers::using_the_safe_split_at_mut_function();
    extern_functions_to_call_external_code::try_it();
    // Ok we're not really calling it from C but it is close :-)
    extern_functions_to_call_external_code::call_from_c();
    accessing_or_modifying_a_mutable_static_variable::defining_and_using_an_immutable_static_variable();
    accessing_or_modifying_a_mutable_static_variable::reading_from_or_writing_to_a_mutable_static_variable_is_unsafe();
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
