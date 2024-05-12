use std::ops::{Add,Mul};

fn main() {
    let mut x1:u64 = 45;
    let mut x2:u64 = 56;
    let x3:u64 = add_numbers(x1,x2);
    println!("{} {} {}",x3,x1,x2);
    let x5 = x2;
    let x4:u64 = mul_numbers(x1,x5);
    println!("{x4} {x1} {x2}")
}

fn add_numbers<T: Add<Output = T>>(i1:T,i2:T) -> T{
    i1+i2
}

fn mul_numbers<T>(i1:T,i2:T) -> T
where T: Mul<Output = T>{
    i1 * i2
}
