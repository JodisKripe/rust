use std::thread;
use std::sync::mpsc; // Multi Producer, Single Consumer

fn main() {
    thread::spawn(|| {
        println!("Inside a thread!!");
    });

    let handle = thread::spawn(|| {
        println!("Also inside a thread!!");
    });

    println!("Not inside a thread!!");

    let result = handle.join();
    match result{
        Ok(_) => println!("Joined!"),
        Err(e) => println!("Nope")
    }
    println!("In the main loop!");
    let homie:i32 = 44;
    let some:i32 = 55;
    let handle2 = thread::spawn( move || -> i32{     // move is used here so that the closure owns the variable some and homie and not just borrows it since the closure may run even after the main function is stopped, which is currently holding the ownership of the variable.
        let temp = homie + some;
        temp * 2
    });
    let result2 = handle2.join();
    match result2{
        Ok(r) => println!("{}",r),
        Err(_) => println!("Oops")
    }

    //// Channels
    let (s,r) = mpsc::channel();
    let h = thread::spawn( move || {
        let _ = s.send("Hello From thread!");
    });
    h.join();
    match r.recv() {
        Ok(m) => println!("Yes, {}",m),
        Err(_) => println!("ERRAUR")
    }

}
