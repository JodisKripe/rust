// In Rust nomenclature, a "crate" is anything that the Rust compiler spits out, regardless of whether it's an EXE or DLL.  
// Although if you wrote a program in Rust, you wouldn't necessarily refer to it as a Rust crate.  
// You would probably just call it an app or application.

// The term crate is more often used to refer to libraries, which you'll understand as we go into modules and dependencies.
mod calc;
use calc::sub;
// These can also be used
// use calc::{add,sub};
// use calc::*:

// Dependencies
/// The Rust Package Registry is a community-driven repository for Rust crates.  
/// You can write and publish your own crates, or use existing crates as dependencies in your project.  
/// There is a lot of useful functionality that is not included in the Rust standard library - which leaves you with two choices:  
/// write it yourself, or find an existing crate.
// One of the most downloaded crates is rand, which allows you to generate random numbers.  
// You can add it to your project in one of two ways.
// The first is to use the cargo command line utility.
// cargo add rand

// Other is to add 
// 
// [dependecies]
// rand = "0.8.5"
// 
// to the Cargo.toml manually.

use rand::random;

fn main() {
    let mut result = calc::add(3,4);
    println!("{result}");
    result = sub(3,4);
    println!("{result}");
    let randomNumber = random::<u32>();
    println!("Random Number Is: {}",randomNumber);
}
