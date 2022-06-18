use std::io;

fn main() {
    let mut x = String::new();
    io::stdin()
        .read_line(&mut x)
        .expect("Error");
    let x:i16 = x.trim().parse().expect("The Value is not valid");
    if x<7{
        println!("The value of x is {}, its less than 7",x);
    }
    else if x==7{
        println!("The Value of x is {}, its equal to 7",x);
    }
    else{
        println!("The value of x is {}, its greater than 7",x);
    }
}
