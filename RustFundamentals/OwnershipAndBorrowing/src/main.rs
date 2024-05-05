// Ownership and Borrowing

fn main() {
    let mut msg1 = String::from("Hello, World!");
    println!("{}", msg1);
    ////////////////////////// let msg2 = msg1;
    ////////////////////////// println!("{}", msg1); 
    //     error[E0382]: borrow of moved value: `msg1`
    //  --> main.rs:7:20
    //   |
    // 4 |     let msg1 = String::from("Hello, World!");
    //   |         ---- move occurs because `msg1` has type `String`, which does not implement the `Copy` trait
    // 5 |     println!("{}", msg1);
    // 6 |     let msg2 = msg1;
    //   |                ---- value moved here
    // 7 |     println!("{}", msg1);
    //   |                    ^^^^ value borrowed here after move
    //   |
    //   = note: this error originates in the macro `$crate::format_args_nl` which comes from the expansion of the macro `println` (in Nightly builds, run with -Z macro-backtrace for more info)
    // help: consider cloning the value if the performance cost is acceptable 

    // The message1 variable is stored on the stack, but the String value itself is stored on the heap.  The "content" of message1 is therefore a memory address that points to the correct location on the heap that contains the actual String data.  The important concept to grasp is that message1 is the "data owner".

    // When we assign the value of message1 to message2, message1 ceases to become the data owner and can no longer reference that heap memory.  The message2 variable has become the new owner for that data.

    // There may be instances where we genuinely need to reference the same data across multiple variables.  There are some hacky ways to do it, such as copying or cloning the data.  But this doesn't work well if the data then changes.  This is where borrowing comes into play.
    // let msg3 = &msg1;
    // println!("msg1 lives at :{:p} \nand has the value: \'{}\'",msg3,msg3);
    let msg4 = &mut msg1;
    *msg4 = String::from("Yolo Bronkey");
    println!("{msg1}"); // Commenting one of these will work
    // println!("{msg4}"); // Commenting one of these will work
}
