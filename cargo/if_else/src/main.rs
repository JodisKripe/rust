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
    // Labeled Loop
    let mut count= 0;
    'counting_up: loop{
        println!("count: {}",count);
        let mut remaining =10;

        loop{
            println!("remaining: {}",remaining);
            if remaining==9 {
                break;
            }
            if count==2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count+=1;
    }
    
    let mut num=5;
    let result = loop{
        if num<10{
            num+=1;
        }
        else{break num * 2;}
    };
    println!("value of num: {}, \nValue of result: {}", num,result);
}
