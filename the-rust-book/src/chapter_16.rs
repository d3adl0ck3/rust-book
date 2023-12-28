pub fn all() {
    creating_a_new_thread_with_spawn::try_it();
    println!("Moving on to tryit with join");
    creating_a_new_thread_with_spawn::try_with_join();
    using_move_closures_with_threads::try_it();
    using_message_passing_to_transfer_data_between_threads::try_it();
    using_message_passing_to_transfer_data_between_threads::sending_multiple_values_and_seeing_the_receiver_waiting();
    using_message_passing_to_transfer_data_between_threads::creating_multiple_producers_by_cloning_the_transmitter();
    shared_state_concurrency::the_api_of_mutex_t();
    shared_state_concurrency::sharing_a_mutex_between_multiple_threads();
}
mod creating_a_new_thread_with_spawn {
    use std::thread;
    use std::time::Duration;
    pub fn try_it() {
        thread::spawn(|| {
            for i in 1..10 {
                println!("hi number {} from the spawned thread!", i);
                thread::sleep(Duration::from_millis(1));
            }
        });

        for i in 1..5 {
            println!("hi number {} from the main thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    }
    pub fn try_with_join() {
        let handle = thread::spawn(|| {
            for i in 1..10 {
                println!("hi number {} from the spawned thread!", i);
                thread::sleep(Duration::from_millis(1));
            }
        });
        // Try commenting out the later one and put this one in
        // handle.join().unwrap(); 
        for i in 1..5 {
            println!("hi number {} from the main thread!", i);
            thread::sleep(Duration::from_millis(1));
        }

        handle.join().unwrap();
    }
}
mod using_move_closures_with_threads {
    use std::thread;
    pub fn try_it() {
        let v = vec![1,2,3];

        let handle = thread::spawn(move ||{
            println!("Here's a vector: {:?}", v);
        });

        handle.join().unwrap();
    }
}
mod using_message_passing_to_transfer_data_between_threads {
    // multiple producer single consumer
    use std::sync::mpsc;
    use std::thread;
    pub fn try_it() {
        let (tx, rx) = mpsc::channel();
        thread::spawn(move || {
            let val = String::from("hi");
            // unwrap to panic on error - real world should be smarter
            tx.send(val).unwrap();
            // this won't work since the ownership is passed
            // println!("val is {}", val);
        });

        let received = rx.recv().unwrap();
        println!("Got: {}", received);
    }
    use std::time::Duration;
    pub fn sending_multiple_values_and_seeing_the_receiver_waiting() {
        let (tx, rx) = mpsc::channel();

        thread::spawn(move || {
            let vals = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("thread"),
            ];

            for val in vals {
                tx.send(val).unwrap();
                thread::sleep(Duration::from_secs(1));
            }
        });

        for received in rx {
            println!("Got: {}", received);
        } 
    }
    pub fn creating_multiple_producers_by_cloning_the_transmitter() {
        let (tx, rx) = mpsc::channel();

        let tx1 = tx.clone();
        thread::spawn(move || {
            let vals = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("thread"),
            ];

            for val in vals {
                tx1.send(val).unwrap();
                thread::sleep(Duration::from_secs(1));
            }
        });

        thread::spawn(move || {
            let vals = vec![
                String::from("more"),
                String::from("messages"),
                String::from("for"),
                String::from("you"),
            ];

            for val in vals {
                tx.send(val).unwrap();
                thread::sleep(Duration::from_secs(1));
            }
        });

        for received in rx {
            println!("Got: {}", received);
        }
    }
}
mod shared_state_concurrency {
    // This goes through various permutations trying to use a vanilla Rc instead of Arc
    // here we just have the working result
    use std::sync::{Arc, Mutex};
    pub fn the_api_of_mutex_t() {
        let m = Mutex::new(5);

        {
            let mut num = m.lock().unwrap();
            *num = 6;
        }

        println!("m = {:?}", m);
    }
    use std::thread;
    pub fn sharing_a_mutex_between_multiple_threads() {
       let counter = Arc::new(Mutex::new(0));
       let mut handles = vec![];

       for _ in 0..10 {
           let counter = Arc::clone(&counter);
            let handle = thread::spawn(move ||{
                let mut num = counter.lock().unwrap();
                *num += 1;
            });
            handles.push(handle);
       }

       for handle in handles {
            handle.join().unwrap();
       }

       println!("Result {}", *counter.lock().unwrap());
    }
}
mod extensible_concurrency_with_the_sync_and_send_traits {
    // Nothing to see here!  Basically it just says 'here there be (unsafe) monsters'
}
