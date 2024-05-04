//Control Flow

use std::env::args;
use std::io;
use std::io::Write;
enum Status{
    Online,
    Offline
}

fn main() {
    let b1 = true;
    let b2 = true;
    let b3 = false;

    if b2 || b1 && b3{
        println!("b2 || b1 && b3");
    }
    if (b1 || b3) && b2{
        println!("(b1 || b3) && b2");
    }
    else{
        println!("UGH!");
    }

    let animal = "Dog";
    match animal{
        "Dog" => println!("Woof"),
        "Cat" => println!("Meow"),
        _ => println!("Unknown")
    }

    //Enums
    let some = Status::Offline;
    match some{
        Status::Online => println!("yoda"),
        Status::Offline => println!("licious")
    }

    //Loops
    for i in 1..10{
        println!("{}",i);
    }

    let mut counter =10;
    while counter > 0{
        println!("{}",counter);
        counter-=1;
    }
    loop{
        if counter == 5 {
            break;
        }
        println!("{}",counter);
        counter+=1;
    }

    //Functions
    println!("{}",add_ints(7,8));
    println!("{}",short_add(16,8));

    //Command Line Arguements
    let args:Vec<String> = args().collect();
    for arg in args.iter(){
        println!("{}",arg);
    }

    // Inputs
    loop{
        print!("> ");
        let _ = io::stdout().flush();
        let mut input = String::new();
        let _ = io::stdin().read_line(&mut input);
        if input.trim_end().eq_ignore_ascii_case("exit"){
            break;
        }
        println!("You said {}",input);
    }

    let clos = ||{
        println!("Clos");
    };
    clos();

    let smh = |message:&str| -> String{
        println!("{}",message);
        String::from(message)
    };

    let yum = smh("Yoda");
    println!("{}",yum);

    {
        let lmao = "yoohoo";
    }

    println!("{}",yoohoo);
}

fn add_ints(i1:u8,i2:u8) -> u8{
    return i1 + i2;
}

fn short_add(i1:u8,i2:u8) -> u8{
    i1+i2
}
