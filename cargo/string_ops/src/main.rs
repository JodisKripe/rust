use std::io;

fn main() {
    let mut x = String::from("hello");
    x.push_str(",World");
    let y = x.clone();
    println!("X:{}\nY:{}",x,y);
    let res= RetString("dope".to_string());
    println!("res:\n\n{}",res);
    let Lenght = Length(&"This isn't fun".to_string());
    println!("{}",Lenght);
    println!("{},{}",slicing("Test".to_string(),1,3).0,slicing("Test".to_string(),1,3).1);
}

fn RetString(mut strr:String) -> String{
    strr.push_str("\nEnd SIG\n\n\n");
    strr
}

fn Length(Str:&String) -> usize{
    let bytes = Str.as_bytes();
    for (i,&item) in bytes.iter().enumerate() {
        if item==b' '{
            return i;
        }   
    }
    println!("{}",Str);
    Str.len()
}

fn slicing(Str:String , start:usize , end:usize) -> (String,usize){
    let strr = String::from(Str);
    let tot:usize= start + end;
    let res = &strr[start..end];
    (res.to_string(),tot)
}
