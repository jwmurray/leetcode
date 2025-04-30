use std::sync::{Arc, Condvar, Mutex};
use std::thread;
use std::time::Duration;

struct Foo {
    first_done: (Mutex<bool>, Condvar),
    second_done: (Mutex<bool>, Condvar),
}

impl Foo {
    fn new() -> Self {
        Foo {
            first_done: (Mutex::new(false), Condvar::new()),
            second_done: (Mutex::new(false), Condvar::new()),
        }
    }

    fn first(&self) {
        println!("first");
        let (lock, cvar) = &self.first_done;
        *lock.lock().unwrap() = true;
        cvar.notify_one();
    }

    fn second(&self) {
        let (lock, cvar) = &self.first_done;
        let mut guard = lock.lock().unwrap();
        while !*guard {
            guard = cvar.wait(guard).unwrap();
        }
        println!("second");
        let (lock, cvar) = &self.second_done;
        *lock.lock().unwrap() = true;
        cvar.notify_one();
    }

    fn third(&self) {
        let (lock, cvar) = &self.second_done;
        let mut guard = lock.lock().unwrap();
        while !*guard {
            guard = cvar.wait(guard).unwrap();
        }
        println!("third");
    }
}

fn main() {
    let foo = Arc::new(Foo::new());

    // Create three threads with different delays to simulate random ordering
    let foo1 = Arc::clone(&foo);
    let t1 = thread::spawn(move || {
        thread::sleep(Duration::from_millis(100));
        foo1.first();
    });

    let foo2 = Arc::clone(&foo);
    let t2 = thread::spawn(move || {
        thread::sleep(Duration::from_millis(50));
        foo2.second();
    });

    let foo3 = Arc::clone(&foo);
    let t3 = thread::spawn(move || {
        thread::sleep(Duration::from_millis(150));
        foo3.third();
    });

    // Wait for all threads to complete
    t1.join().unwrap();
    t2.join().unwrap();
    t3.join().unwrap();
}
