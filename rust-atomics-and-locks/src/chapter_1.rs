use std::cell::Cell;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;
use std::sync::Arc;
use std::sync::Condvar;
use std::sync::Mutex;
use std::thread;
use std::time::Duration;

pub fn all() {
    example_1();
    example_2();
    example_3();
    scoped_threads_1();
    static_variables();
    static_box_leak();
    reference_counting_1();
    reference_counting_2();
    arc_scope();
    cell_sharing();
    refcell_sharing();
    mutex();
    if false {
        thread_parking();
    }
    if false {
        cond_var();
    }
}
fn example_1() {
    let one = thread::spawn(f);
    let two = thread::spawn(f);

    println!("Hello from the main thread.");

    // These are required or the program can exit before threads have finished
    one.join().unwrap();
    two.join().unwrap();
}

fn f() {
    println!("Hello from another thread!");

    let id = thread::current().id();
    println!("This is my thread id: {id:?}");
}
fn example_2() {
    let numbers = vec![1, 2, 3];

    thread::spawn(move || {
        for n in &numbers {
            println!("{n}");
        }
    }).join().unwrap();
}
fn example_3() {
    let numbers = vec![1, 2, 3];

    let t = thread::spawn(move || {
        let len = numbers.len();
        let sum = numbers.iter().sum::<usize>();
        sum / len
    });
    let average = t.join().unwrap();
    println!("average: {average}");
}
fn scoped_threads_1() {
    let numbers = vec![1, 2, 3];

    thread::scope(|s| {
        s.spawn(|| {
            println!("length: {}", numbers.len());
        });
        s.spawn(|| {
            for n in &numbers {
                println!("number {n}");
            }
        });
    });

    /*
     * Doesn't work since it would be two mutable references at once!
     * let mut numbers = vec![1, 2, 3];

thread::scope(|s| {
    s.spawn(|| {
        numbers.push(1);
    });
    s.spawn(|| {
        numbers.push(2); // Error!
    });
});
     */
}
fn static_variables() {
    static X: [i32; 3] = [1, 2, 3];

    let one = thread::spawn(|| dbg!(&X));
    let two = thread::spawn(|| dbg!(&X));
    one.join().unwrap();
    two.join().unwrap();
}
fn static_box_leak() {
    let x: &'static [i32; 3] = Box::leak(Box::new([1, 2, 3]));

    let one = thread::spawn(move || dbg!(x));
    let two = thread::spawn(move || dbg!(x));
    one.join().unwrap();
    two.join().unwrap();
}
fn reference_counting_1() {
    let a = Rc::new([1, 2, 3]);
    let b = a.clone();
    // Cannot pass these to threads as they can't be safely moved
    assert_eq!(a.as_ptr(), b.as_ptr()); // Same allocation!
}
fn reference_counting_2() {

    let a = Arc::new([1, 2, 3]);
    let b = a.clone();
    //arcs CAN be moved between threads i.e. they are thread safe
    let one = thread::spawn(move || dbg!(a));
    let two = thread::spawn(move || dbg!(b));
    one.join().unwrap();
    two.join().unwrap();
}
fn arc_scope() {
    
    // Same scope
    let a = Arc::new([1, 2, 3]);

    let b = a.clone();

    let t = thread::spawn(move || {
        dbg!(b);
    });

    dbg!(a);
    t.join().unwrap();

    // Nested scope
    let a = Arc::new([1, 2, 3]);

    let t = thread::spawn({
        let a = a.clone();
        move || {
            dbg!(a);
        }
    });

    dbg!(a);
    t.join().unwrap();
}
fn cell_sharing() {
    fn x() {
    }
    fn f(a: &Cell<i32>, b: &Cell<i32>) {
        let before = a.get();
        b.set(b.get() + 1);
        let after = a.get();
        if before != after {
            x(); // might happen
        }
    }
    fn f2(v: &Cell<Vec<i32>>) {
        let mut v2 = v.take(); // Replaces the contents of the Cell with an empty Vec
        v2.push(1);
        v.set(v2); // Put the modified Vec back
    }

}
fn refcell_sharing() {

fn f(v: &RefCell<Vec<i32>>) {
    v.borrow_mut().push(1); // We can modify the `Vec` directly.
}
}
fn mutex() {
    let n = Mutex::new(0);
    thread::scope(|s| {
        for _ in 0..10 {
            s.spawn(|| {
                let mut guard = n.lock().unwrap();
                for _ in 0..100 {
                    *guard += 1;
                }
                // If we don't drop we hold the lock while sleeping
                drop(guard); // New: drop the guard before sleeping!
                thread::sleep(Duration::from_secs(1));
            });
        }
    });
    assert_eq!(n.into_inner().unwrap(), 1000);
}
fn thread_parking() {
    let queue = Mutex::new(VecDeque::new());

    thread::scope(|s| {
        // Consuming thread
        let t = s.spawn(|| loop {
            let item = queue.lock().unwrap().pop_front();
            if let Some(item) = item {
                dbg!(item);
            } else {
                thread::park();
            }
        });

        // Producing thread
        for i in 0.. {
            queue.lock().unwrap().push_back(i);
            t.thread().unpark();
            thread::sleep(Duration::from_secs(1));
        }
    });
}
fn cond_var() {
    let queue = Mutex::new(VecDeque::new());
    let not_empty = Condvar::new();

    thread::scope(|s| {
        s.spawn(|| {
            loop {
                let mut q = queue.lock().unwrap();
                let item = loop {
                    if let Some(item) = q.pop_front() {
                        break item;
                    } else {
                        q = not_empty.wait(q).unwrap();
                    }
                };
                drop(q);
                dbg!(item);
            }
        });

        for i in 0.. {
            queue.lock().unwrap().push_back(i);
            not_empty.notify_one();
            thread::sleep(Duration::from_secs(1));
        }
    });
}
