use std::io;

fn main() {
    //tuple
    let tup = (500, 0.6 , 0);
    let five_hundred = tup.0;
    let point_six = tup.1;
    let nill = tup.2;
    println!("The values of five_hundred, point_six and nill are: {}, {} and {}",five_hundred, point_six, nill);
    //Array
    let arr: [u32;5] = [4,5,6,7,8];
    let mut index = String::new();
    io::stdin()
        .read_line(&mut index)
        .expect("Couldn't read line");
    let index:usize= match index.trim().parse(){
        Ok(num) => num,
        Err(_) => 0
    };
    println!("Value of element index {} is {}",index, arr[index]);
    //Structs(?)
    let x:u32=5;
    let y:usize = {
        let x = 7;
        x+5
    };
    println!("Value of x: {} \nValue of y: {}",x,y);
}
