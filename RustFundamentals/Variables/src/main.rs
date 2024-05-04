//  Variables

fn main() {
    // `let`
    // inference works in rust
    const _MY_NAME: &str = "JodisKripe";

    // Rust uses different text casing depending on the type of object being declared.  Here is a summary of the conventions:

    // Object	Casing
    // Variables	snake_case
    // Functions	snake_case
    // Constants	SCREAMING_SNAKE_CASE
    // Types	PascalCase
    // Traits	PascalCase
    // Enums	PascalCase

    let integer : i8 = 65;
    let uint8 : u8 = integer as u8;
    let uinteger : u16 = integer as u16;

    let uint : u16 = 60000;
    let int8 : i8 = uint as i8; // Data Lost
    println!("{} {}",uinteger,int8);

    let somechar:char = uint8 as char;
    println!("{}",somechar);

    //Mutability
    // Variables in rust are immutable by default

    let mut someint:u8 = 255;
    someint = 128;
    println!("{}",someint);

    let mut arr:[i32;5] = [1,2,3,4,5];
    arr[3] = 77;
    println!("{}",arr[3]); 

}
